use crate::moonraker::{format_response, MoonrakerClient};
use anyhow::Result;
use colored::Colorize;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, CmdKind};
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Editor, Helper};
use std::borrow::Cow;
use std::collections::HashSet;

struct MoonriverHelper {
    macros: HashSet<String>,
    gcode_commands: HashSet<String>,
    klipper_commands: HashSet<String>,
}

impl MoonriverHelper {
    fn new() -> Self {
        let mut gcode_commands = HashSet::new();

        // Standard G-code commands from Klipper documentation
        // Basic movement and positioning
        let commands = vec![
            // G-codes (movement and basic commands)
            "G0", "G1", "G2", "G3", "G4", "G10", "G11", "G17", "G18", "G19",
            "G28", "G90", "G91", "G92",
            
            // M-codes (standard)
            "M18", "M84", "M82", "M83", "M104", "M105", "M106", "M107", "M109",
            "M112", "M114", "M115", "M117", "M118", "M119", "M140", "M190",
            "M204", "M220", "M221", "M400",
            
            // Additional M-codes from broader RepRap compatibility
            "M0", "M1", "M17", "M20", "M21", "M22", "M23", "M24", "M25", "M26", "M27",
            "M73", "M110", "M111", "M113", "M120", "M121", "M141", "M143", "M191",
            "M205", "M206", "M207", "M208", "M209", "M226", "M280", "M300", "M301",
            "M302", "M303", "M304", "M305", "M401", "M402", "M403", "M404", "M405",
            "M406", "M407", "M408", "M409", "M410", "M412", "M413", "M415", "M420",
            "M421", "M422", "M425", "M428", "M500", "M501", "M502", "M503", "M504",
            "M505", "M510", "M511", "M512", "M524", "M540", "M550", "M551", "M552",
            "M553", "M554", "M555", "M556", "M557", "M558", "M559", "M560", "M561",
            "M562", "M563", "M564", "M565", "M566", "M567", "M568", "M569", "M570",
            "M571", "M572", "M573", "M574", "M575", "M576", "M577", "M578", "M579",
            "M580", "M581", "M582", "M583", "M584", "M585", "M586", "M587", "M588",
            "M589", "M591", "M592", "M593", "M594", "M595", "M596", "M597", "M598",
            "M599", "M600", "M601", "M602", "M603", "M605", "M665", "M666", "M667",
            "M668", "M669", "M670", "M671", "M672", "M673", "M674", "M675", "M700",
            "M701", "M702", "M703", "M704", "M705", "M706", "M707", "M708", "M709",
            "M710", "M850", "M851", "M852", "M860", "M861", "M862", "M863", "M864",
            "M865", "M866", "M867", "M868", "M869", "M871", "M876", "M900", "M905",
            "M906", "M907", "M908", "M909", "M910", "M911", "M912", "M913", "M914",
            "M915", "M916", "M917", "M918", "M919", "M928", "M951", "M997", "M998",
            "M999",
        ];

        for cmd in commands {
            gcode_commands.insert(cmd.to_string());
        }

        // Klipper extended commands from documentation
        let mut klipper_commands = HashSet::new();
        let klipper_cmds = vec![
            // Core commands
            "HELP", "STATUS", "RESTART", "FIRMWARE_RESTART",
            
            // Movement and positioning
            "GET_POSITION", "SET_GCODE_OFFSET", "SAVE_GCODE_STATE", "RESTORE_GCODE_STATE",
            "SET_VELOCITY_LIMIT", "SET_KINEMATIC_POSITION",
            
            // Temperature
            "SET_HEATER_TEMPERATURE", "TURN_OFF_HEATERS", "TEMPERATURE_WAIT",
            "PID_CALIBRATE", "SET_TEMPERATURE_FAN_TARGET",
            
            // Bed mesh and calibration
            "BED_MESH_CALIBRATE", "BED_MESH_PROFILE", "BED_MESH_OUTPUT", "BED_MESH_MAP",
            "BED_MESH_CLEAR", "BED_MESH_OFFSET",
            "BED_SCREWS_ADJUST", "BED_TILT_CALIBRATE",
            "SCREWS_TILT_CALCULATE",
            
            // Probing
            "PROBE", "QUERY_PROBE", "PROBE_ACCURACY", "PROBE_CALIBRATE",
            "Z_OFFSET_APPLY_PROBE", "Z_OFFSET_APPLY_ENDSTOP",
            "MANUAL_PROBE", "Z_ENDSTOP_CALIBRATE",
            "ACTIVATE_PROBE", "DEACTIVATE_PROBE",
            
            // Delta calibration
            "DELTA_CALIBRATE", "DELTA_ANALYZE",
            
            // Extruder
            "ACTIVATE_EXTRUDER", "SET_PRESSURE_ADVANCE", "SET_EXTRUDER_ROTATION_DISTANCE",
            "SYNC_EXTRUDER_MOTION",
            
            // Firmware retraction
            "SET_RETRACTION", "GET_RETRACTION",
            
            // Stepper control
            "SET_STEPPER_ENABLE", "STEPPER_BUZZ", "FORCE_MOVE",
            
            // TMC drivers
            "DUMP_TMC", "INIT_TMC", "SET_TMC_CURRENT", "SET_TMC_FIELD",
            
            // Input shaper and resonance
            "SET_INPUT_SHAPER", "MEASURE_AXES_NOISE", "TEST_RESONANCES", "SHAPER_CALIBRATE",
            
            // Accelerometer
            "ACCELEROMETER_MEASURE", "ACCELEROMETER_QUERY",
            "ACCELEROMETER_DEBUG_READ", "ACCELEROMETER_DEBUG_WRITE",
            
            // Endstops and limits
            "QUERY_ENDSTOPS", "QUERY_ADC",
            
            // Print control
            "PAUSE", "RESUME", "CANCEL_PRINT", "CLEAR_PAUSE",
            "SDCARD_PRINT_FILE", "SDCARD_RESET_FILE",
            
            // Configuration
            "SAVE_CONFIG", "SET_GCODE_VARIABLE", "SAVE_VARIABLE",
            "SET_IDLE_TIMEOUT",
            
            // Display and output
            "SET_DISPLAY_TEXT", "SET_DISPLAY_GROUP", "RESPOND",
            
            // Fans and pins
            "SET_FAN_SPEED", "SET_PIN", "SET_LED", "SET_LED_TEMPLATE",
            
            // Gantry leveling
            "QUAD_GANTRY_LEVEL", "Z_TILT_ADJUST",
            
            // Skew correction
            "SET_SKEW", "GET_CURRENT_SKEW", "CALC_MEASURED_SKEW", "SKEW_PROFILE",
            
            // Manual stepper
            "MANUAL_STEPPER",
            
            // Tuning tower
            "TUNING_TOWER",
            
            // Filament sensors
            "QUERY_FILAMENT_SENSOR", "SET_FILAMENT_SENSOR",
            "QUERY_FILAMENT_WIDTH", "RESET_FILAMENT_WIDTH_SENSOR",
            "ENABLE_FILAMENT_WIDTH_SENSOR", "DISABLE_FILAMENT_WIDTH_SENSOR",
            "QUERY_RAW_FILAMENT_WIDTH", "ENABLE_FILAMENT_WIDTH_LOG",
            "DISABLE_FILAMENT_WIDTH_LOG",
            
            // Exclude object
            "EXCLUDE_OBJECT", "EXCLUDE_OBJECT_DEFINE", "EXCLUDE_OBJECT_START",
            "EXCLUDE_OBJECT_END",
            
            // Dual carriage
            "SET_DUAL_CARRIAGE", "SAVE_DUAL_CARRIAGE_STATE", "RESTORE_DUAL_CARRIAGE_STATE",
            
            // Palette 2
            "PALETTE_CONNECT", "PALETTE_DISCONNECT", "PALETTE_CLEAR",
            "PALETTE_CUT", "PALETTE_SMART_LOAD",
            
            // Smart effector
            "SET_SMART_EFFECTOR", "RESET_SMART_EFFECTOR",
            
            // Servo
            "SET_SERVO",
            
            // BLTouch
            "BLTOUCH_DEBUG", "BLTOUCH_STORE",
            
            // Delayed gcode
            "UPDATE_DELAYED_GCODE",
            
            // Angle calibration
            "ANGLE_CALIBRATE", "ANGLE_CHIP_CALIBRATE", "ANGLE_DEBUG_READ", "ANGLE_DEBUG_WRITE",
            
            // Endstop phase
            "ENDSTOP_PHASE_CALIBRATE",
            
            // Axis twist compensation
            "AXIS_TWIST_COMPENSATION_CALIBRATE",
            
            // Generic cartesian
            "SET_STEPPER_CARRIAGES",
            
            // Load cell
            "LOAD_CELL_DIAGNOSTIC", "LOAD_CELL_CALIBRATE", "LOAD_CELL_TARE",
            "LOAD_CELL_READ", "LOAD_CELL_TEST_TAP",
            
            // Probe eddy current
            "PROBE_EDDY_CURRENT_CALIBRATE", "LDC_CALIBRATE_DRIVE_CURRENT",
            
            // PWM cycle time
            "SET_DIGIPOT",
            
            // Print stats
            "SET_PRINT_STATS_INFO",
            
            // SDCard loop
            "SDCARD_LOOP_BEGIN", "SDCARD_LOOP_END", "SDCARD_LOOP_DESIST",
            
            // Temperature probe
            "TEMPERATURE_PROBE_CALIBRATE", "TEMPERATURE_PROBE_NEXT",
            "TEMPERATURE_PROBE_COMPLETE", "TEMPERATURE_PROBE_ENABLE",
            
            // Z thermal adjust
            "SET_Z_THERMAL_ADJUST",
        ];

        for cmd in klipper_cmds {
            klipper_commands.insert(cmd.to_string());
        }

        Self {
            macros: HashSet::new(),
            gcode_commands,
            klipper_commands,
        }
    }

