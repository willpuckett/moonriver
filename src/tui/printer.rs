/// Print job from history
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PrintJob {
    pub job_id: String,
    pub filename: String,
    pub status: String, // completed, error, cancelled
    pub start_time: f64,
    pub end_time: f64,
    pub total_duration: f64,
    pub filament_used: f64,
    pub print_duration: f64,
}

impl Default for PrintJob {
    fn default() -> Self {
        PrintJob {
            job_id: String::new(),
            filename: String::new(),
            status: String::new(),
            start_time: 0.0,
            end_time: 0.0,
            total_duration: 0.0,
            filament_used: 0.0,
            print_duration: 0.0,
        }
    }
}

/// Printer connection and state information
#[derive(Debug, Clone)]
pub struct PrinterState {
    pub connected: bool,
    pub state: String,
    pub temperatures: Temperatures,
    pub toolhead: Toolhead,
    pub print_stats: PrintStats,
}

impl Default for PrinterState {
    fn default() -> Self {
        PrinterState {
            connected: false,
            state: "disconnected".to_string(),
            temperatures: Temperatures::default(),
            toolhead: Toolhead::default(),
            print_stats: PrintStats::default(),
        }
    }
}

/// Temperature information
#[derive(Debug, Clone, Default)]
pub struct Temperatures {
    pub extruder: HeaterState,
    pub bed: HeaterState,
    pub chamber: Option<HeaterState>,
    pub mcus: Vec<McuTemp>,
    pub fans: Vec<FanState>,
}

/// MCU temperature sensor
#[derive(Debug, Clone)]
pub struct McuTemp {
    pub name: String,
    pub temperature: f64,
}

/// Fan state
#[derive(Debug, Clone)]
pub struct FanState {
    pub name: String,
    pub speed: f64, // 0.0 to 1.0
    pub rpm: Option<f64>,
}

/// Individual heater state
#[derive(Debug, Clone, Default)]
pub struct HeaterState {
    pub temperature: f64,
    pub target: f64,
    pub power: f64,
}

/// Toolhead position and state
#[derive(Debug, Clone, Default)]
pub struct Toolhead {
    pub position: [f64; 4], // X, Y, Z, E
    pub homed_axes: Vec<String>,
    #[allow(dead_code)]
    pub print_time: f64,
}

/// Print statistics
#[derive(Debug, Clone, Default)]
pub struct PrintStats {
    pub state: String, // standby, printing, paused, complete, cancelled, error
    pub filename: String,
    pub total_duration: f64,
    pub print_duration: f64,
    pub filament_used: f64,
}

