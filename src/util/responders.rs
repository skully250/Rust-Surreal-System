use serde::Serialize;

//Creating multiple messages with different statuses to handle seperate types of responses
pub trait ResponseMessage {}

#[derive(Serialize, Responder)]
#[response(status = 200, content_type = "json")]
pub struct ServerMessage<'a> {
    pub message: &'a str
}

#[derive(Serialize, Responder)]
#[response(status = 400, content_type = "json")]
pub struct ErrorMessage<'a> {
    pub message: &'a str
}

impl ResponseMessage for ServerMessage<'_> {}
impl ResponseMessage for ErrorMessage<'_> {}