    fn set_macros(&mut self, macros: Vec<String>) {
        self.macros = macros.into_iter().collect();
    }

    fn add_klipper_commands(&mut self, commands: Vec<String>) {
        for cmd in commands {
            self.klipper_commands.insert(cmd);
        }
    }
}

impl Completer for MoonriverHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let mut candidates = Vec::new();
        let input = &line[..pos];
        let input_upper = input.to_uppercase();

        // Check for G-code command completion
        for cmd in &self.gcode_commands {
            if cmd.starts_with(&input_upper) {
                candidates.push(Pair {
                    display: cmd.clone(),
                    replacement: cmd.clone(),
                });
            }
        }

        // Check for Klipper extended command completion
        for cmd in &self.klipper_commands {
            if cmd.starts_with(&input_upper) {
                candidates.push(Pair {
                    display: cmd.clone(),
                    replacement: cmd.clone(),
                });
            }
        }

        // Check for macro completion
        for macro_name in &self.macros {
            let macro_upper = macro_name.to_uppercase();
            if macro_upper.starts_with(&input_upper) {
                candidates.push(Pair {
                    display: macro_name.clone(),
                    replacement: macro_name.clone(),
                });
            }
        }

        Ok((0, candidates))
    }
}

impl Hinter for MoonriverHelper {
    type Hint = String;

    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
        None
    }
}

