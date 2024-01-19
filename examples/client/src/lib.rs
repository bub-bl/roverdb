use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use rover_core::server::ConnectionInfo;

pub enum SocketError {
    ConnectionError,
    SendError,
    ReceiveError(),
}

pub type SocketResponse = Result<[u8], SocketError>;

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    Connected,
    Disconnected,
}

#[derive(Debug)]
pub struct Client {
    socket: TcpStream,
    connection_state: ConnectionState,
    connection_info: ConnectionInfo,
    last_sent_message: Option<TcpStream>,
    last_received_message: Option<TcpStream>,
}

impl Client {
    pub async fn connect(connection_info: ConnectionInfo) -> Self {
        let socket = TcpStream::connect(connection_info.get_addr()).await;
        let mut conn_state: ConnectionState;

        match socket {
            Ok(_) => conn_state = ConnectionState::Connected,
            Err(_) => conn_state = ConnectionState::Disconnected,
        }

        Self {
            socket: TcpStream::connect(connection_info.get_addr()).await.unwrap(),
            connection_state: conn_state,
            connection_info,
            last_sent_message: None,
            last_received_message: None,
        }
    }

    pub async fn send_data(mut self, data: String) {
        let data_bytes = data.as_bytes();
        self.socket.write_all(data_bytes).await.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use rover_core::server::ConnectionInfo;
    use crate::Client;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_connection() {
        env_logger::init();

        let conn_info = ConnectionInfo::new("127.0.0.1".to_string(), 5630);
        let client = Client::connect(conn_info).await;

        println!("Sending..");
        client.send_data("Test message".to_string()).await;
        println!("Received!");
    }
}
