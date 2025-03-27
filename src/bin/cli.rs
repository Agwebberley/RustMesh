use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use RustMesh::{Commands, serialize_command, deserialize_command};
use clap::{Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect to the local daemon on TCP port 12345.
    let mut stream = TcpStream::connect("127.0.0.1:12345").await?;

    let args = Args::parse();

    let command = match args.command {
        Commands::Status {} => Commands::Status {},
        Commands::ListPeers { limit } => Commands::ListPeers { limit },
        Commands::AddFolder { path } => Commands::AddFolder { path },
        Commands::RemoveFolder { path } => Commands::RemoveFolder { path },
        Commands::AddPeer { ip_addr } => Commands::AddPeer { ip_addr },
        Commands::SearchForPeers { timeout } => Commands::SearchForPeers { timeout },
        // Add additional command cases as needed...
    };

    let message = serialize_command(&command);

    stream.write_all(message.as_bytes()).await?;
    stream.flush().await?;

    // Prepare a buffer to hold the response.
    let mut buf = vec![0; 1024];

    // Read the response from the TCP connection.
    let n = stream.read(&mut buf).await?;
    if n == 0 {
        println!("No response received.");
    } else {
        let response = String::from_utf8_lossy(&buf[..n]);
        println!("Received response: {}", response);
    }

    Ok(())
}
