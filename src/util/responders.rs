use rocket::serde::json::Json;
use serde::Serialize;

//Creating multiple messages with different statuses to handle seperate types of responses

#[derive(Serialize, Debug)]
pub struct JsonMessage {
    pub status: bool,
    pub message: String
}

#[derive(Responder, Debug)]
pub struct ServerMessage {
    pub message: Json<JsonMessage>
}

impl ServerMessage {
    pub fn new(message: JsonMessage) -> Self {
        Self {
            message: Json::from(message)
        }
    }
}

#[derive(Debug, Responder)]
pub enum RequestResponse {
    #[response(status = 200, content_type = "json")]
    OKRequest(ServerMessage),

    #[response(status = 400, content_type = "json")]
    BadRequest(ServerMessage),

    #[response(status = 500, content_type = "json")]
    InternalErrorRequest(ServerMessage),

    #[response(status = 501, content_type = "json")]
    NotImplementedRequest(ServerMessage)
}