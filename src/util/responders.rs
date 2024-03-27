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

//Response types to the frontend
pub type ApiResult<T> = Result<T, Status>;
pub type Jsonstr<'a> = ApiResult<JsonStatus<&'a str>>;
pub type JsonString = ApiResult<JsonStatus<String>>;

impl JsonStatus<&str> {
    //Lifetimes have issues with Self return types?
    //Unsure why
    pub fn success(message: &str) -> JsonStatus<&str> {
        return JsonStatus {
            status_code: Status::Ok,
            status: true,
            message: message,
        };
    }

    pub fn failure(message: &str) -> JsonStatus<&str> {
        return JsonStatus {
            status_code: Status::InternalServerError,
            status: false,
            message: message,
        };
    }

    pub fn custom(code: Status, status: bool, message: &str) -> JsonStatus<&str> {
        return JsonStatus {
            status_code: code,
            status: status,
            message: message,
        };
    }
}

impl JsonStatus<String> {
    pub fn created(item: &str) -> Self {
        let message = format!("Successfully created {item}");
        return JsonStatus {
            status_code: Status::Ok,
            status: true,
            message: message,
        };
    }
}

impl<'r, T> Responder<'r, 'static> for JsonStatus<T>
where
    T: Display + Serialize,
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
