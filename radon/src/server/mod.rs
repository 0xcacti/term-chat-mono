pub mod error;

use axum::Router;
use clap::Parser;
use serde_derive::Deserialize;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub address: String,
    pub ws_enabled: Option<bool>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            address: "127.0.0.1:8080".to_string(),
            ws_enabled: None,
        }
    }
}

#[derive(Debug, Parser)]
pub struct RunArgs {
    /// The address to run the server on including port
    #[arg(long = "address", value_name = "address:port")]
    address: Option<String>,
    // Enable websocket server
    #[arg(long = "websocket", value_name = "ws_enabled")]
    ws: Option<bool>,
}

impl ServerConfig {
    pub fn merge_with_args(&mut self, args: &RunArgs) {
        if let Some(address) = &args.address {
            self.address = address.clone();
        }
        if let Some(ws_enabled) = &args.ws {
            self.ws_enabled = Some(*ws_enabled);
        }
    }
}

pub struct Server {
    config: ServerConfig,
    router: Router,
    state: Arc<AppState>,
}

pub struct AppState {
    user_set: Mutex<HashSet<String>>,
    tx: broadcast::Sender<String>,
}

impl AppState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(10);
        let user_set = Mutex::new(HashSet::new());
        Self { user_set, tx }
    }
}

impl Server {
    pub fn new(config: ServerConfig) -> Result<Self, error::ServerError> {
        let state = Arc::new(AppState::new());
        let router = Router::new();
        Ok(Self {
            config,
            router,
            state,
        })
    }

    pub async fn run(&mut self) -> Result<(), error::ServerError> {
        // self.router = self.router.route("/", get(api::index));
        println!("starting server on {:?}", self.config.address);
        Ok(())
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::message::MessageType;
//     use std::sync::atomic::{AtomicU16, Ordering};
//     use tokio::task::JoinHandle;
//
//     static NEXT_PORT: AtomicU16 = AtomicU16::new(8000);
//
//     fn get_server_address() -> String {
//         let port = NEXT_PORT.fetch_add(1, Ordering::SeqCst);
//         format!("127.0.0.1:{}", port)
//     }
//
//     fn get_test_chat_message() -> Vec<u8> {
//         let message = Message::new(MessageType::Chat, "hello".to_string());
//         message.encode()
//     }
//     async fn setup(server_address: &str) -> JoinHandle<()> {
//         let mut server = Server::new(server_address).await.unwrap();
//         let server_handle = tokio::spawn(async move {
//             server.run().await.unwrap();
//         });
//         server_handle
//     }
//
//     async fn read_and_validate_message(client: &mut TcpStream) {
//         let mut len_buf = [0u8; 4];
//         client.read_exact(&mut len_buf).await.unwrap();
//         let msg_len = u32::from_be_bytes(len_buf) as usize;
//
//         let mut msg_buf = vec![0u8; msg_len];
//         client.read_exact(&mut msg_buf).await.unwrap();
//         let received_message: Result<Message, _> = serde_json::from_slice(&msg_buf);
//         match received_message {
//             Ok(message) => {
//                 assert_eq!(message.message_type, MessageType::Chat);
//                 assert_eq!(message.payload, "hello");
//             }
//             Err(e) => {
//                 panic!("failed to parse message: {}", e);
//             }
//         }
//     }
//
//     #[tokio::test]
//     async fn test_client_can_connect() {
//         let server_address = get_server_address();
//         let server_handle = setup(&server_address).await;
//
//         let client = TcpStream::connect(server_address).await;
//         assert!(client.is_ok());
//
//         server_handle.abort();
//     }
//
//     #[tokio::test]
//     async fn test_client_can_send_message() {
//         let server_address = get_server_address();
//         let server_handle = setup(&server_address).await;
//         let mut client = TcpStream::connect(server_address).await.unwrap();
//         let message_buf = get_test_chat_message();
//         client.write_all(&message_buf).await.unwrap();
//         server_handle.abort();
//     }
//
//     #[tokio::test]
//     async fn test_client_can_receive_message() {
//         let server_address = get_server_address();
//         let server_handle = setup(&server_address).await;
//         let mut client_one = TcpStream::connect(&server_address).await.unwrap();
//         let mut client_two = TcpStream::connect(&server_address).await.unwrap();
//         let message_buf = get_test_chat_message();
//         client_one.write_all(&message_buf.clone()).await.unwrap();
//         read_and_validate_message(&mut client_two).await;
//         client_one.write_all(&message_buf.clone()).await.unwrap();
//         read_and_validate_message(&mut client_two).await;
//         client_one.write_all(&message_buf.clone()).await.unwrap();
//         read_and_validate_message(&mut client_two).await;
//         server_handle.abort();
//     }
// }
