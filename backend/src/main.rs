use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket, Message},
    routing::get,
    response::{IntoResponse, Response},
    Router,
};

use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Serialize)]
struct ChatMessage {
    content: String,
}

#[tokio::main]
async fn main() {
    let app : Router<(), > = Router::new().route("/ws", get(handler));

    let _ = axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(ws : WebSocketUpgrade) -> Response {
    println!("handler called");
    ws.on_upgrade(handle_socket)
}

//TODO: Recieve and Send messages!
async fn handle_socket(mut socket : WebSocket) {
    let test_msg = ChatMessage {
        content: String::from("test"),
    };
    
    for _i in 0..10 {
        socket.send(Message::Text(
            serde_json::to_string(&test_msg).unwrap()
            ))
            .await
            .unwrap();
    }

    println!("Test Message Recieved!");
}

