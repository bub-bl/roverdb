use std::io::{BufRead, Read};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Runtime;
use rover_core::server::ConnectionInfo;
use crate::server::constants::{DEFAULT_MAX_CONNECTIONS, DEFAULT_MAX_THREADS, DEFAULT_THREAD_STACK_SIZE};

mod error;
mod listener;
pub mod constants;

#[derive(Debug, Clone)]
pub enum ServerStatus {
    Initializing,
    Running,
    Stopped,
}

pub struct Server {
    id: u16,
    name: String,
    status: ServerStatus,
    max_connections: u16,
    max_threads: usize,
    connection_info: ConnectionInfo,
    runtime: Runtime,
}

impl Server {
    pub fn new() -> Self {
        let name = &"rover-01";
        let conn_info = ConnectionInfo::default();

        Self {
            id: 0,
            name: name.to_string(),
            status: ServerStatus::Initializing,
            max_connections: DEFAULT_MAX_CONNECTIONS,
            max_threads: DEFAULT_MAX_THREADS,
            connection_info: conn_info.clone(),
            runtime: tokio::runtime::Builder::new_multi_thread()
                .worker_threads(DEFAULT_MAX_THREADS)
                .thread_name(name.to_string())
                .thread_stack_size(DEFAULT_THREAD_STACK_SIZE)
                .enable_all()
                .build().unwrap(),
        }
    }

    async fn handle_client(socket: &mut TcpStream) {
        let mut buffer = [0; 1024];
        let bytes_read = socket.read(&mut buffer).await.unwrap();
        let received_data = &buffer[..bytes_read];

        println!("Data Received: {:?}", received_data);
    }

    pub fn start(&mut self) {
        let conn_info = &self.connection_info;
        let addr = conn_info.socket_addr;

        self.runtime.block_on(async {
            let listener = TcpListener::bind(&addr).await.unwrap();
            println!("Listening on {}", &addr);

            while let Ok((mut socket, _)) = listener.accept().await {
                tokio::spawn(async move {
                    Self::handle_client(&mut socket).await;
                });
            }
        });
    }

    pub fn stop(&self) {
        todo!()
    }

    pub fn restart(&self) {
        todo!()
    }
}
