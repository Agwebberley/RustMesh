use serde::{Serialize, Deserialize};
use clap::Subcommand;

#[derive(Serialize, Deserialize, Subcommand, Debug)]
pub enum Commands {
    Status {},
    ListPeers {
        // LEARNING: Option<> Denotes an optional field
        limit: Option<u8>
    },
    AddFolder {
        path: String,
    },
    RemoveFolder {
        path: String,
    },
    AddPeer {
        ip_addr: String,
    },
    SearchForPeers {
        timeout: Option<u32>,
    },
}

// When sending a command, you can serialize it to JSON:
pub fn serialize_command(cmd: &Commands) -> String {
    serde_json::to_string(cmd).expect("Failed to serialize command")
}

// And to deserialize:
pub fn deserialize_command(data: &str) -> Commands {
    serde_json::from_str(data).expect("Failed to deserialize command")
}