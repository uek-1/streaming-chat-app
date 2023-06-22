use core::pin::Pin;
use std::task::{Context, Poll};


use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket, Message},
    routing::get,
    response::{IntoResponse, Response},
    Router,
};
use futures_util::{sink::{Sink, SinkExt}, stream::{self, Stream, StreamExt, SplitSink, SplitStream}, Future};
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Serialize)]
struct ChatMessage {
    content: String,
}

#[derive(Clone, PartialEq, Serialize)]
struct ChatStream {
}

impl Stream for ChatStream {
    type Item = String;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(None)
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

    let chat_stream = ChatStream{};
    let (mut chat_stream_sender, mut chat_stream_receiver) = chat_stream.split();

    tokio::spawn(write(socket_sender, chat_stream_receiver));
    tokio::spawn(read(socket_receiver, chat_stream_sender));
}

async fn read(socket_receiver: SplitStream<WebSocket>, chat_message_sender: SplitSink<ChatStream, String>) {
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
    
    println!("END WHILE");
}

async fn write(socket_sender: SplitSink<WebSocket, Message>, chat_message_receiver: SplitStream<ChatStream>) {
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
    
    while let Some(chat_msg) = chat_message_receiver.next().await {
        socket_sender.send(Message::Text(
            serde_json::to_string(&chat_msg).unwrap()
        ))
        .await
        .unwrap();
    }

}


