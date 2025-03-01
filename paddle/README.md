# Paddle Game

A simple Pong-like game built with the Bevy game engine in Rust.

## Features

- Player vs Computer paddle gameplay
- Score tracking for both players
- Physics-based ball movement and collisions
- Keyboard input controls
- Clean, modular code structure

## Architecture

The game is built using Bevy's Entity Component System (ECS) architecture and is organized into several key modules:

- `components`: Core game entities like paddles, ball, and their associated components
- `input`: Input handling system using `leafwing_input_manager`
- `physics`: Collision detection and resolution
- `game`: Score tracking and game state management
- `scoreboard`: UI elements for displaying the score

## Systems

The game runs the following main systems each update:

- Ball movement and position projection
- Physics collision handling
- Input processing
- Score detection and updates
- Paddle movement (both player and AI-controlled opponent)
- UI updates

## Building and Running

To run the game:

1. Ensure you have Rust installed
2. Clone the repository
3. Run `cargo run` in the project directory

The game window will open and fit to the browser canvas, with score displays in the top corners.

## Controls

Use the keyboard to control your paddle (right side). The opponent paddle (left side) is controlled by simple AI.

## Local Development

This project uses a Makefile to streamline the development workflow. You can view all available commands by running:

```bash
make help
```

### Initial Setup

To install all required dependencies:
```bash
make devenv
```

This will:
- Install the WASM target for Rust
- Install the wasm-server-runner
- Install cargo-watch for hot reloading
- Install the matchbox server for multiplayer

### Running the Game

There are several ways to run the game:

- `make run` - Run the game (default)
- `make run.native` - Run natively
- `make run.web` - Run in browser using WASM
- `make watch.web` - Run in browser with hot reloading
- `make watch.native` - Run natively with hot reloading


