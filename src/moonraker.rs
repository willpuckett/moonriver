use anyhow::{anyhow, Result};
use colored::Colorize;
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;

pub struct MoonrakerClient {
    write: mpsc::UnboundedSender<Message>,
    read: mpsc::UnboundedReceiver<String>,
    request_id: u64,
}

impl MoonrakerClient {
    pub async fn connect(url: &str) -> Result<Self> {
        let url = Url::parse(url)?;

        println!("{}", format!("Connecting to {}...", url).cyan());

        let (ws_stream, _) = connect_async(url.as_str())
            .await
            .map_err(|e| anyhow!("Failed to connect to Moonraker: {}", e))?;

        println!("{}", "Connected to Moonraker!".green().bold());

        let (ws_write, ws_read) = ws_stream.split();

        // Create channels for communication
        let (write_tx, mut write_rx) = mpsc::unbounded_channel::<Message>();
        let (read_tx, read_rx) = mpsc::unbounded_channel::<String>();

        // Spawn task to handle writing to WebSocket
        tokio::spawn(async move {
            let mut ws_write = ws_write;
            while let Some(msg) = write_rx.recv().await {
                if let Err(e) = ws_write.send(msg).await {
                    eprintln!("{}", format!("Error sending message: {}", e).red());
                    break;
                }
            }
        });

        // Spawn task to handle reading from WebSocket
        tokio::spawn(async move {
            let mut ws_read = ws_read;
            while let Some(msg_result) = ws_read.next().await {
                match msg_result {
                    Ok(Message::Text(text)) => {
                        if let Err(e) = read_tx.send(text.to_string()) {
                            eprintln!("{}", format!("Error forwarding message: {}", e).red());
                            break;
                        }
                    }
                    Ok(Message::Close(_)) => {
                        println!("{}", "Connection closed by server".yellow());
                        break;
                    }
                    Err(e) => {
                        eprintln!("{}", format!("WebSocket error: {}", e).red());
                        break;
                    }
                    _ => {}
                }
            }
        });

        let mut client = Self {
            write: write_tx,
            read: read_rx,
            request_id: 1,
        };

        // Subscribe to printer status updates
        client.subscribe_to_updates().await?;

        Ok(client)
    }

    async fn subscribe_to_updates(&mut self) -> Result<()> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "printer.objects.subscribe",
            "params": {
                "objects": {
                    "gcode_move": null,
                    "toolhead": null,
                    "extruder": null,
                    "heater_bed": null,
                    "print_stats": null,
                }
            },
            "id": self.next_id()
        });

        self.send_raw(&request.to_string()).await?;
        Ok(())
    }

    fn next_id(&mut self) -> u64 {
        let id = self.request_id;
        self.request_id += 1;
        id
    }

    async fn send_raw(&self, message: &str) -> Result<()> {
        self.write
            .send(Message::Text(message.to_string().into()))
            .map_err(|e| anyhow!("Failed to send message: {}", e))?;
        Ok(())
    }

    pub async fn send_gcode(&mut self, gcode: &str) -> Result<()> {
        let gcode = gcode.trim();

        // Check for emergency stop
        if gcode.to_uppercase() == "M112" {
            println!("{}", "🚨 EMERGENCY STOP TRIGGERED 🚨".red().bold());
            return self.emergency_stop().await;
        }

        let request = json!({
            "jsonrpc": "2.0",
            "method": "printer.gcode.script",
            "params": {
                "script": gcode
            },
            "id": self.next_id()
        });

        self.send_raw(&request.to_string()).await?;
        Ok(())
    }

    async fn emergency_stop(&mut self) -> Result<()> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "printer.emergency_stop",
            "params": {},
            "id": self.next_id()
        });

        self.send_raw(&request.to_string()).await?;
        Ok(())
    }

    pub async fn get_macros(&mut self) -> Result<Vec<String>> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "printer.gcode.help",
            "params": {},
            "id": self.next_id()
        });

        self.send_raw(&request.to_string()).await?;

        // Wait for response
        if let Some(response) = self.read.recv().await {
            if let Ok(value) = serde_json::from_str::<Value>(&response) {
                if let Some(result) = value.get("result") {
                    let mut macros = Vec::new();
                    if let Some(obj) = result.as_object() {
                        for key in obj.keys() {
                            macros.push(key.clone());
                        }
                    }
                    return Ok(macros);
                }
            }
        }

        Ok(Vec::new())
    }

    pub fn try_receive_message(&mut self) -> Option<String> {
        self.read.try_recv().ok()
    }

    pub async fn disconnect(self) -> Result<()> {
        // Close the write channel
        drop(self.write);
        println!("{}", "Disconnected from Moonraker".yellow());
        Ok(())
    }
}

pub fn format_response(response: &str) {
    if let Ok(value) = serde_json::from_str::<Value>(response) {
        // Handle JSON-RPC response
        if let Some(result) = value.get("result") {
            if result.is_string() {
                println!("{}", result.as_str().unwrap().green());
            } else {
                println!("{}", serde_json::to_string_pretty(&result).unwrap().green());
            }
        } else if let Some(error) = value.get("error") {
            println!("{}", format!("Error: {}", error).red().bold());
        } else if let Some(method) = value.get("method") {
            // Handle notification
            if method == "notify_gcode_response" {
                if let Some(params) = value.get("params").and_then(|p| p.get(0)) {
                    let msg = params.as_str().unwrap_or("");

                    // Color code based on content
                    if msg.contains("error") || msg.contains("!!") {
                        println!("{}", msg.red().bold());
                    } else if msg.contains("warning") || msg.contains("//") {
                        println!("{}", msg.yellow());
                    } else if msg.starts_with("ok") {
                        println!("{}", msg.green());
                    } else {
                        println!("{}", msg.cyan());
                    }
                }
            } else if method == "notify_status_update" {
                // Silently handle status updates (we could display these if needed)
            }
        }
    } else {
        // Plain text response
        println!("{}", response.green());
    }
}
