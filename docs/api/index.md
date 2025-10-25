# API Reference

Documentation for the Moonriver Rust API.

## Overview

Moonriver is built with a modular architecture that separates concerns:

- **CLI** - Command-line argument parsing
- **MoonrakerClient** - WebSocket communication with Moonraker
- **REPL** - Interactive terminal interface
- **Config** - Configuration structures

## Modules

### `cli`

Handles command-line argument parsing using `clap`.

```rust
pub struct Cli {
    pub host: String,
    pub port: u16,
    pub api_key: Option<String>,
    pub commands: Vec<String>,
}
```

### `moonraker`

WebSocket client for communicating with Moonraker.

```rust
pub struct MoonrakerClient {
    // Internal fields omitted
}

impl MoonrakerClient {
    pub async fn connect(url: &str) -> Result<Self>;
    pub async fn send_gcode(&mut self, gcode: &str) -> Result<()>;
    pub async fn get_macros(&mut self) -> Result<Vec<String>>;
    pub fn try_receive_message(&mut self) -> Option<String>;
    pub async fn disconnect(self) -> Result<()>;
}

pub fn format_response(response: &str);
```

### `repl`

Interactive REPL interface with history and completion.

```rust
pub async fn run_repl(client: MoonrakerClient) -> Result<()>;
```

### `config`

Configuration structures (planned for future use).

```rust
pub struct Config {
    pub host: String,
    pub port: u16,
    pub api_key: Option<String>,
}
```

## Usage as a Library

While Moonriver is primarily a CLI tool, you can use it as a library:

```toml
[dependencies]
moonriver = "0.1"
```

Example:

```rust
use moonriver::moonraker::MoonrakerClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "ws://192.168.1.100:7125/websocket";
    let mut client = MoonrakerClient::connect(url).await?;
    
    client.send_gcode("G28").await?;
    
    // Process responses
    while let Some(msg) = client.try_receive_message() {
        println!("{}", msg);
    }
    
    client.disconnect().await?;
    Ok(())
}
```

## Dependencies

Moonriver uses these key dependencies:

- **tokio** - Async runtime
- **tokio-tungstenite** - WebSocket client
- **rustyline** - Line editing and history
- **clap** - CLI argument parsing
- **colored** - Terminal colors
- **serde/serde_json** - JSON serialization
- **anyhow** - Error handling

## Error Handling

Moonriver uses `anyhow::Result` for error handling:

```rust
use anyhow::Result;

async fn example() -> Result<()> {
    // Operations that may fail
    Ok(())
}
```

## Async Runtime

All I/O operations are async using Tokio:

```rust
#[tokio::main]
async fn main() -> Result<()> {
    // Async code here
    Ok(())
}
```

## Next Steps

- [Client API](/api/client) - MoonrakerClient details
- [REPL API](/api/repl) - REPL implementation
- [Source Code](https://github.com/yourusername/moonriver) - View on GitHub
