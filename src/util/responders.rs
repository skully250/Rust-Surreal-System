use std::io::Cursor;

use rocket::{
    http::{ContentType, Status},
    response::{Responder, self},
    Response,
};
use serde::Serialize;

//Creating multiple messages with different statuses to handle seperate types of responses

#[derive(Serialize, Debug)]
pub struct JsonMessage<'a> {
    #[serde(skip_serializing)]
    pub status_code: Status,
    pub status: bool,
    pub message: &'a str,
}

impl<'a, 'r, 'o: 'r> Responder<'r, 'o> for JsonMessage<'a> {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> response::Result<'o> {
        let mut build = Response::build();
        let string = serde_json::to_string(&self).map_err(|e| {
            error_!("JSON Failed to serialize{:?}", e);
            Status::InternalServerError
        })?;
        build
            .header(ContentType::JSON)
            .sized_body(string.len(), Cursor::new(string))
            .status(self.status_code)
            .ok()
    }
}
