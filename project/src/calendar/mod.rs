use std::collections::HashSet;
use std::fmt::Write;

use actix_web::{Error, HttpResponse};
use actix_web::http::header;
use icalendar::{Calendar as iCalendar, Component, EventLike, Property};
use lazy_regex::regex;
use regex::Regex;
use tracing::info;

use crate::calendar::event_type::{EventType, Filter};
use crate::calendar::fetch::{fetch_calendar, Id};
use crate::calendar::utils::{from_event, from_property};
use crate::handlers::cal::QueryArgs;
use crate::https::Client;

pub mod event_type;
mod fetch;
mod utils;

pub struct Calendar {
    inner: iCalendar,
}

impl Calendar {
    pub async fn from_query(query: QueryArgs, client: Client) -> Result<Self, Error> {
        let id = Id::from_student_or_person_number(
            query.student_number,
            query.person_number,
        )?;

        match &id {
            Id::Student(id) => {
                info!(
                    student_id = id.as_str(),
                    "Got calendar request"
                );
            }
            Id::Person(id) => {
                info!(
                    person_id = id.as_str(),
                    "Got calendar request"
                );
            }
        }

        let calendar = fetch_calendar(client, id, query.token).await?;

        let filter = if let Some(include) = query.include {
            Filter::new_include(
                include.iter()
                    .map(|typ| *typ)
                    .collect::<HashSet<_>>()
            )
        } else if let Some(exclude) = query.exclude {
            Filter::new_exclude(
                exclude.iter()
                    .map(|typ| *typ)
                    .collect::<HashSet<_>>()
            )
        } else {
            Filter::new_none()
        };

        let ignored_events = query.ignore
            .unwrap_or_default()
            .into_iter()
            .collect::<HashSet<_>>();

        let mut result = iCalendar::new();
        {
            let mut prod_id_prop = result.properties.iter_mut()
                .filter(|property| property.key() == "PRODID");

            if let Some(prop) = prod_id_prop.next() {
                *prop = Property::new(
                    "PRODID",
                    "TUM-CalProxy/0.1",
                );
            } else {
                result.append_property(Property::new(
                    "PRODID",
                    "TUM-CalProxy/0.1",
                ));
            }
        }

        for property in calendar.properties {
            if property.name.as_str() == "PRODID" {
                continue;
            }
            if let Some(property) = from_property(property) {
                result.append_property(property);
            }
        }

        let mut already_parsed = HashSet::new();
        for event in calendar.events {
            let mut event = from_event(event);

            let summary = if let Some(summary) = event.get_summary() {
                summary.replace("\\", "")
            } else {
                continue;
            };

            let dedup_key = format!(
                "{}-{:?}",
                summary, event.get_start(),
            );
            if already_parsed.contains(&dedup_key) {
                continue;
            } else {
                already_parsed.insert(dedup_key);
            }

            let name_reg: &Regex = regex!(r#"(?x)
                (?<name> .*? )
                \s?
                (?: [ \(\[ ]
                    (?<id> [A-Z]{2,3}[0-9]+ (?:, \s? [A-Z]{2,3}[0-9]+)* )
                [ \)\] ] \s? )?
                (?<tag> [A-Z]{2} ),
                \s?
                (?<group> .* )
            "#);
            let captures = match name_reg.captures(&summary) {
                Some(captures) => captures,
                None => {
                    // Add the event anyway, but without any additional information or filtering
                    info!("Encountered event with unknown format: {}", summary);
                    result.push(event);
                    continue;
                }
            };

            let full_name = captures["name"].trim().to_string();
            let id = captures.name("id").map(|id| id.as_str());
            let typ = if let Ok(id) = captures["tag"].parse::<EventType>() {
                id
            } else {
                info!("Encountered unknown event type: {}", &captures["tag"]);
                result.push(event);
                continue;
            };
            let group = captures["group"].trim();

            if !filter.contains(typ) {
                continue;
            }
            if ignored_events.contains(&full_name) {
                continue;
            }
            if let Some(id) = id { if ignored_events.contains(id) {
                continue;
            } }

            let name = utils::replace_course_name(full_name.clone());
            event.summary(name.as_str());

            let room_reg: &Regex = regex!(r#"(?x)
                \(
                (?<building_id> \d{4} ) \.
                (?<floor> \d\d|EG|UG|DG|Z\d|U\d ) \.
                (?<room_id> [\dA-Z]+ )
                \)
            "#);

            let mut room = None;
            if let Some(loc) = event.get_location() {
                let loc = loc.replace("\\", "");

                if let Some(captures) = room_reg.captures(&loc) {
                    let building_id = &captures["building_id"];

                    if let Some(address) = utils::match_building_id(building_id) {
                        room = Some(loc);
                        event.location(address.replace(",", "\\,").as_str());
                    } else {
                        info!("Encountered unknown building ID: {}", building_id)
                    }
                } else if !loc.starts_with("Online") {
                    info!("Encountered location with unknown format: {}", loc);
                }
            }

            let mut description = String::new();
            write!(&mut description, "Name: {}\n", full_name).expect("Could not write to string");
            write!(&mut description, "Typ: {} ({})\n", typ, typ.id()).expect("Could not write to string");
            if let Some(id) = id {
                write!(&mut description, "ID: {}\n", id).expect("Could not write to string");
            }
            if let Some(room) = room {
                write!(&mut description, "Raum: {}\n", room).expect("Could not write to string");
            }
            write!(&mut description, "Gruppe: {}\n", group).expect("Could not write to string");
            write!(&mut description, "\n------------\n\n").expect("Could not write to string");
            write!(&mut description, "{}", summary).expect("Could not write to string");
            if let Some(desc) = event.get_description() {
                write!(&mut description, "\n\n{}", desc).expect("Could not write to string");
            }
            event.description(description.as_str());

            result.push(event);
        }

        Ok(Self {
            inner: result,
        })
    }

    pub fn to_response(self) -> HttpResponse {
        HttpResponse::Ok()
            .append_header((header::X_CONTENT_TYPE_OPTIONS, "nosniff"))
            .append_header((header::CONTENT_TYPE, "text/calendar;charset=utf-8"))
            .append_header((header::CONTENT_DISPOSITION, "attachment;filename=calendar.ics"))
            .append_header((header::CONTENT_LANGUAGE, "de"))
            .body(self.inner.to_string())
    }
}