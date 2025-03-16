use clap::CommandFactory;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // The Short and long options for the name argument
    // are automatically generated from the field name

    // init doesn't require a value
    #[arg(short, long, default_value_t = false)]
    init: bool,

    #[arg(short, long, default_value_t = false)]
    add_folder: bool,

    #[arg(short, long, default_value_t = 10)]
    list_peers: u8,
}

fn init() {
    // This function will be called when the init flag is set
    println!("Initializing...");
}

fn add_folder() {
    // This function will be called when the add_folder flag is set
    println!("Adding folder...");
}

fn list_peers() {
    // This function will be called when the list_peers flag is set
    println!("Listing peers...");
}

fn main() {
    let args = Args::parse();
    println!("init: {}", args.init);
    println!("add_folder: {}", args.add_folder);
    println!("list_peers: {}", args.list_peers);

    // Check which flag is set and call the corresponding function
    if args.init {
        init();
    }
    if args.add_folder {
        add_folder();
    }
    if args.list_peers > 0 {
        list_peers();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_init_flag() {
        let args = Args::parse_from(&["test", "--init"]);
        assert!(args.init);
        assert!(!args.add_folder);
        assert_eq!(args.list_peers, 10);
    }
    #[test]
    fn test_add_folder_flag() {
        let args = Args::parse_from(&["test", "--add-folder"]);
        assert!(!args.init);
        assert!(args.add_folder);
        assert_eq!(args.list_peers, 10);
    }
    #[test]
    fn test_list_peers_flag() {
        let args = Args::parse_from(&["test", "--list-peers", "5"]);
        assert!(!args.init);
        assert!(!args.add_folder);
        assert_eq!(args.list_peers, 5);
    }
    #[test]
    fn test_default_list_peers() {
        let args = Args::parse_from(&["test"]);
        assert!(!args.init);
        assert!(!args.add_folder);
        assert_eq!(args.list_peers, 10);
    }
    #[test]
    fn verify_app() {
        Args::command().debug_assert();
    }
}
