use actix_web::{HttpResponse, Responder, Scope, web, Error};
use actix_web::web::Query;
use hyper::Method;
use serde::{Deserialize, Deserializer};
use serde::de::value::StrDeserializer;

use crate::calendar::Calendar;
use crate::calendar::event_type::EventType;
use crate::error;
use crate::https::Client;

pub fn service() -> Scope {
    Scope::new("/proxy")
        .route("", web::get().to(handler))
        .default_service(web::route().to(|| async {
            error::MethodNotAvailable::new(&[&Method::GET])
        }))
}

#[derive(Deserialize)]
pub struct QueryArgs {
    #[serde(rename = "pStud")]
    pub student_number: Option<String>,
    #[serde(rename = "pPers")]
    pub person_number: Option<String>,
    #[serde(rename = "pToken")]
    pub token: String,
    #[serde(default, deserialize_with = "deserialize_vec_from_csv")]
    pub include: Option<Vec<EventType>>,
    #[serde(default, deserialize_with = "deserialize_vec_from_csv")]
    pub exclude: Option<Vec<EventType>>,
    #[serde(default, deserialize_with = "deserialize_vec_from_csv")]
    pub ignore: Option<Vec<String>>,
}

fn deserialize_vec_from_csv<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>
{
    let mut s = if let Some(s) = Option::<String>::deserialize(deserializer)? {
        s
    } else {
        return Ok(None);
    };

    s = s.replace("\\,", "&COMMA;"); // Replace escaped commas with a placeholder

    let mut v = Vec::new();
    for item in s.split(',') {
        let item = item.replace("&COMMA;", ",");
        let de = StrDeserializer::new(item.as_str());
        let item = T::deserialize(de)?;
        v.push(item);
    }
    Ok(Some(v))
}

async fn handler(Query(query): Query<QueryArgs>, client: Client) -> impl Responder {
    let calendar = Calendar::from_query(query, client).await?;
    Ok::<HttpResponse, Error>(calendar.to_response())
}