# Moonriver v0.2.0 Roadmap

## Vision
Transform moonriver from a simple REPL into a full-featured TUI dashboard with real-time monitoring, inspired by btm and krui.

## Inspiration Sources

### btm (bottom) - ClementTsang/bottom
- Mouse interaction patterns with `on_left_mouse_up(x, y)`
- Layout management with constraints
- Event handling architecture
- Widget system with borders/tables/graphs

### krui - jfoucher/krui ⭐ **Excellent Reference!**
**Architecture Insights:**
- **Tab system**: Semantic letter keys (`m`/`c`/`p`/`j`/`h`) instead of F-keys
- **Header bar**: Connection status (✔/✕), printer state, stepper/filament indicators, system load
- **Footer bar**: Shows available key bindings for current context
- **Modal dialogs**: Temperature input, print confirmation (OK/Cancel pattern)
- **StatefulList pattern**: For navigable lists with state management
- **Panel toggles**: Quick keyboard shortcuts to show/hide panels

**Widget Patterns:**
- **Temperature widget**: 
  - Visual power bars `[||||||||]` showing heater power 0-100%
  - Color gradients: green (0-30%) → yellow (30-60%) → red (60-100%)
  - Shows current temp, target temp, and power percentage
  - Distinguishes heaters (magenta) vs temperature_fans (red)
  - **Togglable**: Press `t` to show/hide
  
- **Toolhead panel**:
  - Position table: X/Y/Z coordinates with mm precision
  - Homed status (true/false) for each axis
  - Individual "Home" buttons for each axis
  - Quad gantry leveling (QGL) support
  - **Dedicated tab**: Press `p` to focus

- **Print status**:
  - Progress percentage and layer info (current/total)
  - Speed, filament used, flow rate
  - Time estimates (slicer, file, total, ETA)
  - Embedded gcode preview image using viuer
  - **Part of main tab**: Press `m` to view

- **Job history**:
  - Sortable list with filename, duration, filament
  - Status indicators: ✔ (success) / ✕ (failed/cancelled)
  - Click to reprint from history
  - **Dedicated tab**: Press `j` to browse full history
  
- **Console tab**:
  - Scrollable GCode history with timestamps (HH:MM)
  - Color coding: `!!` prefix = red/bold errors
  - Input field with Normal/Editing modes
  - Tab key toggles between scroll and input
  - **Dedicated tab**: Press `c` to open

- **Special features**:
  - Ctrl+C emergency stop / firmware restart (instead of F10)
  - Webcam feed in terminal (via viuer crate) - optional
  - Scrollbar widgets with position indicators
  - Input state management (cursor position tracking)

## Core Features

### 1. TUI Framework Integration
- [x] Add ratatui (formerly tui-rs) - modern terminal UI framework
- [x] Add crossterm for terminal backend (already have compatibility via colored)
- [ ] Mouse event support for clicks and scrolls
- [ ] Keyboard navigation enhancement
- [ ] Responsive layout system

### 2. Status Bars & Panels

#### Panel Visibility & Layout
- [ ] All panels are togglable on/off
- [ ] Persist panel visibility preferences in config
- [ ] Keyboard shortcuts for quick toggle (t, s, 1-9)
- [ ] Click panel headers to collapse/expand
- [ ] Smooth resize animations (optional)
- [ ] Minimum/maximum panel sizes
- [ ] Remember panel sizes between sessions

#### Temperature/Thermal Panel (inspired by krui)
- [ ] List all heaters from Moonraker's `available_sensors`
- [ ] Each heater shows: name, current temp, target temp
- [ ] Visual power bars `[||||||||]` showing 0-100% power
- [ ] Color gradients: green (0-30%) → yellow-orange (30-60%) → red (60-100%)
- [ ] Distinguish heater types: heaters (magenta) vs temperature_fans (red)
- [ ] Click to set target temperature (modal dialog)
- [ ] Support for: extruder, bed, chamber, MCU, temperature_fans
- [ ] Real-time updates via WebSocket subscription

