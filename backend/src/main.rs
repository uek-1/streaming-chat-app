use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket, Message},
    routing::get,
    response::{IntoResponse, Response},
    Router,
};
use futures_util::{sink::SinkExt, stream::{StreamExt, SplitSink, SplitStream}};
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

async fn handle_socket(mut socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();

    tokio::spawn(write(sender));
    tokio::spawn(read(receiver));
}

async fn read(receiver: SplitStream<WebSocket>) {
    let mut receiver = receiver;

    while let Some(chat_msg) = receiver.next().await {
        let msg = if let Ok(msg) = chat_msg {
            println!("{:?}", msg);
            msg
        }
        else {
            return
        };
    }
    
    println!("END WHILE");
}

async fn write(sender: SplitSink<WebSocket, Message>) {
    let mut sender = sender;
    let test_msg = ChatMessage {
        content: String::from("First Message"),
    };
    sender.send(Message::Text(
        serde_json::to_string(&test_msg).unwrap()
    ))
    .await
    .unwrap();

    println!("TEST MESSAGE RECIEVED");
}


