use miniserve::{http, Content, Request, Response};
use serde::{Deserialize, Serialize};

// Define the data structure for incoming JSON
#[derive(Deserialize)]
struct ChatRequest {
    messages: Vec<String>,
}

// Define the data structure for outgoing JSON
#[derive(Serialize)]
struct ChatResponse {
    messages: Vec<String>,
}

fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

fn chat(req: Request) -> Response {
    match req {
        Request::Get => Err(http::StatusCode::METHOD_NOT_ALLOWED),
        Request::Post(body) => {
            if let Ok(chat_request) = serde_json::from_str::<ChatRequest>(&body) {
                let mut messages = chat_request.messages;
                messages.push(String::from("And how does that make you feel?"));

                let response_body = serde_json::to_string(&ChatResponse { messages }).unwrap();
                Ok(Content::Json(response_body))
            } else {
                Err(http::StatusCode::BAD_REQUEST)
            }
        }
    }
}

fn main() {
    miniserve::Server::new()
        .route("/", index)
        .route("/chat", chat)
        .run()
}
