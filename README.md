# RustMesh - A Lightweight Peer-to-Peer File Synchronization Tool

## Disclaimer
README Written by ChatGPT.  These are not completed features but rather a vision/roadmap of what will be.  This project is solely for educational purposes and intends to help me learn Rust, not to make a production-ready product.

## 🚀 Overview
RustMesh is a decentralized, peer-to-peer (P2P) file synchronization tool built in **Rust**. It allows users to share and synchronize files across multiple devices **without relying on a central server**. RustMesh leverages modern Rust async programming, cryptographic security, and efficient networking protocols to ensure fast and secure file transfers.

## 🔥 Features (Planned)
✅ Decentralized peer discovery (mDNS or DHT)  
✅ Secure, encrypted connections (Noise Protocol or TLS)  
✅ Real-time file synchronization  
✅ Chunk-based file transfer for large files  
✅ File versioning and conflict resolution  
✅ CLI interface for easy control  
✅ (Optional) Web UI for managing peers and sync status  

## 🏗️ Project Goals
1. **Learn Rust Fundamentals**: Ownership, error handling, concurrency, and async programming.
2. **Understand P2P Networking**: Implement secure peer discovery and communication.
3. **Build an Efficient File Sync System**: Optimize data transfer with chunking and zero-copy techniques.
4. **Ensure Security**: Encrypt file transfers and authenticate peers.
5. **Make It Fast & Lightweight**: Use Rust’s performance optimizations to handle large-scale file sync.

## 🛠️ Technologies Used
- **Rust Async Runtime**: `tokio` or `async-std`
- **Networking**: `quinn` (QUIC), `libp2p`, `mio`
- **File Monitoring**: `notify`
- **Security**: `rustls`, `snow` (Noise Protocol)
- **Serialization**: `serde`, `bincode`
- **CLI Interface**: `clap`
- **(Optional) Web UI**: `warp`, `axum`

## 📌 How It Works
1. **Peer Discovery**: Devices on the same network find each other using mDNS/DHT.
2. **Secure Handshake**: Peers authenticate and exchange encryption keys.
3. **File Watching**: The tool detects changes in a synced directory.
4. **Chunk-Based Transfer**: Files are split into small chunks for efficient transfer.
5. **Version Control**: Prevents sync conflicts using hashes and timestamps.

## 📅 Development Roadmap
### **Milestone 1**: Set up a basic CLI
- Implement `init`, `add-folder`, and `list-peers` commands.

### **Milestone 2**: Peer Discovery
- Enable peers to find each other on a local network via mDNS.

### **Milestone 3**: Secure Peer Communication
- Implement encrypted messaging between peers.

### **Milestone 4**: File Monitoring
- Watch a directory for file changes and log them.

### **Milestone 5**: Chunk-Based File Transfer
- Transfer files efficiently using chunking and resumable uploads.

### **Milestone 6**: Version Control & Conflict Resolution
- Store multiple file versions and handle conflicts.

### **Milestone 7**: Performance & UX Improvements
- Optimize networking, add progress bars, and refine error handling.

## 🚀 Getting Started
### **Prerequisites**
- Install Rust:  
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- Clone this repository:  
  ```sh
  git clone https://github.com/yourusername/rustmesh.git
  cd rustmesh
  ```

### **Running the Project**
1. Build the project:
   ```sh
   cargo build --release
   ```
2. Start RustMesh:
   ```sh
   cargo run -- sync start
   ```

### **Example Usage**
```sh
# Initialize a sync folder
rustmesh sync init ~/Documents/shared

# Add a peer by IP
rustmesh peer add 192.168.1.100

# Start file synchronization
rustmesh sync start
```

## 💡 Future Enhancements
- 🔗 NAT Traversal for wider connectivity
- 🌍 Fully distributed file sync using `libp2p`
- 📱 Web UI for managing peers and file sync

## 🤝 Contributing
If you're interested in contributing, feel free to open issues or submit pull requests!

## 📜 License
This project is licensed under the **MIT License**.

---
Built with ❤️ in Rust
