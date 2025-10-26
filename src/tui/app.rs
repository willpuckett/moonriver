use super::printer::PrinterState;
use super::stateful_list::StatefulList;
use super::tabs::Tab;
use crate::config::Config;
use crate::moonraker::MoonrakerClient;
use crate::tui::event::Event;
use crate::tui::printer::PrintJob;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};

/// Console message types
#[derive(Debug, Clone)]
pub enum ConsoleMessage {
    Command(String),
    Response(String),
    Error(String),
    Info(String),
}

/// Panel visibility state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelVisibility {
    pub temperature: bool,
    pub job_history: bool,
    pub position: bool,
}

impl Default for PanelVisibility {
    fn default() -> Self {
        PanelVisibility {
            temperature: true,
            job_history: true,
            position: true,
        }
    }
}

/// Input modes for text fields
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Editing,
}

/// Temperature heater being edited
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TempEditTarget {
    Extruder,
    Bed,
}

/// Fan being edited (index into the fans array)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FanEditTarget {
    pub index: usize,
}

/// Position axis being edited
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PosEditTarget {
    X,
    Y,
    Z,
}

/// Input field state
#[derive(Debug, Clone)]
pub struct InputState {
    pub value: String,
    pub cursor_position: u16,
    pub mode: InputMode,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            value: String::new(),
            cursor_position: 0,
            mode: InputMode::Normal,
        }
    }

    pub fn enter_edit_mode(&mut self) {
        self.mode = InputMode::Editing;
        self.cursor_position = self.value.len() as u16;
    }

    pub fn exit_edit_mode(&mut self) {
        self.mode = InputMode::Normal;
    }

    pub fn clear(&mut self) {
        self.value.clear();
        self.cursor_position = 0;
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

/// Main application state
pub struct App {
    /// Whether the application should keep running
    pub running: bool,
    /// Current active tab
    pub current_tab: Tab,
    /// Panel visibility state
    pub panel_visibility: PanelVisibility,
    /// Console scroll position
    pub console_scroll: u16,
    /// Console input field
    pub console_input: InputState,
    /// Temperature editing input field
    pub temp_input: InputState,
    /// Which temperature is being edited (if any)
    pub temp_edit_target: Option<TempEditTarget>,
    /// Fan speed editing input field
    pub fan_input: InputState,
    /// Which fan is being edited (if any)
    pub fan_edit_target: Option<FanEditTarget>,
    /// Position editing input field
    pub pos_input: InputState,
    /// Which position axis is being edited (if any)
    pub pos_edit_target: Option<PosEditTarget>,
    /// Console message history
    pub console_messages: Vec<ConsoleMessage>,
    /// Command history for up/down navigation
    pub command_history: Vec<String>,
    /// Current position in command history (None = not browsing)
    pub history_index: Option<usize>,
    /// Pending commands to send
    pub pending_commands: Vec<String>,
    /// Print job history
    pub job_list: StatefulList<PrintJob>,
    /// Configuration
    #[allow(dead_code)]
    pub config: Config,
    /// Server URL
    pub server_url: String,
    /// HTTP base URL (for REST API calls)
    pub http_url: String,
    /// Printer state
    pub printer: PrinterState,
    /// Optional Moonraker client (will be set after connection)
    pub client: Option<MoonrakerClient>,
    /// HTTP client for REST API calls
    pub http_client: reqwest::Client,
    /// Power device click areas (stored after rendering)
    pub power_device_click_areas: Vec<(ratatui::layout::Rect, String)>,
}

impl App {
    pub fn new(server_url: String, config: Config) -> Self {
        let http_url = format!("http://{}", server_url);
        
        // Initialize console input in editing mode so cursor is active
        let mut console_input = InputState::new();
        console_input.enter_edit_mode();
        
        App {
            running: true,
            current_tab: Tab::default(),
            panel_visibility: PanelVisibility::default(),
            console_scroll: 0,
            console_input,
            temp_input: InputState::new(),
            temp_edit_target: None,
            fan_input: InputState::new(),
            fan_edit_target: None,
            pos_input: InputState::new(),
            pos_edit_target: None,
            console_messages: Vec::new(),
            command_history: Vec::new(),
            history_index: None,
            pending_commands: Vec::new(),
            job_list: StatefulList::with_items(Vec::new()),
            config,
            http_url,
            server_url,
            printer: PrinterState::default(),
            client: None,
            http_client: reqwest::Client::new(),
            power_device_click_areas: Vec::new(),
        }
    }

    /// Set the Moonraker client
    pub fn set_client(&mut self, client: MoonrakerClient) {
        self.printer.connected = true;
        self.printer.state = "connecting".to_string();
        self.client = Some(client);
    }
    
    /// Add a console message and ensure auto-scroll to show latest content
    fn add_console_message(&mut self, message: ConsoleMessage) {
        self.console_messages.push(message);
        // Set scroll to a large value to ensure we show the bottom
        // The rendering widget will clamp this to the actual content height
        self.console_scroll = 9999;
    }

    /// Handle an event and return whether to continue running
    pub async fn handle_event(&mut self, event: Event) -> crate::tui::Result<bool> {
        match event {
            Event::Key(key) => self.handle_key(key).await?,
            Event::Mouse(mouse) => self.handle_mouse(mouse)?,
            Event::Resize(_w, _h) => {
                // Terminal was resized - ratatui handles this automatically
            }
            Event::Tick => {
                // Regular update tick
            }
        }

        Ok(self.running)
    }

    /// Handle mouse input
    fn handle_mouse(&mut self, mouse: crossterm::event::MouseEvent) -> crate::tui::Result<()> {
        use crossterm::event::{MouseButton, MouseEventKind};

        // Handle power device clicks in header (row 0)
        if mouse.row == 0 && mouse.kind == MouseEventKind::Down(MouseButton::Left) {
            for (area, device_name) in &self.power_device_click_areas {
                if mouse.column >= area.x 
                    && mouse.column < area.x + area.width
                    && mouse.row >= area.y
                    && mouse.row < area.y + area.height
                {
                    // Toggle the power device
                    let device_name = device_name.clone();
                    self.pending_commands.push(format!("__TOGGLE_POWER__{}", device_name));
                    return Ok(());
                }
            }
        }

        // Handle mouse scroll for Jobs tab
        if self.current_tab == Tab::Jobs {
            match mouse.kind {
                MouseEventKind::ScrollUp => {
                    self.job_list.previous();
                    return Ok(());
                }
                MouseEventKind::ScrollDown => {
                    self.job_list.next();
                    return Ok(());
                }
                MouseEventKind::Down(MouseButton::Left) => {
                    // Check if click is in the jobs list area
                    let terminal_size = crossterm::terminal::size().unwrap_or((80, 24));
                    let footer_row = terminal_size.1.saturating_sub(1);
                    
                    // Calculate jobs list area (accounting for header, temp bar, position bar, and footer)
                    let mut top_offset = 1; // header
                    if self.panel_visibility.temperature {
                        top_offset += 1;
                    }
                    if self.panel_visibility.position {
                        top_offset += 1;
                    }
                    
                    // Jobs list is between top_offset and footer_row
                    if mouse.row >= top_offset && mouse.row < footer_row {
                        // Calculate which item was clicked
                        let list_start_row = top_offset + 1; // +1 for border
                        let clicked_index = (mouse.row - list_start_row) as usize;
                        
                        // Get the current scroll offset from the list state
                        let current_offset = self.job_list.state.offset();
                        let actual_index = current_offset + clicked_index;
                        
                        // Select the clicked item if it's valid
                        if actual_index < self.job_list.items.len() {
                            self.job_list.state.select(Some(actual_index));
                        }
                        
                        return Ok(());
                    }
                }
                _ => {}
            }
        }

        // Only handle left mouse button clicks for other interactions
        if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
            // Get terminal size to determine footer position
            let terminal_size = crossterm::terminal::size().unwrap_or((80, 24));
            let footer_row = terminal_size.1.saturating_sub(1); // Footer is at last row
            
            // Check if click is on the footer row
            if mouse.row == footer_row {
                // Get all clickable footer elements
                let footer_bounds = crate::tui::widgets::footer::get_footer_bounds(
                    ratatui::layout::Rect {
                        x: 0,
                        y: footer_row,
                        width: terminal_size.0,
                        height: 1,
                    },
                    self.current_tab,
                );
                
                for (action, bounds) in footer_bounds {
                    // Check if click is within this element's horizontal bounds
                    if mouse.column >= bounds.x && mouse.column < bounds.x + bounds.width {
                        match action {
                            crate::tui::widgets::footer::FooterAction::Tab(tab) => {
                                self.current_tab = tab;
                                
                                // If switching to Jobs tab, fetch jobs
                                if tab == Tab::Jobs && self.client.is_some() {
                                    self.pending_commands.push("__FETCH_JOBS__".to_string());
                                }
                            }
                            crate::tui::widgets::footer::FooterAction::Escape => {
                                // Handle escape - go back to Console (default tab)
                                if self.current_tab == Tab::Help {
                                    self.current_tab = Tab::Console;
                                }
                            }
                            crate::tui::widgets::footer::FooterAction::Quit => {
                                // Quit the application
                                self.running = false;
                            }
                        }
                        
                        break;
                    }
                }
            } else if self.current_tab == Tab::Console {
                // Check if click is in the console input area
                // The input box is at the bottom of the console area (last 3 lines before footer)
                // Console input is 3 lines tall and ends just before the footer
                let console_input_start = footer_row.saturating_sub(3);
                let console_input_end = footer_row;
                
                if mouse.row >= console_input_start && mouse.row < console_input_end {
                    // Click is in the input area - enter editing mode
                    self.console_input.mode = InputMode::Editing;
                }
            }
            
            // Check if click is on a temperature setpoint or fan
            // Temperature bar is at row 1 (after header at row 0) when visible
            if self.panel_visibility.temperature && mouse.row == 1 {
                let temp_bounds = crate::tui::widgets::temperatures::get_temp_bounds(
                    ratatui::layout::Rect {
                        x: 0,
                        y: 1,
                        width: terminal_size.0,
                        height: 1,
                    },
                    self
                );
                
                for (element, bounds) in temp_bounds {
                    if mouse.column >= bounds.x && mouse.column < bounds.x + bounds.width {
                        // Clear any other editing states first
                        self.pos_edit_target = None;
                        self.pos_input.exit_edit_mode();
                        
                        match element {
                            crate::tui::widgets::temperatures::TempBarElement::Extruder => {
                                // Clear other temp/fan editing states
                                self.fan_edit_target = None;
                                self.fan_input.exit_edit_mode();
                                
                                // Start editing extruder temperature
                                self.temp_edit_target = Some(TempEditTarget::Extruder);
                                self.temp_input.value.clear();
                                self.temp_input.mode = InputMode::Editing;
                                self.temp_input.cursor_position = 0;
                            }
                            crate::tui::widgets::temperatures::TempBarElement::Bed => {
                                // Clear other temp/fan editing states
                                self.fan_edit_target = None;
                                self.fan_input.exit_edit_mode();
                                
                                // Start editing bed temperature
                                self.temp_edit_target = Some(TempEditTarget::Bed);
                                self.temp_input.value.clear();
                                self.temp_input.mode = InputMode::Editing;
                                self.temp_input.cursor_position = 0;
                            }
                            crate::tui::widgets::temperatures::TempBarElement::Fan(index) => {
                                // Clear other temp editing states
                                self.temp_edit_target = None;
                                self.temp_input.exit_edit_mode();
                                
                                // Start editing fan speed
                                self.fan_edit_target = Some(FanEditTarget { index });
                                self.fan_input.value.clear();
                                self.fan_input.mode = InputMode::Editing;
                                self.fan_input.cursor_position = 0;
                            }
                        }
                        break;
                    }
                }
            }
            
            // Check if click is on position bar (row 2 if both temp and position bars visible)
            let position_bar_row = if self.panel_visibility.temperature { 2 } else { 1 };
            if self.panel_visibility.position && mouse.row == position_bar_row {
                let pos_bounds = crate::tui::widgets::position_bar::get_position_bounds(
                    ratatui::layout::Rect {
                        x: 0,
                        y: position_bar_row,
                        width: terminal_size.0,
                        height: 1,
                    },
                    self
                );
                
                for (action, bounds) in pos_bounds {
                    if mouse.column >= bounds.x && mouse.column < bounds.x + bounds.width {
                        match action {
                            crate::tui::widgets::position_bar::PositionAction::EditAxis(axis) => {
                                // Clear any temp/fan editing states first
                                self.temp_edit_target = None;
                                self.temp_input.exit_edit_mode();
                                self.fan_edit_target = None;
                                self.fan_input.exit_edit_mode();
                                
                                // Start editing this axis
                                self.pos_edit_target = Some(axis);
                                self.pos_input.value.clear();
                                self.pos_input.mode = InputMode::Editing;
                                self.pos_input.cursor_position = 0;
                            }
                            crate::tui::widgets::position_bar::PositionAction::HomeAll => {
                                // Send home all command
                                self.pending_commands.push("G28".to_string());
                                self.console_messages.push(ConsoleMessage::Info(
                                    "🏠 Homing all axes...".to_string()
                                ));
                            }
                        }
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    /// Handle keyboard input
    async fn handle_key(&mut self, key: KeyEvent) -> crate::tui::Result<()> {
        // Handle Ctrl+C for emergency stop/quit
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            if self.printer.connected {
                // Send emergency stop
                self.console_messages.push(ConsoleMessage::Error(
                    "🚨 EMERGENCY STOP TRIGGERED 🚨".to_string()
                ));
                if let Some(client) = &mut self.client {
                    let _ = client.send_gcode("M112").await;
                }
            }
            self.running = false;
            return Ok(());
        }

        // Handle Esc key
        if key.code == KeyCode::Esc {
            if self.console_input.mode == InputMode::Editing {
                self.console_input.exit_edit_mode();
            } else if self.temp_input.mode == InputMode::Editing {
                // Cancel temperature editing
                self.temp_input.exit_edit_mode();
                self.temp_edit_target = None;
            } else if self.fan_input.mode == InputMode::Editing {
                // Cancel fan editing
                self.fan_input.exit_edit_mode();
                self.fan_edit_target = None;
            } else if self.pos_input.mode == InputMode::Editing {
                // Cancel position editing
                self.pos_input.exit_edit_mode();
                self.pos_edit_target = None;
            } else {
                // TODO: Close modals or return to previous view
            }
            return Ok(());
        }

        // Handle position editing mode
        if self.pos_input.mode == InputMode::Editing {
            self.handle_pos_input(key).await?;
            return Ok(());
        }

        // Handle temperature editing mode
        if self.temp_input.mode == InputMode::Editing {
            self.handle_temp_input(key).await?;
            return Ok(());
        }

        // Handle fan editing mode
        if self.fan_input.mode == InputMode::Editing {
            self.handle_fan_input(key).await?;
            return Ok(());
        }

        // Handle input mode for console
        if self.current_tab == Tab::Console && self.console_input.mode == InputMode::Editing {
            self.handle_console_input(key)?;
            return Ok(());
        }

        // Handle Jobs tab navigation
        if self.current_tab == Tab::Jobs {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    self.job_list.previous();
                    return Ok(());
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    self.job_list.next();
                    return Ok(());
                }
                KeyCode::Char('r') => {
                    // Refresh job list
                    if self.client.is_some() {
                        self.pending_commands.push("__FETCH_JOBS__".to_string());
                        self.console_messages.push(ConsoleMessage::Info(
                            "Refreshing job list...".to_string()
                        ));
                    }
                    return Ok(());
                }
                KeyCode::Enter => {
                    // Start selected job
                    if let Some(selected) = self.job_list.selected() {
                        let filename = selected.filename.clone();
                        // Add message and use GCode command to start print
                        self.console_messages.push(ConsoleMessage::Info(
                            format!("Starting print: {}", filename)
                        ));
                        // Use M23/M24 to start the print via GCode
                        self.pending_commands.push(format!("SDCARD_PRINT_FILE FILENAME=\"{}\"", filename));
                    }
                    return Ok(());
                }
                _ => {}
            }
        }

        // Handle normal mode keys
        if let KeyCode::Char(c) = key.code {
            // Tab switching
            if let Some(tab) = Tab::from_key(c) {
                let old_tab = self.current_tab;
                self.current_tab = tab;
                
                // Fetch jobs when switching to Jobs tab
                if self.current_tab == Tab::Jobs && old_tab != Tab::Jobs
                    && self.client.is_some() {
                        // Queue job fetch
                        self.pending_commands.push("__FETCH_JOBS__".to_string());
                    }
                
                return Ok(());
            }

            // Panel toggles (work from all tabs)
            match c {
                't' => {
                    self.panel_visibility.temperature = !self.panel_visibility.temperature;
                }
                'l' => {
                    self.panel_visibility.position = !self.panel_visibility.position;
                }
                'i' => {
                    // Enter edit mode on console tab
                    if self.current_tab == Tab::Console {
                        self.console_input.mode = InputMode::Editing;
                    }
                }
                // Homing commands on position tab
                'x' if self.current_tab == Tab::Position => {
                    if self.client.is_some() {
                        self.pending_commands.push("G28 X".to_string());
                        self.console_messages.push(ConsoleMessage::Info(
                            "Homing X axis...".to_string()
                        ));
                    }
                }
                'y' if self.current_tab == Tab::Position => {
                    if self.client.is_some() {
                        self.pending_commands.push("G28 Y".to_string());
                        self.console_messages.push(ConsoleMessage::Info(
                            "Homing Y axis...".to_string()
                        ));
                    }
                }
                'z' if self.current_tab == Tab::Position => {
                    if self.client.is_some() {
                        self.pending_commands.push("G28 Z".to_string());
                        self.console_messages.push(ConsoleMessage::Info(
                            "Homing Z axis...".to_string()
                        ));
                    }
                }
                'a' if self.current_tab == Tab::Position => {
                    if self.client.is_some() {
                        self.pending_commands.push("G28".to_string());
                        self.console_messages.push(ConsoleMessage::Info(
                            "Homing all axes...".to_string()
                        ));
                    }
                }
                'q' => {
                    self.running = false;
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Handle console input in editing mode
    fn handle_console_input(&mut self, key: KeyEvent) -> crate::tui::Result<()> {
        match key.code {
            KeyCode::Enter => {
                let command = self.console_input.value.trim().to_string();
                if !command.is_empty() {
                    // Add command to history
                    self.console_messages.push(ConsoleMessage::Command(command.clone()));
                    
                    // Add to command history (avoid duplicates at end)
                    if self.command_history.last() != Some(&command) {
                        self.command_history.push(command.clone());
                    }
                    // Keep history to a reasonable size
                    if self.command_history.len() > 100 {
                        self.command_history.remove(0);
                    }
                    
                    // Queue command for sending
                    if self.client.is_some() {
                        self.pending_commands.push(command);
                    } else {
                        self.console_messages.push(ConsoleMessage::Error(
                            "Not connected to printer".to_string()
                        ));
                    }
                    
                    self.console_input.clear();
                    self.history_index = None; // Reset history browsing
                }
            }
            KeyCode::Up => {
                // Navigate backwards through history
                if !self.command_history.is_empty() {
                    if let Some(idx) = self.history_index {
                        if idx > 0 {
                            self.history_index = Some(idx - 1);
                            self.console_input.value = self.command_history[idx - 1].clone();
                            self.console_input.cursor_position = self.console_input.value.len() as u16;
                        }
                    } else {
                        // Start at the end of history
                        let idx = self.command_history.len() - 1;
                        self.history_index = Some(idx);
                        self.console_input.value = self.command_history[idx].clone();
                        self.console_input.cursor_position = self.console_input.value.len() as u16;
                    }
                }
            }
            KeyCode::Down => {
                // Navigate forwards through history
                if let Some(idx) = self.history_index {
                    if idx < self.command_history.len() - 1 {
                        self.history_index = Some(idx + 1);
                        self.console_input.value = self.command_history[idx + 1].clone();
                        self.console_input.cursor_position = self.console_input.value.len() as u16;
                    } else {
                        // At the end, clear input
                        self.history_index = None;
                        self.console_input.clear();
                    }
                }
            }
            KeyCode::Char(c) => {
                self.console_input.value.push(c);
                self.console_input.cursor_position += 1;
                self.history_index = None; // Stop browsing history when typing
            }
            KeyCode::Backspace => {
                if !self.console_input.value.is_empty() {
                    self.console_input.value.pop();
                    self.console_input.cursor_position = 
                        self.console_input.cursor_position.saturating_sub(1);
                    self.history_index = None; // Stop browsing history when editing
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Handle temperature input
    async fn handle_temp_input(&mut self, key: KeyEvent) -> crate::tui::Result<()> {
        match key.code {
            KeyCode::Enter => {
                // Parse and set temperature
                if let Ok(temp) = self.temp_input.value.trim().parse::<f64>() {
                    if (0.0..=300.0).contains(&temp) {
                        let gcode = match self.temp_edit_target {
                            Some(TempEditTarget::Extruder) => format!("M104 S{}", temp as u32),
                            Some(TempEditTarget::Bed) => format!("M140 S{}", temp as u32),
                            None => String::new(),
                        };
                        
                        if !gcode.is_empty() {
                            self.add_console_message(ConsoleMessage::Command(gcode.clone()));
                            
                            if self.client.is_some() {
                                // Optimistically update local state for immediate UI feedback
                                match self.temp_edit_target {
                                    Some(TempEditTarget::Extruder) => {
                                        self.printer.temperatures.extruder.target = temp;
                                    }
                                    Some(TempEditTarget::Bed) => {
                                        self.printer.temperatures.bed.target = temp;
                                    }
                                    None => {}
                                }
                                
                                self.pending_commands.push(gcode);
                                let heater_name = match self.temp_edit_target {
                                    Some(TempEditTarget::Extruder) => "Extruder",
                                    Some(TempEditTarget::Bed) => "Bed",
                                    None => "",
                                };
                                self.add_console_message(ConsoleMessage::Info(
                                    format!("{} target set to {}°C", heater_name, temp as u32)
                                ));
                            } else {
                                self.add_console_message(ConsoleMessage::Error(
                                    "Not connected to printer".to_string()
                                ));
                            }
                        }
                    } else {
                        self.add_console_message(ConsoleMessage::Error(
                            "Temperature must be between 0 and 300°C".to_string()
                        ));
                    }
                } else {
                    self.add_console_message(ConsoleMessage::Error(
                        "Invalid temperature value".to_string()
                    ));
                }
                
                // Exit editing mode
                self.temp_input.exit_edit_mode();
                self.temp_edit_target = None;
            }
            KeyCode::Char(c) if c.is_ascii_digit() => {
                self.temp_input.value.push(c);
                self.temp_input.cursor_position += 1;
            }
            KeyCode::Backspace => {
                if !self.temp_input.value.is_empty() {
                    self.temp_input.value.pop();
                    self.temp_input.cursor_position = 
                        self.temp_input.cursor_position.saturating_sub(1);
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Handle fan speed input
    async fn handle_fan_input(&mut self, key: KeyEvent) -> crate::tui::Result<()> {
        match key.code {
            KeyCode::Enter => {
                // Parse and set fan speed
                if let Ok(speed_percent) = self.fan_input.value.trim().parse::<u8>() {
                    if speed_percent <= 100 {
                        let speed_value = (speed_percent as f64) / 100.0;
                        
                        // Determine which fan to control
                        if let Some(target) = self.fan_edit_target
                            && let Some(fan) = self.printer.temperatures.fans.get_mut(target.index) {
                                // Use M106 for part fan (fan), or SET_FAN_SPEED for named fans
                                let gcode = if fan.name == "Part" {
                                    format!("M106 S{}", (speed_value * 255.0) as u8)
                                } else {
                                    format!("SET_FAN_SPEED FAN={} SPEED={:.2}", fan.name, speed_value)
                                };
                                
                                self.console_messages.push(ConsoleMessage::Command(gcode.clone()));
                                
                                if self.client.is_some() {
                                    // Update local state immediately for responsive UI
                                    fan.speed = speed_value;
                                    
                                    self.pending_commands.push(gcode);
                                    self.console_messages.push(ConsoleMessage::Info(
                                        format!("{} fan speed set to {}%", fan.name, speed_percent)
                                    ));
                                } else {
                                    self.console_messages.push(ConsoleMessage::Error(
                                        "Not connected to printer".to_string()
                                    ));
                                }
                            }
                    } else {
                        self.console_messages.push(ConsoleMessage::Error(
                            "Fan speed must be between 0 and 100%".to_string()
                        ));
                    }
                } else {
                    self.console_messages.push(ConsoleMessage::Error(
                        "Invalid fan speed value".to_string()
                    ));
                }
                
                // Exit editing mode
                self.fan_input.exit_edit_mode();
                self.fan_edit_target = None;
            }
            KeyCode::Char(c) if c.is_ascii_digit() => {
                // Only allow up to 3 digits (max 100)
                if self.fan_input.value.len() < 3 {
                    self.fan_input.value.push(c);
                    self.fan_input.cursor_position += 1;
                }
            }
            KeyCode::Backspace => {
                if !self.fan_input.value.is_empty() {
                    self.fan_input.value.pop();
                    self.fan_input.cursor_position = 
                        self.fan_input.cursor_position.saturating_sub(1);
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Handle position input
    async fn handle_pos_input(&mut self, key: KeyEvent) -> crate::tui::Result<()> {
        match key.code {
            KeyCode::Enter => {
                // Parse and move to position
                if let Ok(pos) = self.pos_input.value.trim().parse::<f64>() {
                    // Determine valid range based on axis
                    let (min, max, axis_letter) = match self.pos_edit_target {
                        Some(PosEditTarget::X) => (0.0, 400.0, "X"),
                        Some(PosEditTarget::Y) => (0.0, 400.0, "Y"),
                        Some(PosEditTarget::Z) => (0.0, 400.0, "Z"),
                        None => (0.0, 0.0, ""),
                    };
                    
                    if pos >= min && pos <= max && !axis_letter.is_empty() {
                        let gcode = format!("G0 {}{:.2} F3000", axis_letter, pos);
                        
                        self.console_messages.push(ConsoleMessage::Command(gcode.clone()));
                        
                        if self.client.is_some() {
                            self.pending_commands.push(gcode);
                            self.console_messages.push(ConsoleMessage::Info(
                                format!("Moving {} to {:.2}mm", axis_letter, pos)
                            ));
                        } else {
                            self.console_messages.push(ConsoleMessage::Error(
                                "Not connected to printer".to_string()
                            ));
                        }
                    } else {
                        self.console_messages.push(ConsoleMessage::Error(
                            format!("Position must be between {:.0} and {:.0}mm", min, max)
                        ));
                    }
                } else {
                    self.console_messages.push(ConsoleMessage::Error(
                        "Invalid position value".to_string()
                    ));
                }
                
                // Exit editing mode
                self.pos_input.exit_edit_mode();
                self.pos_edit_target = None;
            }
            KeyCode::Char(c) if c.is_ascii_digit() || c == '.' => {
                // Allow digits and one decimal point
                if c == '.' && self.pos_input.value.contains('.') {
                    // Already has a decimal point
                } else {
                    self.pos_input.value.push(c);
                    self.pos_input.cursor_position += 1;
                }
            }
            KeyCode::Char('-') if self.pos_input.value.is_empty() => {
                // Allow negative sign only at the start
                self.pos_input.value.push('-');
                self.pos_input.cursor_position += 1;
            }
            KeyCode::Backspace => {
                if !self.pos_input.value.is_empty() {
                    self.pos_input.value.pop();
                    self.pos_input.cursor_position = 
                        self.pos_input.cursor_position.saturating_sub(1);
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Update application state (called each tick)
    pub async fn update(&mut self) -> crate::tui::Result<()> {
        // Handle special internal commands first (before borrowing client)
        let mut i = 0;
        while i < self.pending_commands.len() {
            if self.pending_commands[i] == "__FETCH_JOBS__" {
                let cmd = self.pending_commands.remove(i);
                drop(cmd); // Drop to avoid unused variable warning
                if let Err(e) = self.fetch_job_history().await {
                    self.console_messages.push(ConsoleMessage::Error(
                        format!("Error fetching jobs: {}", e)
                    ));
                }
            } else if self.pending_commands[i].starts_with("__TOGGLE_POWER__") {
                let cmd = self.pending_commands.remove(i);
                let device_name = cmd.strip_prefix("__TOGGLE_POWER__").unwrap_or("");
                if let Err(e) = self.toggle_power_device(device_name).await {
                    self.console_messages.push(ConsoleMessage::Error(
                        format!("Error toggling power device: {}", e)
                    ));
                }
            } else {
                i += 1;
            }
        }
        
        // Send any remaining pending commands
        if let Some(client) = &mut self.client {
            while let Some(command) = self.pending_commands.pop() {
                match client.send_gcode(&command).await {
                    Ok(_) => {
                        // Command sent successfully - response will be shown via notify_gcode_response
                    }
                    Err(e) => {
                        self.console_messages.push(ConsoleMessage::Error(
                            format!("Error sending command: {}", e)
                        ));
                    }
                }
            }
            
            // Try to receive any pending messages
            if let Some(message) = client.try_receive_message()
                && let Err(e) = self.process_message(&message) {
                    eprintln!("Error processing message: {}", e);
                }
        }
        Ok(())
    }

    /// Process a message from the WebSocket
    fn process_message(&mut self, message: &str) -> anyhow::Result<()> {
        let value: serde_json::Value = serde_json::from_str(message)?;
        
        // Handle objects.list response to subscribe to discovered sensors and fans
        if let Some(result) = value.get("result")
            && let Some(objects) = result.get("objects").and_then(|o| o.as_array()) {
                // This is an objects.list response
                let object_names: Vec<String> = objects
                    .iter()
                    .filter_map(|o| o.as_str())
                    .map(|s| s.to_string())
                    .collect();
                
                // Queue up subscription to additional objects
                // We'll do this synchronously in the update loop
                if let Some(client) = &mut self.client {
                    // Subscribe to temperature sensors and fans
                    let subscribe_objects: Vec<String> = object_names
                        .iter()
                        .filter(|name| {
                            name.starts_with("temperature_sensor ") ||
                            name.starts_with("temperature_fan ") ||
                            name.starts_with("heater_fan ") ||
                            name.starts_with("controller_fan ")
                        })
                        .cloned()
                        .collect();
                    
                    if !subscribe_objects.is_empty() {
                        // Use blocking runtime since we're in sync context
                        let _ = tokio::task::block_in_place(|| {
                            tokio::runtime::Handle::current().block_on(async {
                                client.subscribe_to_additional_objects(subscribe_objects).await
                            })
                        });
                    }
                }
            }
        
        // Update printer state from status updates
        super::printer::update_from_json(&mut self.printer, &value);
        
        // Handle GCode responses for console
        if let Some(method) = value.get("method").and_then(|m| m.as_str()) {
            if method == "notify_gcode_response"
                && let Some(params) = value.get("params").and_then(|p| p.get(0))
                && let Some(msg) = params.as_str() {
                    // Add to console messages
                    if msg.contains("error") || msg.contains("!!") {
                        self.add_console_message(ConsoleMessage::Error(msg.to_string()));
                    } else {
                        self.add_console_message(ConsoleMessage::Response(msg.to_string()));
                    }
                }
        } else if let Some(error) = value.get("error") {
            // Handle error responses
            let error_msg = error.get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown error");
            self.add_console_message(ConsoleMessage::Error(error_msg.to_string()));
        }
        
        Ok(())
    }
    
    /// Fetch print jobs from Moonraker files list
    pub async fn fetch_job_history(&mut self) -> crate::tui::Result<()> {
        let url = format!("{}/server/files/list?root=gcodes", self.http_url);
        
        match self.http_client.get(&url).send().await {
            Ok(response) => {
                if let Ok(json) = response.json::<serde_json::Value>().await
                    && let Some(files) = json.get("result").and_then(|r| r.as_array()) {
                        let mut job_items = Vec::new();
                        
                        for file in files {
                            // Only include gcode files
                            let filename = file.get("filename")
                                .or_else(|| file.get("path"))
                                .and_then(|v| v.as_str())
                                .unwrap_or("unknown")
                                .to_string();
                            
                            if !filename.to_lowercase().ends_with(".gcode") {
                                continue;
                            }
                            
                            let print_job = PrintJob {
                                job_id: String::new(), // Not applicable for file list
                                filename,
                                status: String::new(), // Not applicable for file list
                                start_time: 0.0,
                                end_time: 0.0,
                                total_duration: file.get("estimated_time")
                                    .and_then(|v| v.as_f64())
                                    .unwrap_or(0.0),
                                filament_used: file.get("filament_total")
                                    .and_then(|v| v.as_f64())
                                    .unwrap_or(0.0),
                                print_duration: 0.0,
                            };
                            job_items.push(print_job);
                        }
                        
                        self.job_list = StatefulList::with_items(job_items);
                        if !self.job_list.items.is_empty() {
                            self.job_list.state.select(Some(0));
                        }
                    }
            }
            Err(e) => {
                self.console_messages.push(ConsoleMessage::Error(
                    format!("Failed to fetch print jobs: {}", e)
                ));
            }
        }
        
        Ok(())
    }
    
    /// Start a print job
    #[allow(dead_code)]
    pub async fn start_print_job(&mut self, filename: &str) -> crate::tui::Result<()> {
        let url = format!("{}/printer/print/start", self.http_url);
        let body = serde_json::json!({
            "filename": filename
        });
        
        self.console_messages.push(ConsoleMessage::Info(
            format!("Starting print: {}", filename)
        ));
        
        match self.http_client.post(&url).json(&body).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    self.console_messages.push(ConsoleMessage::Info(
                        "Print started successfully!".to_string()
                    ));
                } else {
                    self.console_messages.push(ConsoleMessage::Error(
                        format!("Failed to start print: {}", response.status())
                    ));
                }
            }
            Err(e) => {
                self.console_messages.push(ConsoleMessage::Error(
                    format!("Failed to start print: {}", e)
                ));
            }
        }
        
        Ok(())
    }
    
    /// Fetch power devices from Moonraker
    pub async fn fetch_power_devices(&mut self) -> crate::tui::Result<()> {
        let url = format!("{}/machine/device_power/devices", self.http_url);
        
        match self.http_client.get(&url).send().await {
            Ok(response) => {
                if let Ok(json) = response.json::<serde_json::Value>().await
                    && let Some(result) = json.get("result")
                    && let Some(devices) = result.get("devices").and_then(|d| d.as_array())
                {
                    let mut power_devices = Vec::new();
                    
                    for device in devices {
                        let name = device.get("device")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown")
                            .to_string();
                        
                        let status = device.get("status")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown")
                            .to_string();
                        
                        let device_type = device.get("type")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown")
                            .to_string();
                        
                        let locked_while_printing = device.get("locked_while_printing")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                        
                        power_devices.push(crate::tui::printer::PowerDevice {
                            name,
                            status,
                            device_type,
                            locked_while_printing,
                        });
                    }
                    
                    self.printer.power_devices = power_devices;
                }
            }
            Err(e) => {
                // Silently fail - not all printers have power devices configured
                eprintln!("Failed to fetch power devices: {}", e);
            }
        }
        
        Ok(())
    }
    
    /// Toggle a power device on/off
    pub async fn toggle_power_device(&mut self, device_name: &str) -> crate::tui::Result<()> {
        // Check if device is locked while printing
        if let Some(device) = self.printer.power_devices.iter().find(|d| d.name == device_name)
            && device.locked_while_printing && self.printer.print_stats.state == "printing"
        {
            self.console_messages.push(ConsoleMessage::Error(
                format!("Device '{}' is locked while printing", device_name)
            ));
            return Ok(());
        }
        
        let url = format!("{}/machine/device_power/device", self.http_url);
        let body = serde_json::json!({
            "device": device_name,
            "action": "toggle"
        });
        
        match self.http_client.post(&url).json(&body).send().await {
            Ok(response) => {
                if let Ok(json) = response.json::<serde_json::Value>().await
                    && let Some(result) = json.get("result")
                    && let Some(new_status) = result.get(device_name).and_then(|v| v.as_str())
                {
                    // Update the device status in our state
                    if let Some(device) = self.printer.power_devices.iter_mut().find(|d| d.name == device_name) {
                        device.status = new_status.to_string();
                    }
                }
            }
            Err(e) => {
                self.console_messages.push(ConsoleMessage::Error(
                    format!("Failed to toggle power device: {}", e)
                ));
            }
        }
        
        Ok(())
    }
}