#### Machine Position Panel (inspired by krui toolhead)
- [ ] X, Y, Z coordinates with mm precision (X: 2 decimals, Z: 3 decimals)
- [ ] Homing status indicators (true/false) for each axis
- [ ] Relative vs absolute positioning mode indicator
- [ ] Quick home buttons: X, Y, Z, All axes
- [ ] Quad gantry leveling (QGL) button if available
- [ ] Speed/feedrate display
- [ ] Click to quick-navigate/jog

#### System Utilization Panel (from krui header)
- [ ] System load average (color-coded: green<0.3, yellow<0.6, red≥0.6)
- [ ] Connection status indicator (✔ connected / ✕ disconnected)
- [ ] Printer state badge (standby/ready/printing/paused/error)
- [ ] Stepper motor status indicator
- [ ] Filament switch sensor status
- [ ] Memory/disk usage
- [ ] Network status

#### Print Jobs Panel (from krui history)
- [ ] Scrollable list of recent prints
- [ ] Each item shows:
  - Filename
  - Print duration (HH:MM:SS format)
  - Filament used (mm)
  - End status: ✔ (success) / ✕ (failed/cancelled)
  - End time/date
- [ ] Sort by most recent first
- [ ] Click to select, Enter to confirm reprint
- [ ] Modal confirmation dialog before reprinting
- [ ] Search/filter jobs

#### Active Print Status (from krui printing view)
- [ ] Show when print is active instead of history
- [ ] Filename and progress percentage
- [ ] Layer info: current layer / total layers
- [ ] Speed (mm/s), flow rate (mm³/s)
- [ ] Filament used (mm)
- [ ] Time displays:
  - Print duration (elapsed)
  - Slicer estimate (remaining)
  - File estimate (remaining)
  - ETA (actual time)
- [ ] Progress bar or gauge
- [ ] Optional: G-code preview image (via viuer crate)

### 3. Interactive Elements
- [ ] Mouse click support throughout (using crossterm mouse events)
- [ ] Keyboard shortcuts (semantic letter keys):
  - **Tab switching:**
    - `m`: Main dashboard (temperatures + history/print status)
    - `c`: Console (GCode terminal)
    - `p`: Position (toolhead coordinates and homing)
    - `j`: Jobs (full-screen job history browser)
    - `h` or `?`: Help (context-sensitive)
  - **Global actions:**
    - `q`: Quit application
    - `Ctrl+C`: Emergency stop / firmware restart
    - `Esc`: Cancel/close dialogs or return to previous view
  - **Panel/Widget toggles:**
    - `t`: Toggle temperature panel visibility
    - `s`: Toggle system info panel visibility
    - `1-9`: Quick toggle individual panels
  - **Navigation:**
    - `Tab` / `Shift+Tab`: Cycle focus between panels
    - Arrow keys: Navigate within lists and panels
    - `j`/`k` or Up/Down: Navigate lists (vim-style)
    - `h`/`l` or Left/Right: Navigate horizontally
    - Enter: Activate/confirm selection
    - `g` / `G`: Jump to top/bottom of lists
    - `/`: Search/filter (when in list view)
- [ ] Modal dialog system (temperature input, print confirmation)
- [ ] Input modes: Normal vs Editing (rustyline-style)
- [ ] Scrollbar indicators for long lists
- [ ] Focus indicators for selected panels
- [ ] Clickable buttons with visual feedback
- [ ] Visual toggle indicators for panel visibility

### 4. Console/Terminal Integration
- [ ] Integrate existing REPL as a dedicated tab
- [ ] GCode history view with timestamps (HH:MM format)
- [ ] Color-coded messages:
  - Normal: default color
  - Errors (!! prefix): red + bold
  - Warnings: yellow
