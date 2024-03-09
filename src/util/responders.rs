use std::{fmt::Display, io::Cursor};

use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    Response,
};
use serde::Serialize;

//Creating multiple messages with different statuses to handle seperate types of responses

#[derive(Serialize, Debug)]
pub struct JsonStatus<T>
where
    T: Display,
{
    #[serde(skip_serializing)]
    pub status_code: Status,
    pub status: bool,
    pub message: T,
}

impl JsonStatus<T> {
    pub fn Created(item: &str) -> Self<&str> {
        let message = format!("Successfully created {item}");
        JsonStatus {
            status_code: Status::Ok,
            status: true,
            message: message
        }
    }
}

impl<'r, T> Responder<'r, 'static> for JsonStatus<T>
where
    T: Display + Serialize
{
    fn respond_to(self, _: &'r rocket::Request<'_>) -> response::Result<'static> {
        let mut build = Response::build();
        let string = serde_json::to_string(&self).map_err(|e| {
            error_!("JSON Failed to serialize: {:?}", e);
            Status::InternalServerError
        })?;
        build
            .header(ContentType::JSON)
            .sized_body(string.len(), Cursor::new(string))
            .status(self.status_code)
            .ok()
    }
}