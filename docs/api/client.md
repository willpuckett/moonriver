# MoonrakerClient API

The `MoonrakerClient` is the core component for communicating with Moonraker via WebSocket.

## Struct Definition

```rust
pub struct MoonrakerClient {
    // Internal implementation details
}
```

## Methods

### `connect`

Establishes a WebSocket connection to Moonraker.

```rust
pub async fn connect(url: &str) -> Result<Self>
```

**Parameters:**
- `url` - WebSocket URL (e.g., `ws://192.168.1.100:7125/websocket`)

**Returns:**
- `Result<MoonrakerClient>` - Connected client or error

**Example:**

```rust
let client = MoonrakerClient::connect("ws://localhost:7125/websocket").await?;
```

### `send_gcode`

Sends a G-code command to the printer.

```rust
pub async fn send_gcode(&mut self, gcode: &str) -> Result<()>
```

**Parameters:**
- `gcode` - G-code command or Klipper macro to execute

**Returns:**
- `Result<()>` - Success or error

**Special Handling:**
- M112 commands trigger emergency stop
- Commands are sent via `printer.gcode.script` JSON-RPC method

**Example:**

```rust
client.send_gcode("G28").await?;
client.send_gcode("M104 S200").await?;
client.send_gcode("PRINT_START").await?;
```

### `get_macros`

Retrieves available G-code commands and macros from Klipper.

```rust
pub async fn get_macros(&mut self) -> Result<Vec<String>>
```

**Returns:**
- `Result<Vec<String>>` - List of available command names

**Example:**

```rust
let macros = client.get_macros().await?;
for macro_name in macros {
    println!("{}", macro_name);
}
```

### `try_receive_message`

Non-blocking check for incoming messages from Moonraker.

```rust
pub fn try_receive_message(&mut self) -> Option<String>
```

**Returns:**
- `Some(String)` - Message if available
- `None` - No message ready

**Example:**

```rust
while let Some(msg) = client.try_receive_message() {
    println!("Received: {}", msg);
}
```

### `disconnect`

Closes the WebSocket connection gracefully.

```rust
pub async fn disconnect(self) -> Result<()>
```

**Example:**

```rust
client.disconnect().await?;
```

## Helper Functions

### `format_response`

Formats and prints a Moonraker response with appropriate coloring.

```rust
pub fn format_response(response: &str)
```

**Parameters:**
- `response` - Raw JSON-RPC response from Moonraker

**Color Coding:**
- Green: Successful responses, "ok" messages
- Cyan: Informational messages
- Yellow: Warnings, "//" prefixed messages
- Red: Errors, "!!" prefixed messages

**Example:**

```rust
let msg = client.try_receive_message();
if let Some(response) = msg {
    format_response(&response);
}
```

## Message Flow

### Connection

1. Client calls `connect(url)`
2. WebSocket established
3. Client subscribes to printer status updates
4. Client is ready for commands

### Sending Commands

1. Client calls `send_gcode(command)`
2. Command wrapped in JSON-RPC request
3. Sent via WebSocket
4. Moonraker processes command
5. Responses arrive asynchronously

### Receiving Messages

Messages from Moonraker come in several forms:

#### Response Messages

```json
{
  "jsonrpc": "2.0",
  "result": "ok",
  "id": 1
}
```

#### Notification Messages

```json
{
  "jsonrpc": "2.0",
  "method": "notify_gcode_response",
  "params": ["// Homing complete"]
}
```

#### Status Updates

```json
{
  "jsonrpc": "2.0",
  "method": "notify_status_update",
  "params": [{
    "eventtime": 123456.789,
    "status": {
      "extruder": {
        "temperature": 200.5,
        "target": 200.0
      }
    }
  }]
}
```

## Error Handling

All methods return `Result<T>` using the `anyhow` crate:

```rust
use anyhow::Result;

async fn example() -> Result<()> {
    let mut client = MoonrakerClient::connect("ws://localhost:7125/websocket").await?;
    client.send_gcode("G28").await?;
    client.disconnect().await?;
    Ok(())
}
```

## Complete Example

```rust
use moonriver::moonraker::MoonrakerClient;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect
    let mut client = MoonrakerClient::connect(
        "ws://192.168.1.100:7125/websocket"
    ).await?;
    
    // Send commands
    client.send_gcode("G28").await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    client.send_gcode("M105").await?;
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // Process responses
    while let Some(msg) = client.try_receive_message() {
        moonriver::moonraker::format_response(&msg);
    }
    
    // Disconnect
    client.disconnect().await?;
    
    Ok(())
}
```

## Thread Safety

`MoonrakerClient` is **not** thread-safe. Use one client per connection and don't share across threads without proper synchronization.

For multiple printers, create multiple client instances.

## Async Requirements

All I/O methods are async and require a Tokio runtime:

```rust
#[tokio::main]
async fn main() -> Result<()> {
    // Your async code here
}
```

## Next Steps

- [REPL API](/api/repl) - Interactive interface
- [Usage Examples](/guide/getting-started) - Practical examples
- [Source Code](https://github.com/willpuckett/moonriver) - Implementation details
