use actix_web::Error;
use hyper::{body, Request};
use ical::parser::ical::component::IcalCalendar;
use ical::IcalParser;
use tracing::{error, warn};

use crate::error::{CalendarError, InternalServerError, QueryError};
use crate::https::Client;

pub enum Id {
    Student(String),
    Person(String),
}

impl Id {
    pub fn from_student_or_person_number(
        student_number: Option<String>,
        person_number: Option<String>,
    ) -> Result<Self, Error> {
        if let Some(student_number) = student_number {
            Ok(Self::Student(student_number))
        } else if let Some(person_number) = person_number {
            Ok(Self::Person(person_number))
        } else {
            Err(QueryError::new("pStud or pPers".to_string()).into())
        }
    }

    fn to_query_string(&self) -> String {
        match self {
            Self::Student(student_number) => format!("pStud={}", student_number),
            Self::Person(person_number) => format!("pPers={}", person_number),
        }
    }
}

pub async fn fetch_calendar(client: Client, id: Id, token: String) -> Result<IcalCalendar, Error> {
    let make_req_fn = || {
        let request = Request::builder()
            .method("GET")
            .uri({
                let uri = "https://campus.tum.de/tumonlinej/ws/termin/ical?";
                format!("{}{}&pToken={}", uri, id.to_query_string(), token)
            })
            .header("User-Agent", "CalProxy/0.1")
            .body(hyper::Body::empty());

        match request {
            Ok(request) => Ok(request),
            Err(e) => {
                error!("Error building request: {}", e);
                Err(InternalServerError::new())
            }
        }
    };

    let mut remaining_tries = 2;
    let response = loop {
        match client.request(make_req_fn()?).await {
            Ok(response) => break response,
            Err(e) => {
                remaining_tries -= 1;

                if remaining_tries == 0 {
                    error!("Error fetching calendar: {}", e);
                    return Err(InternalServerError::new().into());
                } else {
                    warn!("Error fetching calendar: {}, retrying", e);
                }
            }
        }
    };

    let (parts, body) = response.into_parts();

    if !parts.status.is_success() {
        return Err(CalendarError::new().into());
    }

    let bytes = match body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Error reading response body: {}", e);
            return Err(InternalServerError::new().into());
        }
    };

    let mut calendar = IcalParser::new(bytes.as_ref());
    let cal = match calendar.next() {
        Some(Ok(cal)) => cal,
        Some(Err(e)) => {
            error!("Error parsing calendar: {}", e);
            return Err(InternalServerError::new().into());
        }
        None => {
            warn!("TUMOnline returned an empty calendar");
            return Err(InternalServerError::new().into());
        }
    };

    Ok(cal)
}
