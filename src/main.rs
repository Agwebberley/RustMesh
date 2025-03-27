use clap::Parser;
use crate::cli_commands::Commands;
mod cli_commands;
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    let args = cli_commands::Args::parse();

    match args.command {
        Commands::Init { .. } => {}
        Commands::ListPeers { limit } => list_peers(limit),
        Commands::AddFolder { .. } => {}
        Commands::RemoveFolder { .. } => {}
        Commands::AddPeer { .. } => {}
        Commands::SearchForPeers { timeout } => search_for_peers(timeout),
    }
}

pub fn list_peers(limit: Option<u8>) {
    println!("Listing peers...");
    println!("Limit: {:?}", limit);
}

fn search_for_peers(timeout: Option<u32>) {
    println!("Searching for peers...");
    // Given that peer "discovery" doesn't seem possible.
    // My new solution is that each time a peer is connected to they exchange
    // peer lists.  Because of that this function will just poll all peers on the list.

}

fn connect_to_peer(ip_addr: String, port: Option<u16>) {

    let port = port.unwrap_or_else(|| 8080);

    println!("Connecting to {}:{}", ip_addr, port);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_command() {
        use clap::CommandFactory;
        cli_commands::Args::command().debug_assert();
    }
}