impl Highlighter for MoonriverHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        // Simple syntax highlighting
        let upper = line.to_uppercase();
        let first_word = upper.split_whitespace().next().unwrap_or("");

        // Check if it's a standard G-code command (G or M code)
        if (first_word.starts_with('G') || first_word.starts_with('M'))
            && self.gcode_commands.contains(first_word)
        {
            return Cow::Owned(line.bright_green().to_string());
        }

        // Check if it's a Klipper extended command
        if self.klipper_commands.contains(first_word) {
            return Cow::Owned(line.bright_blue().to_string());
        }

        // Check if it's a macro
        for macro_name in &self.macros {
            if upper.starts_with(&macro_name.to_uppercase()) {
                return Cow::Owned(line.bright_cyan().to_string());
            }
        }

        Cow::Borrowed(line)
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _forced: CmdKind) -> bool {
        true
    }
}

impl Validator for MoonriverHelper {}

impl Helper for MoonriverHelper {}

pub async fn run_repl(mut client: MoonrakerClient) -> Result<()> {
    let mut rl = Editor::new()?;
    let mut helper = MoonriverHelper::new();

    // Try to load macros
    if let Ok(macros) = client.get_macros().await {
        helper.set_macros(macros);
    }

    // Try to get available commands from HELP dynamically
    if let Ok(()) = client.send_gcode("HELP").await {
        // Wait longer for HELP response - it can be large
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        
        // Parse HELP response to extract command names
        let mut help_commands = Vec::new();
        
        // Keep checking for messages until we get the HELP result
        while let Some(msg) = client.try_receive_message() {
            // Try to parse as JSON
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&msg) {
                // Check for JSON-RPC result containing HELP output
                if let Some(result) = json.get("result") {
                    if let Some(result_str) = result.as_str() {
                        // Parse the text format HELP output
                        for line in result_str.lines() {
                            if line.starts_with("//") || line.contains(":") {
                                // Extract command name from lines like "// COMMAND_NAME: description"
                                let cmd_part = if line.starts_with("//") {
                                    line.strip_prefix("//").unwrap_or(line).trim()
                                } else {
                                    line.trim()
                                };
                                
                                if let Some(cmd_name) = cmd_part.split(':').next() {
                                    let cmd = cmd_name.trim();
                                    if !cmd.is_empty() 
                                        && !cmd.starts_with("_") 
                                        && cmd.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                                        help_commands.push(cmd.to_string());
                                    }
                                }
                            }
                        }
                    } else if let Some(obj) = result.as_object() {
                        // Some Moonraker versions return HELP as a JSON object
                        for key in obj.keys() {
                            if !key.starts_with("_") && key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                                help_commands.push(key.clone());
                            }
                        }
                    }
                }
                
                // Also check notify_gcode_response for HELP output
                if let Some(method) = json.get("method")
                    && method == "notify_gcode_response"
                    && let Some(params) = json.get("params").and_then(|p| p.get(0))
                {
                    let msg_text = params.as_str().unwrap_or("");
                    for line in msg_text.lines() {
                        if line.starts_with("//")
                            && let Some(cmd_part) = line.strip_prefix("//").map(|s| s.trim())
                            && let Some(cmd_name) = cmd_part.split(':').next()
                        {
                            let cmd = cmd_name.trim();
                            if !cmd.is_empty() 
                                && !cmd.starts_with("_") 
                                && cmd.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                                help_commands.push(cmd.to_string());
                            }
                        }
                    }
                }
            }
        }
        
        if !help_commands.is_empty() {
            helper.add_klipper_commands(help_commands);
        }
    }

    rl.set_helper(Some(helper));

    // Load history if it exists
    let history_path = dirs::home_dir().map(|p| p.join(".moonriver_history"));
    if let Some(ref path) = history_path {
        let _ = rl.load_history(path);
    }

    println!(
        "\n{}",
        "🌙 Moonriver - Klipper Console 🌙".bright_magenta().bold()
    );
    println!(
        "{}",
        "Type your G-code commands below. Use Ctrl+D or 'exit' to quit.".cyan()
    );
    println!(
        "{}",
        "Use ',' to separate multiple commands on one line.".cyan()
    );
    println!("{}", "Type 'M112' for emergency stop.\n".yellow().bold());

    loop {
        // Check for any incoming messages before showing prompt
        while let Some(msg) = client.try_receive_message() {
            print!("\r\x1b[K"); // Clear current line
            format_response(&msg);
        }

        let readline = rl.readline(&format!("{} ", ">".bright_blue().bold()));

        match readline {
            Ok(line) => {
                let line = line.trim();

                if line.is_empty() {
                    continue;
                }

                rl.add_history_entry(line)?;

                // Check for exit command
                if line.to_lowercase() == "exit" || line.to_lowercase() == "quit" {
                    println!("{}", "Goodbye!".cyan());
                    break;
                }

                // Split by comma to support multiple commands
                let commands: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

                for cmd in commands {
                    if !cmd.is_empty() {
                        if let Err(e) = client.send_gcode(cmd).await {
                            eprintln!("{}", format!("Error sending command: {}", e).red());
                            continue;
                        }

                        // Wait for responses - use a timeout approach
                        let start = tokio::time::Instant::now();
                        let mut got_response = false;
                        let mut last_message_time = tokio::time::Instant::now();
                        
                        // Wait up to 2 seconds for responses, checking every 30ms
                        // Give 250ms after the last message to ensure all responses arrive
                        while start.elapsed() < tokio::time::Duration::from_millis(2000) {
                            // Check for messages
                            let mut found_msg = false;
                            while let Some(msg) = client.try_receive_message() {
                                format_response(&msg);
                                found_msg = true;
                                got_response = true;
                                last_message_time = tokio::time::Instant::now();
                            }
                            
                            // If we got messages, continue checking
                            if found_msg {
                                tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;
                            } else if got_response && last_message_time.elapsed() > tokio::time::Duration::from_millis(250) {
                                // We've received responses and 250ms has passed with no new messages - we're done
                                break;
                            } else {
                                // No response yet, wait a bit and check again
                                tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;
                            }
                        }
                        
                        // Final drain of any remaining messages
                        tokio::time::sleep(tokio::time::Duration::from_millis(80)).await;
                        while let Some(msg) = client.try_receive_message() {
                            format_response(&msg);
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("{}", "Ctrl+C pressed. Type 'exit' to quit.".yellow());
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "Goodbye!".cyan());
                break;
            }
            Err(err) => {
                eprintln!("{}", format!("Error: {:?}", err).red());
                break;
            }
        }
    }

    // Save history
    if let Some(ref path) = history_path {
        let _ = rl.save_history(path);
    }

    client.disconnect().await?;
    Ok(())
}
