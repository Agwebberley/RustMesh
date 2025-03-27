use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use RustMesh::{Commands, deserialize_command};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Bind to 0.0.0.0:12345 to listen on all interfaces on port 12345.
    let addr = "0.0.0.0:12345";
    let listener = TcpListener::bind(addr).await?;
    println!("RustMesh daemon listening on {}", addr);

    loop {
        // Accept an incoming TCP connection.
        let (mut stream, peer_addr) = listener.accept().await?;
        println!("New connection from: {}", peer_addr);

        // Create a buffer and read from the stream.
        let mut buf = vec![0; 1024];
        let n = stream.read(&mut buf).await?;
        let message = String::from_utf8_lossy(&buf[..n]);
        println!("Raw message: {:?}", message);

        // Deserialize the message into a Commands enum.
        let command: Commands = deserialize_command(&message);
        println!("Received command: {:?}", command);

        // Process the command and create a response.
        let response = format!("Command {:?} processed successfully", command);

        // Write the response back to the client.
        stream.write_all(response.as_bytes()).await?;
        stream.flush().await?;
    }
}
