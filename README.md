# 🎮 CoolGamer

A fun collection of games built with Rust and [Bevy](https://bevyengine.org/)! This project currently features:


## 🎯 Blockshot

A fast-paced multiplayer shooter where two players duke it out in a grid-based arena. Features:

- Real-time multiplayer using GGRS rollback netcode
- Smooth movement and shooting mechanics
- Web browser support (WASM)
- Clean, minimalist visuals


## 🌟 Venture Time

A work-in-progress adventure game with:

- Clean, minimalist visuals
- Web browser support (WASM)
- More details coming soon!


## 🏓 Paddle

A classic pong-inspired game


## 🚀 Getting Started

### Prerequisites

- Rust and Cargo installed
- For web development:
  ```bash
  rustup target add wasm32-unknown-unknown
  cargo install wasm-server-runner
  cargo install matchbox_server
  ```

## 🛠️ Development

The project uses a workspace structure with multiple games/experiments. Each game is its own crate with independent dependencies.


### Development Commands

This project uses a Makefile to streamline development. View all available commands with:

```bash
make help
```

#### Running Games

```bash
# Run Blockshot natively
make blockshot.run

# Run Blockshot in browser
make blockshot.run.web

# Run Venture Time natively
make venture.run

# Run Venture Time in browser with WASM
cd venture_time && cargo run --target wasm32-unknown-unknown

# Run Paddle natively
make paddle.run

# Run Paddle in browser
make paddle.run.web

# Run the matchbox server (for multiplayer)
matchbox_server
```

#### Development Setup

Install all required dependencies:

```bash
make devenv
```

This will:
- Install the WASM target for Rust
- Install wasm-server-runner for browser testing
- Install cargo-watch for hot reloading
- Install matchbox_server for multiplayer

## 📜 License

Released under the Unlicense - see the [LICENSE](LICENSE) file for details. Feel free to use this code however you'd like!