- [ ] Scrollable output (arrow keys when not editing)
- [ ] Input field for GCode commands
- [ ] Tab key toggles between scroll mode and input mode
- [ ] Command history (up/down arrows in input mode)

## Technical Architecture

### Dependencies
```toml
# Already added in Cargo.toml:
ratatui = "0.29"
crossterm = { version = "0.28", features = ["event-stream"] }
chrono = "0.4"  # For timestamp formatting

# Optional additions:
# viuer = "0.7"  # For G-code preview images (like krui)
```

### Module Structure
```
src/
├── main.rs              # Entry point, mode selection
├── cli.rs               # CLI argument parsing  
├── config.rs            # Configuration management
├── moonraker.rs         # WebSocket client (existing)
├── repl.rs              # REPL mode (existing, to be integrated)
└── tui/                 # NEW: TUI module
    ├── mod.rs           # TUI entry point, main loop
    ├── app.rs           # Application state management (inspired by krui)
    ├── event.rs         # Event handler (keyboard, mouse, WebSocket)
    ├── ui.rs            # Main render function
    ├── tabs.rs          # Tab management (Main/Console/Position/Jobs/etc.)
    ├── modal.rs         # Modal dialog system
    ├── stateful_list.rs # StatefulList helper (from krui)
    └── widgets/         # Widget modules
        ├── mod.rs
        ├── header.rs        # Top status bar with connection/state
        ├── footer.rs        # Bottom bar with context-sensitive key hints
        ├── temperatures.rs  # Temperature panel with power bars (togglable)
        ├── position.rs      # Machine position display
        ├── system_info.rs   # System utilization panel (togglable)
        ├── job_history.rs   # Print job history list
        ├── print_status.rs  # Active print display
        ├── console.rs       # GCode console/REPL
        └── button.rs        # Reusable button widget
```

### Key Data Structures (inspired by krui)

```rust
pub struct App {
    pub running: bool,
    pub printer: Printer,
    pub current_tab: Tab,
    pub selected_widget: Widget,
    pub panel_visibility: PanelVisibility,  // NEW: Track which panels are shown
    pub history: StatefulList<HistoryItem>,
    pub console_scroll: u16,
    pub console_scroll_state: ScrollbarState,
    pub console_input: InputState,
    pub temperature_input: InputState,
    pub selected_heater: Option<Heater>,
    // ... WebSocket connection fields
}

pub enum Tab {
    Main,        // Dashboard with history/temps (press 'm')
    Console,     // GCode console (press 'c')
    Position,    // Toolhead position and homing (press 'p')
    Jobs,        // Full job history browser (press 'j')
    Help,        // Context-sensitive help (press 'h' or '?')
}

pub struct Printer {
    pub connected: bool,
    pub status: PrinterStatus,
    pub toolhead: Toolhead,
    pub current_print: Option<PrintStats>,
    // ...
}

pub struct PrinterStatus {
    pub heaters: StatefulList<Heater>,
    pub state: String,  // standby, ready, printing, etc.
    pub stepper_enable: bool,
    pub filament_switch: bool,
    pub gcodes: Vec<GCodeLine>,
}

pub struct Heater {
    pub name: String,
    pub temperature: f64,
    pub target: f64,
    pub power: f64,  // 0.0-1.0 for power bar visualization
    pub heater_type: HeaterType,  // Heater vs TemperatureFan
}

pub struct PanelVisibility {
    pub temperature: bool,
    pub system_info: bool,
    pub job_history: bool,
    pub position: bool,
    // Add more as needed
}

pub struct InputState {
    pub value: String,
    pub cursor_position: u16,
    pub mode: InputMode,  // Normal vs Editing
}

pub enum InputMode {
    Normal,
    Editing,
}

// StatefulList pattern from krui
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}
```

### Key Bindings Reference

