use core::pin::Pin;
use std::{task::{Context, Poll}, collections::VecDeque};
use tokio::sync::mpsc;

use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket, Message},
    routing::get,
    response::{IntoResponse, Response},
    Router,
};
use futures_util::{sink::{Sink, SinkExt}, stream::{self, Stream, StreamExt, SplitSink, SplitStream}};
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Serialize)]
struct ChatMessage {
    content: String,
}

impl From<String> for ChatMessage {
    fn from(value: String) -> Self {
        ChatMessage { content: value }
    }
}

#[derive(Clone, PartialEq, Serialize)]
struct ChatStream {
    pub chat_messages : Box<VecDeque<String>>,
}

impl ChatStream {
    pub fn new() -> Self {
        ChatStream {
            chat_messages : Box::new(VecDeque::new()),
        }
    }

}

impl Unpin for ChatStream {

}

impl Stream for ChatStream {
    type Item = String;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        println!("{:?}", self.chat_messages.clone());
        let chat_stream = self.get_mut();
        match chat_stream.chat_messages.pop_front() {
            Some(x) => Poll::Ready(Some(x)),
            None => Poll::Pending,
        }
    }
}

impl Sink<String> for ChatStream {
    type Error = &'static str;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
         Poll::Ready(Ok(()))   
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, item: String) -> Result<(), Self::Error> {
        self.get_mut().chat_messages.push_back(item);
        Ok(())
    }

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
    let (mut socket_sender, mut socket_receiver) = socket.split();
    
    let (chat_message_sender, chat_message_receiver) = mpsc::channel::<String>(10); 
    tokio::spawn(write(socket_sender,  chat_message_receiver));
    tokio::spawn(read(socket_receiver, chat_message_sender));
}

async fn read(socket_receiver: SplitStream<WebSocket>, chat_message_sender: mpsc::Sender<String>) {
    let mut socket_receiver = socket_receiver;
    let mut chat_message_sender = chat_message_sender;
    while let Some(chat_msg) = socket_receiver.next().await {
        let msg = if let Ok(msg) = chat_msg {
            println!("{:?}", msg);
            msg
        }
        else {
            return
        };

        if let Message::Text(msg_text) = msg {
            chat_message_sender.send(msg_text)
                .await
                .unwrap();
        } 
    }
    
    println!("END READ");
}

async fn write(socket_sender: SplitSink<WebSocket, Message>, chat_message_receiver: mpsc::Receiver<String>) {
    let mut socket_sender = socket_sender;
    let mut chat_message_receiver = chat_message_receiver;

    let test_msg = ChatMessage {
        content: String::from("First Message"),
    };
    socket_sender.send(Message::Text(
        serde_json::to_string(&test_msg).unwrap()
    ))
    .await
    .unwrap();

    println!("FIRST MESSAGE RECIEVED");
    
    while let Some(chat_msg) = chat_message_receiver.recv().await {

        socket_sender.send(Message::Text(
            serde_json::to_string(&ChatMessage::from(chat_msg)).unwrap()
        ))
        .await
        .unwrap();
    }

    println!("END WRITE");

}


