use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    // The Short and long options for the name argument
    // are automatically generated from the field name

    #[command(subcommand)]
    pub(crate) command: Commands,

    // LEARNING: rather than use bool values to figure out what command is
    // being called, use a subcommand, this checks against an enum.
    // init doesn't require a value
    // #[arg(short, long, default_value_t = false)]
    // pub(crate) init: bool,

    // #[arg(short, long, default_value_t = false)]
    // pub(crate) add_folder: bool,

    // #[arg(short, long, default_value_t = 10)]
    // pub(crate) list_peers: u8,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    Init {

    },
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