```
Global:
  m       - Main dashboard tab
  c       - Console tab  
  p       - Position tab
  j       - Jobs tab
  h / ?   - Help
  q       - Quit
  Ctrl+C  - Emergency stop / firmware restart

Panel Toggles (Main tab):
  t       - Toggle temperature panel
  s       - Toggle system info panel
  1-9     - Quick toggle numbered panels

Navigation:
  Tab / Shift+Tab  - Cycle panel focus
  ↑↓ / jk         - Navigate lists (vim-style alternative)
  ←→ / hl         - Navigate horizontally
  Enter           - Select/activate
  Esc             - Cancel/back
  g / G           - Jump to top/bottom of list
  /               - Search/filter

Console Tab:
  Tab             - Toggle scroll/input mode
  ↑↓              - Command history (input mode)
  ↑↓              - Scroll (scroll mode)
  Enter           - Send command (input mode)
  
Position Tab:
  x / X           - Home X axis
  y / Y           - Home Y axis
  z / Z           - Home Z axis
  a / A           - Home all axes
  q               - Quad gantry level (if available)
```

### Moonraker API Endpoints

```
# WebSocket subscriptions
printer.objects.subscribe
  - heaters, temperature_sensors, temperature_fans
  - toolhead (position, homed_axes, print_time, etc.)
  - print_stats (state, filename, current_layer, etc.)
  - system_stats (cpu, memory, network)
  - webhooks (state)

# HTTP endpoints
GET /server/info                    # Server version, warnings
GET /server/history/list           # Print job history
GET /printer/objects/query         # One-time query for printer state
POST /printer/gcode/script         # Send GCode commands
POST /printer/print/start          # Start a print
POST /printer/emergency_stop       # Emergency stop (Ctrl+C)
POST /printer/gcode/restart        # Firmware restart
GET /server/webcams/list           # Available webcams (future)
```

## Implementation Phases

## Phase 1: Foundation (Week 1-2)
- [ ] Add ratatui and crossterm dependencies
- [ ] Create basic TUI structure with layout
- [ ] Implement semantic key bindings (m/c/p/j/h)
- [ ] Add panel visibility toggle system
- [ ] Migrate REPL to terminal widget
- [ ] Implement keyboard navigation
- [ ] Add mouse support framework

## Phase 2: Core Widgets (Week 3-4)
- [ ] Temperature panel with real-time updates (togglable with 't')
- [ ] Position display widget (dedicated 'p' tab)
- [ ] System info widget
- [ ] Basic styling and themes

## Phase 3: Advanced Features (Week 5-6)
- [ ] Print job history panel
- [ ] Enhanced terminal widget
- [ ] Interactive elements (clicking, context menus)
- [ ] Polish UI/UX

## Phase 4: Testing & Documentation (Week 7)
- [ ] Integration testing
- [ ] Update documentation
- [ ] Add screenshots/demo GIFs
- [ ] Update README with new features

## Design Principles
1. **Performance First** - No stuttering, smooth updates
2. **Mouse + Keyboard** - Both input methods fully supported
3. **Beautiful by Default** - Great aesthetics out of the box
4. **Backwards Compatible** - Keep scripting mode working
5. **Extensible** - Easy to add new widgets/panels

## Moonraker API Endpoints to Use
- `/printer/objects/query` - Real-time printer state
- `/server/temperature_store` - Temperature history
- `/server/gcode_store` - Command history
- `/server/history/list` - Print job history
- `/printer/info` - Printer metadata
- `/machine/system_info` - Host system info

## Visual Inspiration
- btm: Clean layouts, good use of color, smooth mouse interaction
- krui: Printer-specific widgets, intuitive controls
- htop: Status bars, color coding
- lazygit: Keyboard shortcuts, panel navigation

## Success Criteria
- ✅ Smooth 60fps rendering
- ✅ < 100ms input latency
- ✅ < 50MB memory usage
- ✅ All features accessible via keyboard
- ✅ All features accessible via mouse
- ✅ Works on 80x24 terminal (minimum)
- ✅ Looks great on 4K displays
