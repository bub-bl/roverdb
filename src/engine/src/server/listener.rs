use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tracing::error;
use crate::server::error::ListenerError;

pub struct Listener {
    listener: TcpListener,
    last_received_message: [u8; 1024],
}

impl Listener {
    pub async fn new(address: &str) -> Result<Listener, ListenerError> {
        let listener = TcpListener::bind(address).await;

        if let Ok(listener) = listener {
            Ok(Listener {
                listener,
                last_received_message: [0; 1024]
            })
        } else {
            Err(ListenerError::BindFailed)
        }
    }

    pub async fn listen(self) {
        while let Ok((mut socket, _)) = self.listener.accept().await {
            tokio::spawn(async move {
                Self::handle_client(&mut socket).await;
            });
        }
    }

    async fn handle_client(client: &mut TcpStream) {
        println!("Enculer");

        let mut buffer = [0u8; 1024];
        let response = client.read(&mut buffer).await.map_err(|_| ListenerError::BindFailed);

        match response {
            Ok(data) => {
                let response = &buffer[..data];
                println!("Données reçues du client : {:?}", response);
            },
            Err(e) => {
                error!("Error: {}", e)
            }
        };

        // println!("Données reçues du client : {:?}", &buffer[..data]);
    }
}