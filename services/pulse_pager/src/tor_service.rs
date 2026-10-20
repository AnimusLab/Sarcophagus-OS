use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use anyhow::Result;
use crate::ratchet::{PagerMessagePacket, FIXED_PACKET_SIZE};

pub struct TorMailboxServer {
    onion_port: u16,
    shared_secret: [u8; 32],
}

impl TorMailboxServer {
    pub fn new(onion_port: u16, shared_secret: [u8; 32]) -> Self {
        Self { onion_port, shared_secret }
    }

    /// Starts an asynchronous Tor hidden service receiver for incoming encrypted mail packets
    pub async fn start_mailbox_listener(&self) -> Result<()> {
        let bind_addr = format!("127.0.0.1:{}", self.onion_port);
        let listener = TcpListener::bind(&bind_addr).await?;
        println!("[*] Tor V3 Asynchronous Mailbox bound to local proxy endpoint: {}", bind_addr);

        let secret = self.shared_secret;

        loop {
            let (mut socket, _peer_addr) = listener.accept().await?;
            let secret_clone = secret;
            tokio::spawn(async move {
                let mut buffer = vec![0u8; FIXED_PACKET_SIZE];
                match socket.read_exact(&mut buffer).await {
                    Ok(_) => {
                        match PagerMessagePacket::decrypt_and_unpad(&secret_clone, &buffer) {
                            Ok(plaintext) => {
                                let msg = String::from_utf8_lossy(&plaintext);
                                println!("[+] DECRYPTED INBOUND PAGER PACKET: {}", msg);
                                let _ = socket.write_all(b"ACK_STORED").await;
                            }
                            Err(e) => {
                                eprintln!("[-] Failed to decrypt incoming container block: {}", e);
                                let _ = socket.write_all(b"ERR_INVALID_DECRYPTION").await;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("[-] Stream read error: {}", e);
                    }
                }
            });
        }
    }
}

pub struct TorV3Manager {
    control_port: u16,
    local_service_port: u16,
}

impl TorV3Manager {
    pub fn new(control_port: u16, local_service_port: u16) -> Self {
        Self {
            control_port,
            local_service_port,
        }
    }

    /// Spawns an ephemeral V3 Hidden Service dynamically via Tor Control Port
    pub async fn provision_ephemeral_onion(&self) -> Result<String> {
        let control_addr = format!("127.0.0.1:{}", self.control_port);
        let mut stream = TcpStream::connect(&control_addr).await?;

        // 1. Authenticate with Tor Control Port (Cookie or SafeCookie auth)
        // For local Sarcophagus OS builds, we use cookie auth located at /var/run/tor/control.authcookie
        stream.write_all(b"AUTHENTICATE\r\n").await?;
        
        let mut buf = [0u8; 128];
        let n = stream.read(&mut buf).await?;
        let response = String::from_utf8_lossy(&buf[..n]);
        
        if !response.starts_with("250") {
            anyhow::bail!("Tor control port authentication failed: {}", response);
        }

        // 2. Request an Ephemeral V3 Hidden Service pointing to our local mailbox port
        // Syntax: ADD_HS NEW:ED25519-V3 80:<local_port>
        let command = format!("ADD_HS NEW:ED25519-V3 80:{}\r\n", self.local_service_port);
        stream.write_all(command.as_bytes()).await?;

        let n = stream.read(&mut buf).await?;
        let hs_response = String::from_utf8_lossy(&buf[..n]);

        if !hs_response.starts_with("250") {
            anyhow::bail!("Failed to create ephemeral hidden service: {}", hs_response);
        }

        // Parse out the .onion address from the response string
        // Response format example: 250-ID=... \n 250-ServiceID=abcdefg... \n 250 OK
        let onion_address = hs_response
            .lines()
            .find(|line| line.contains("ServiceID="))
            .and_then(|line| line.split("ServiceID=").nth(1))
            .map(|s| format!("{}.onion", s.trim()))
            .ok_or_else(|| anyhow::anyhow!("Could not parse .onion address from Tor response"))?;

        println!("[+] Ephemeral Tor V3 Hidden Service active: {}", onion_address);
        Ok(onion_address)
    }
}