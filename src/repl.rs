use crate::moonraker::{format_response, MoonrakerClient};
use anyhow::Result;
use colored::Colorize;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Editor, Helper};
use std::borrow::Cow;
use std::collections::HashSet;

struct MoonriverHelper {
    macros: HashSet<String>,
    gcode_commands: HashSet<String>,
}

impl MoonriverHelper {
    fn new() -> Self {
        let mut gcode_commands = HashSet::new();

        // Common G-code commands
        let commands = vec![
            "G0", "G1", "G2", "G3", "G4", "G10", "G11", "G28", "G29", "G90", "G91", "G92", "M0",
            "M1", "M17", "M18", "M20", "M21", "M22", "M23", "M24", "M25", "M26", "M27", "M28",
            "M29", "M30", "M31", "M32", "M104", "M105", "M106", "M107", "M109", "M110", "M111",
            "M112", "M113", "M114", "M115", "M117", "M118", "M119", "M120", "M121", "M140", "M141",
            "M143", "M190", "M191", "M204", "M205", "M206", "M207", "M208", "M209", "M220", "M221",
            "M226", "M280", "M300", "M301", "M302", "M303", "M304", "M305", "M400", "M401", "M402",
            "M403", "M404", "M405", "M406", "M407", "M408", "M409", "M410", "M412", "M413", "M415",
            "M420", "M421", "M422", "M425", "M428", "M500", "M501", "M502", "M503", "M504", "M505",
            "M510", "M511", "M512", "M524", "M540", "M550", "M551", "M552", "M553", "M554", "M555",
            "M556", "M557", "M558", "M559", "M560", "M561", "M562", "M563", "M564", "M565", "M566",
            "M567", "M568", "M569", "M570", "M571", "M572", "M573", "M574", "M575", "M576", "M577",
            "M578", "M579", "M580", "M581", "M582", "M583", "M584", "M585", "M586", "M587", "M588",
            "M589", "M591", "M592", "M593", "M594", "M595", "M596", "M597", "M598", "M599", "M600",
            "M601", "M602", "M603", "M605", "M665", "M666", "M667", "M668", "M669", "M670", "M671",
            "M672", "M673", "M674", "M675", "M700", "M701", "M702", "M703", "M704", "M705", "M706",
            "M707", "M708", "M709", "M710", "M850", "M851", "M852", "M860", "M861", "M862", "M863",
            "M864", "M865", "M866", "M867", "M868", "M869", "M871", "M876", "M900", "M905", "M906",
            "M907", "M908", "M909", "M910", "M911", "M912", "M913", "M914", "M915", "M916", "M917",
            "M918", "M919", "M928", "M951", "M997", "M998", "M999",
        ];

        for cmd in commands {
            gcode_commands.insert(cmd.to_string());
        }

        Self {
            macros: HashSet::new(),
            gcode_commands,
        }
    }

    fn set_macros(&mut self, macros: Vec<String>) {
        self.macros = macros.into_iter().collect();
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

        // Check if it's a G-code command
        if upper.starts_with('G') || upper.starts_with('M') {
            return Cow::Owned(line.bright_green().to_string());
        }

        // Check if it's a macro
        for macro_name in &self.macros {
            if upper.starts_with(&macro_name.to_uppercase()) {
                return Cow::Owned(line.bright_cyan().to_string());
            }
        }

        Cow::Borrowed(line)
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _forced: bool) -> bool {
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
        // Check for any incoming messages
        while let Some(msg) = client.try_receive_message() {
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
                        }

                        // Give some time for responses to come in
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

                        // Check for responses
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
