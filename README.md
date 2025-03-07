# UDP Forwarder

## Overview

UDP Forwarder is an asynchronous Rust-based application that forwards UDP packets from a source address to a destination address. The program is designed using `tokio` for asynchronous networking and `log` for logging.

## Features

- Asynchronous UDP forwarding using `tokio`
- Configurable source and destination addresses via command-line arguments
- Graceful shutdown on Ctrl+C
- Error handling with `thiserror`

## Requirements

- Rust (latest stable version recommended)
- Cargo

## Installation

Clone the repository and navigate to the project directory:

```sh
cd udp_forwarder
```

Build the project:

```sh
cargo build --release
```

## Usage

Run the UDP forwarder with the required arguments:

```sh
cargo run --release -- <source_address> <destination_address>
```

Example:

```sh
cargo run --release -- 127.0.0.1:8080 192.168.1.100:9090
```

This will forward UDP packets from `127.0.0.1:8080` to `192.168.1.100:9090`.

## Code Structure

- `main.rs` - Entry point of the application.
- `config.rs` - Loads and parses configuration parameters.
- `error.rs` - Defines error handling using `thiserror`.
- `forwarder.rs` - Implements UDP forwarding logic.

## Logging

Logging is enabled via `env_logger`. To enable debug logging, run:

```sh
RUST_LOG=debug cargo run --release -- <source_address> <destination_address>
```

## License

This project is licensed under the MIT License.
