use axum::extract::ws::{Message, WebSocket};
use prost::Message as GrpcMessage;
use protocol::flwr::{flower_service_client::FlowerServiceClient, ClientMessage};

use std::net::SocketAddr;
use tokio::sync::mpsc;

use futures::{sink::SinkExt, stream::StreamExt};

pub async fn handle_socket(socket: WebSocket, who: SocketAddr) {
    
    let (mut ws_sender, mut ws_receiver) = socket.split();
    let mut server_connection = FlowerServiceClient::connect("http//[::1]:5051").await.unwrap();

    //TODO: First the client must send a AUTH message 

    // Convert and stream client messages 
    //
    // TODO: Remove unnecessary deserialisation, might need to switch to grpcio
    let (tx, rx) = mpsc::channel::<Vec<u8>>(32);
    let client_grpc_messages = tokio_stream::wrappers::ReceiverStream::new(rx).map(|d| ClientMessage::decode(d.as_slice()).unwrap());
    let mut server_grpc_messages = server_connection.join(client_grpc_messages).await.unwrap().into_inner();

    let mut send_task = tokio::spawn(async move {
        // Read client messages and send to server
        let mut count: u64 = 0;
        while let Some(Ok(message)) = ws_receiver.next().await {
            if let Message::Binary(data) = message {
                // rx is wired into the out stream to the server
                if tx.send(data).await.is_err() {
                    break;
                }
                count += 1;
            }
        }
        count
    });

    let mut recv_task = tokio::spawn(async move {
        // Send server messages back to client
        let mut count: u64 = 0;
        while let Some(Ok(msg)) = server_grpc_messages.next().await {
            if ws_sender.send(Message::Binary(msg.encode_to_vec())).await.is_err() {
                break
            };
            count += 1;
        }
        count
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(count) => println!("{count} messages sent"),
                Err(a) => println!("Error sending messages {a:?}")
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(count) => println!("{count} messages recieved"),
                Err(b) => println!("Error receiving messages {b:?}")
            }
            send_task.abort();
        }
    }

    // returning from the handler closes the websocket connection
    println!("Websocket context {who} destroyed");
}
