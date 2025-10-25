# REPL API

The REPL (Read-Eval-Print Loop) module provides the interactive terminal interface.

## Main Function

### `run_repl`

Starts the interactive REPL session.

```rust
pub async fn run_repl(client: MoonrakerClient) -> Result<()>
```

**Parameters:**
- `client` - Connected `MoonrakerClient` instance

**Returns:**
- `Result<()>` - Success when user exits, or error

**Features:**
- Command history (saved to `~/.moonriver_history`)
- Tab completion for G-code and macros
- Syntax highlighting
- Arrow key navigation
- Ctrl+R history search

**Example:**

```rust
use moonriver::moonraker::MoonrakerClient;
use moonriver::repl::run_repl;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MoonrakerClient::connect("ws://localhost:7125/websocket").await?;
    run_repl(client).await?;
    Ok(())
}
```

## Internal Components

### MoonriverHelper

Internal struct that implements rustyline traits for enhanced functionality.

```rust
struct MoonriverHelper {
    macros: HashSet<String>,
    gcode_commands: HashSet<String>,
}
```

**Implements:**
- `Completer` - Tab completion
- `Highlighter` - Syntax highlighting
- `Hinter` - Command hints (future)
- `Validator` - Input validation (future)
- `Helper` - Combines all traits

## Completion

The REPL provides intelligent tab completion:

### G-code Commands

Pre-populated with common G-code commands:
- Movement: G0, G1, G2, G3, G28, etc.
- Temperature: M104, M105, M109, M140, M190, etc.
- Configuration: M500, M501, M502, M503, etc.
- And many more...

### Klipper Macros

Dynamically loaded from the connected printer:
- Custom user macros
- Built-in Klipper commands
- Configuration-specific commands

### Case Insensitive

Completion works regardless of input case:
```
> g28<Tab>  → G28
> m105<Tab> → M105
```

## Syntax Highlighting

Commands are highlighted as you type:

- **Bright Green**: Recognized G-code commands
- **Bright Cyan**: Klipper macros
- **White**: Unrecognized commands

## Command History

### Storage

History is automatically saved to:
```
~/.moonriver_history
```

### Navigation

- `↑` / `Ctrl+P` - Previous command
- `↓` / `Ctrl+N` - Next command
- `Ctrl+R` - Search history backward

### Persistence

- History loaded on startup
- Saved on exit
- Survives across sessions

## Input Handling

### Special Commands

#### Exit
```
> exit
> quit
```
Or press `Ctrl+D`

#### Emergency Stop
```
> M112
```
Immediately sends emergency stop

### Multiple Commands

Separate commands with commas:
```
> G28, M105, GET_POSITION
```

### Command Processing

1. User input captured via rustyline
2. Input trimmed and validated
3. Commands split by comma
4. Each command sent via MoonrakerClient
5. Responses processed and displayed

## Error Handling

The REPL handles various errors gracefully:

### Connection Errors

```rust
Err(ReadlineError::Interrupted) => {
    // Ctrl+C pressed, continue
}

Err(ReadlineError::Eof) => {
    // Ctrl+D pressed, exit
    break;
}
```

### Command Errors

Errors are displayed in red but don't crash the REPL:

```rust
if let Err(e) = client.send_gcode(cmd).await {
    eprintln!("{}", format!("Error: {}", e).red());
}
```

## Message Processing

The REPL continuously checks for messages:

```rust
loop {
    // Check for incoming messages
    while let Some(msg) = client.try_receive_message() {
        format_response(&msg);
    }
    
    // Get user input
    let line = rl.readline("> ")?;
    
    // Process command...
}
```

## Customization

### Prompt

The prompt is customizable:

```rust
let prompt = format!("{} ", ">".bright_blue().bold());
rl.readline(&prompt)?;
```

### Welcome Message

Displayed on startup:

```rust
println!("🌙 Moonriver - Klipper Console 🌙");
println!("Type commands or 'exit' to quit.");
```

## Integration Example

Complete example showing REPL integration:

```rust
use moonriver::moonraker::MoonrakerClient;
use moonriver::repl::run_repl;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI args (not shown)
    let host = "192.168.1.100";
    let port = 7125;
    
    // Build URL
    let url = format!("ws://{}:{}/websocket", host, port);
    
    // Connect
    let client = MoonrakerClient::connect(&url).await?;
    
    // Start REPL
    run_repl(client).await?;
    
    Ok(())
}
```

## Dependencies

The REPL module uses:

- **rustyline** - Line editing and history
- **colored** - Terminal colors
- **tokio** - Async runtime

## Future Enhancements

Planned features:

- Command hints as you type
- Custom color schemes
- Configurable keybindings
- Multi-line command support
- Command aliases

## Next Steps

- [MoonrakerClient API](/api/client) - WebSocket client
- [Interactive Mode Guide](/guide/interactive-mode) - User guide
- [Tab Completion](/features/tab-completion) - Completion details
