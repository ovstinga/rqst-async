use chatbot::{gen_random_number, query_chat};
use miniserve::{http::StatusCode, Content, Request, Response};
use serde::{Deserialize, Serialize};
use tokio::join;

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

async fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

async fn chat(req: Request) -> Response {
    match req {
        Request::Get => Err(StatusCode::METHOD_NOT_ALLOWED),
        Request::Post(body) => {
            if let Ok(chat_request) = serde_json::from_str::<ChatRequest>(&body) {
                let mut messages = chat_request.messages;
                let (r, mut responses) = join!(gen_random_number(), query_chat(&messages));

                let response = responses.remove(r % responses.len());
                messages.push(response);

                let response_body = serde_json::to_string(&ChatResponse { messages }).unwrap();
                Ok(Content::Json(response_body))
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    miniserve::Server::new()
        .route("/", index)
        .route("/chat", chat)
        .run()
        .await
}