/// Parse printer object update from Moonraker
pub fn update_from_json(state: &mut PrinterState, data: &serde_json::Value) {
    // Moonraker sends updates in two formats:
    // 1. Initial subscription response: { "result": { "status": { ... } } }
    // 2. Status updates: { "method": "notify_status_update", "params": [{ ... }] }
    
    let status = if let Some(result) = data.get("result") {
        // Initial subscription response
        result.get("status")
    } else if let Some(method) = data.get("method").and_then(|m| m.as_str()) {
        if method == "notify_status_update" {
            // Status update notification - params is an array with status as first element
            data.get("params").and_then(|p| p.get(0))
        } else {
            None
        }
    } else {
        // Direct status object
        data.get("status")
    };
    
    if let Some(status) = status {
        // Update extruder temperature
        if let Some(extruder) = status.get("extruder") {
            if let Some(temp) = extruder.get("temperature").and_then(|v| v.as_f64()) {
                state.temperatures.extruder.temperature = temp;
            }
            if let Some(target) = extruder.get("target").and_then(|v| v.as_f64()) {
                state.temperatures.extruder.target = target;
            }
            if let Some(power) = extruder.get("power").and_then(|v| v.as_f64()) {
                state.temperatures.extruder.power = power;
            }
        }

        // Update bed temperature
        if let Some(bed) = status.get("heater_bed") {
            if let Some(temp) = bed.get("temperature").and_then(|v| v.as_f64()) {
                state.temperatures.bed.temperature = temp;
            }
            if let Some(target) = bed.get("target").and_then(|v| v.as_f64()) {
                state.temperatures.bed.target = target;
            }
            if let Some(power) = bed.get("power").and_then(|v| v.as_f64()) {
                state.temperatures.bed.power = power;
            }
        }

        // Update MCU temperatures and fans
        // Update individual sensors rather than replacing the entire list
        if let Some(obj) = status.as_object() {
            for (key, value) in obj.iter() {
                // MCU temperature sensors
                // Look for: "temperature_sensor <name>", "temperature_fan <name>", "mcu", "temperature_host"
                let is_temp_sensor = key.starts_with("temperature_sensor ") || 
                                    key.starts_with("temperature_fan ") ||
                                    key == "mcu" || 
                                    key == "temperature_host";
                
                if is_temp_sensor
                    && let Some(temp) = value.get("temperature").and_then(|v| v.as_f64()) {
                        let name = if let Some(stripped) = key.strip_prefix("temperature_sensor ") {
                            stripped.to_string()
                        } else if let Some(stripped) = key.strip_prefix("temperature_fan ") {
                            stripped.to_string()
                        } else if key == "temperature_host" {
                            "Host".to_string()
                        } else {
                            key.to_uppercase()
                        };
                        
                        // Update existing MCU or add new one
                        if let Some(existing) = state.temperatures.mcus.iter_mut().find(|m| m.name == name) {
                            existing.temperature = temp;
                        } else {
                            state.temperatures.mcus.push(McuTemp {
                                name,
                                temperature: temp,
                            });
                        }
                    }
                
                // Fans - look for fan objects with speed property
                let is_fan = key == "fan" || 
                            key.starts_with("heater_fan ") || 
                            key.starts_with("controller_fan ") || 
                            key.starts_with("temperature_fan ");
                
                if is_fan
                    && let Some(speed) = value.get("speed").and_then(|v| v.as_f64()) {
                        let rpm = value.get("rpm").and_then(|v| v.as_f64());
                        let name = if key == "fan" {
                            "Part".to_string()
                        } else if let Some(stripped) = key.strip_prefix("heater_fan ") {
                            stripped.to_string()
                        } else if let Some(stripped) = key.strip_prefix("controller_fan ") {
                            stripped.to_string()
                        } else if let Some(stripped) = key.strip_prefix("temperature_fan ") {
                            stripped.to_string()
                        } else {
                            key.clone()
                        };
                        
                        // Update existing fan or add new one
                        if let Some(existing) = state.temperatures.fans.iter_mut().find(|f| f.name == name) {
                            existing.speed = speed;
                            existing.rpm = rpm;
                        } else {
                            state.temperatures.fans.push(FanState {
                                name,
                                speed,
                                rpm,
                            });
                        }
                    }
            }
        }

        // Update toolhead position
        if let Some(toolhead) = status.get("toolhead") {
            if let Some(position) = toolhead.get("position").and_then(|v| v.as_array()) {
                for (i, val) in position.iter().enumerate() {
                    if i < 4
                        && let Some(pos) = val.as_f64() {
                            state.toolhead.position[i] = pos;
                        }
                }
            }
            if let Some(homed) = toolhead.get("homed_axes").and_then(|v| v.as_str()) {
                state.toolhead.homed_axes = homed.chars().map(|c| c.to_string()).collect();
            }
        }

        // Update print stats
        if let Some(print_stats) = status.get("print_stats") {
            if let Some(state_str) = print_stats.get("state").and_then(|v| v.as_str()) {
                state.print_stats.state = state_str.to_string();
            }
            if let Some(filename) = print_stats.get("filename").and_then(|v| v.as_str()) {
                state.print_stats.filename = filename.to_string();
            }
            if let Some(duration) = print_stats.get("total_duration").and_then(|v| v.as_f64()) {
                state.print_stats.total_duration = duration;
            }
            if let Some(duration) = print_stats.get("print_duration").and_then(|v| v.as_f64()) {
                state.print_stats.print_duration = duration;
            }
            if let Some(filament) = print_stats.get("filament_used").and_then(|v| v.as_f64()) {
                state.print_stats.filament_used = filament;
            }
        }
    }
}
