use core::pin::Pin;
use std::{task::{Context, Poll}, collections::VecDeque, sync::{Arc, Mutex}};
use tokio::sync::broadcast;

use axum::{
    extract::{ws::{WebSocketUpgrade, WebSocket, Message}, State},
    routing::get,
    response::{IntoResponse, Response},
    Router,
};
use futures_util::{sink::{Sink, SinkExt}, stream::{self, Stream, StreamExt, SplitSink, SplitStream}};
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
struct ChatMessage {
    time : String,
    username : String,
    message: String,
}

impl From<String> for ChatMessage {
    fn from(value: String) -> Self {
        ChatMessage { message: value , time: "".to_string(), username : "".to_string()}
    }
}

impl From<Value> for ChatMessage {
    fn from(value: Value) -> Self {
        ChatMessage {
            time: value["time"].to_string(),
            username : value["username"].to_string(),
            message : value["message"].to_string(),
        }
    }
}

#[derive(Clone)]
struct ChatStreamState {
    sender : broadcast::Sender<ChatMessage>,
    receiver : Arc<Mutex<broadcast::Receiver<ChatMessage>>>
}



#[tokio::main]
async fn main() {
    let (chat_message_sender, chat_message_receiver) = broadcast::channel::<ChatMessage>(10); 
    
    let state = ChatStreamState {
        sender : chat_message_sender,
        receiver : Arc::new(Mutex::new(chat_message_receiver))
    };

    let app : Router<(), > = Router::new()
        .route("/ws", get(handler))
        .with_state(state);

    let _ = axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(ws : WebSocketUpgrade, State(state) : State<ChatStreamState>) -> Response {
    println!("handler called");
    let chat_sender = state.sender;
    let chat_receiver = chat_sender.subscribe();
    ws.on_upgrade(|socket| handle_socket(socket, chat_sender, chat_receiver))
}

async fn handle_socket(mut socket: WebSocket, chat_sender: broadcast::Sender<ChatMessage>, chat_receiver: broadcast::Receiver<ChatMessage>) {
    let (mut socket_sender, mut socket_receiver) = socket.split();
 
    tokio::spawn(write(socket_sender, chat_receiver));
    tokio::spawn(read(socket_receiver, chat_sender));
}

async fn read(socket_receiver: SplitStream<WebSocket>, chat_message_sender: broadcast::Sender<ChatMessage>) {
    let mut socket_receiver = socket_receiver;
    let chat_message_sender = chat_message_sender;
    while let Some(chat_msg) = socket_receiver.next().await {
        let msg = if let Ok(msg) = chat_msg {
            println!("{:?}", msg);
            msg
        }
        else {
            return
        };

        if let Message::Text(msg_text) = msg {
            let value : Value = serde_json::from_str(&msg_text).unwrap();
            chat_message_sender.send(ChatMessage::from(value))
                .unwrap();
        } 
    }
    
    println!("END READ");
}

async fn write(socket_sender: SplitSink<WebSocket, Message>, chat_message_receiver: broadcast::Receiver<ChatMessage>) {
    let mut socket_sender = socket_sender;
    let mut chat_message_receiver = chat_message_receiver;

    let test_msg = ChatMessage::from("First Message".to_string());
    socket_sender.send(Message::Text(
        serde_json::to_string(&test_msg).unwrap()
    ))
    .await
    .unwrap();

    println!("FIRST MESSAGE RECIEVED");
    
    while let Ok(chat_msg) = chat_message_receiver.recv().await {

        socket_sender.send(Message::Text(
            serde_json::to_string(&chat_msg).unwrap()
        ))
        .await
        .unwrap();
    }

    println!("END WRITE");

}

