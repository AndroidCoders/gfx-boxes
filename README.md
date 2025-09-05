File version: 1.01

**TLDR:**
* This project is a Rust application for creating graphical effects.
* It displays multiple animated white boxes on a fullscreen background.
* The application can be run with or without a debug feature to save PNG screenshots.

A project for learning the the Rust programming language by creating graphical effects. This application displays multiple white boxes that animate across a black fullscreen background.

## How to Run

This project uses Cargo, the Rust package manager.

To build and run the application:

```bash
cargo run
```

## Debug Features

### Video Debug Output

To enable saving PNG screenshots for debugging, run the application with the `debug-video-to-png` feature flag:

```bash
cargo run --features debug-video-to-png
```
