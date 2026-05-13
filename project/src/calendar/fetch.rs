use actix_web::Error;
use ical::parser::ical::component::IcalCalendar;
use ical::IcalParser;
use reqwest::Client;
use tracing::{error, warn};

use crate::error::{CalendarError, InternalServerError, QueryError};

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
    let url = format!(
        "https://campus.tum.de/tumonlinej/ws/termin/ical?{}&pToken={}",
        id.to_query_string(),
        token
    );

    let mut remaining_tries = 2;
    let response = loop {
        match client
            .get(&url)
            .header("User-Agent", "CalProxy/0.1")
            .send()
            .await
        {
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

    if !response.status().is_success() {
        return Err(CalendarError::new().into());
    }

    let bytes = match response.bytes().await {
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
