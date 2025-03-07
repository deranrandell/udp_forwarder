use std::net::SocketAddr;
use tokio::io;
use tokio::net::UdpSocket;
use log::{info, debug};

#[derive(Debug)]
pub struct UdpForwarder {
    source_addr: SocketAddr,
    destination_addr: SocketAddr,
}

impl UdpForwarder {
    pub fn new(source_addr: SocketAddr, destination_addr: SocketAddr) -> Self {
        UdpForwarder {
            source_addr,
            destination_addr,
        }
    }

    pub async fn run(&self) -> io::Result<()> {
        // Bind the UDP socket to the source address
        let socket = UdpSocket::bind(self.source_addr).await?;
        info!("Listening on {}", self.source_addr);
        info!("Forwarding to {}", self.destination_addr);

        // A buffer to store incoming data
        let mut buf = vec![0; 65507];

        loop {
            // Receive data from the source
            let (len, _src) = socket.recv_from(&mut buf).await?;
            debug!("Received {} bytes", len);

            // Forward the data to the destination address
            socket.send_to(&buf[..len], self.destination_addr).await?;
            debug!("Forwarded {} bytes to {}", len, self.destination_addr);
        }
    }
}
