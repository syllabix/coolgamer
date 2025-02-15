# 🎮 CoolGamer

A fun collection of games built with Rust and [Bevy](https://bevyengine.org/)! This project currently features:


## 🎯 Blockshot

A fast-paced multiplayer shooter where two players duke it out in a grid-based arena. Features:

- Real-time multiplayer using GGRS rollback netcode
- Smooth movement and shooting mechanics
- Web browser support (WASM)
- Clean, minimalist visuals


## 🌟 Venture Time
*Coming soon!* - An exciting new game experiment in the works.


## 🚀 Getting Started

### Prerequisites

- Rust and Cargo installed
- For web development:
  ```bash
  rustup target add wasm32-unknown-unknown
  cargo install wasm-server-runner
  cargo install matchbox_server
  ```


### Quick Start

1. Clone this repository

2. Choose your adventure:
   ```bash
   # Run Blockshot in browser
   cd blockshot
   cargo run --target wasm32-unknown-unknown

   # Run the matchbox server (for multiplayer)
   matchbox_server
   ```

## 🛠️ Development

The project uses a workspace structure with multiple games/experiments. Each game is its own crate with independent dependencies.


### Development Commands
```bash
# Watch mode for rapid development
cargo watch -cx "run --target wasm32-unknown-unknown"

# Run tests
cargo test
```


## 🤝 Contributing

This is primarily a learning project, but contributions are welcome!


## 📜 License

Released under the Unlicense - see the [LICENSE](LICENSE) file for details. Feel free to use this code however you'd like!