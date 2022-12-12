use rocket::serde::json::Json;

//Creating multiple messages with different statuses to handle seperate types of responses

#[derive(Responder, Debug)]
pub struct ServerMessage {
    pub message: Json<String>
}

impl ServerMessage {
    pub fn new(message: String) -> Self {
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