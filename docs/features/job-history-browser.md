# Job Browser

The Job Browser allows you to view available G-code files and start new print jobs directly from the TUI interface.

## Features

### Browse Available Files
- View all .gcode files in your printer's gcodes directory
- Scroll through the file list with mouse wheel or arrow keys
- Click on files to select them
- See file metadata including estimated print time and filament usage
- Automatic file list refresh when entering the Jobs tab

### Start Print Jobs
- Select any G-code file from the list
- Press Enter or click to start a print
- Real-time feedback in the console

### File Information Display
Each file entry shows:
- **Filename**: Name of the G-code file
- **Estimated Time**: Predicted print duration (if available in file metadata)
- **Filament**: Estimated filament usage (if available in file metadata)

## Mouse Controls

| Action | Result |
|--------|--------|
| **Scroll wheel** | Navigate through file list |
| **Click on file** | Select file |
| **Enter key** | Start print with selected file |

## Keyboard Controls

| Key | Action |
|-----|--------|
| `j` | Switch to Jobs tab |
| `↑` / `↓` | Navigate through file list |
| `PageUp` / `PageDown` | Scroll by page |
| `Home` / `End` | Jump to top/bottom of list |
| `Enter` | Start print with selected file |
| `c` | Switch to Console tab |
| `p` | Switch to Position tab |
| `m` | Return to Main dashboard |
| `q` | Quit application |

## Usage

### Viewing Available Files

1. Press `j` to open the Jobs tab
2. The file list automatically fetches when you enter the tab
3. Use mouse scroll wheel or arrow keys to navigate through the list
4. Selected file is highlighted

### Starting a Print Job

1. Navigate to the desired file using mouse or arrow keys
2. Press `Enter` to start the print
3. Check the console (press `c`) for confirmation
4. Monitor progress on the main dashboard (press `m`)

## States

### Not Connected
When not connected to a printer, the Jobs tab displays:
```
Not connected to printer
Connect to view available print files
```

### No Files
If no G-code files are found:
```
No print files found
Upload .gcode files to your printer's gcodes directory
```

### Loading
While fetching files:
```
Fetching files from Moonraker...
```

## Technical Details

### Data Source
Available files are fetched from Moonraker's `/server/files/list?root=gcodes` API endpoint.

### Job Start Method
Jobs are started using the `SDCARD_PRINT_FILE` GCode command, which instructs Klipper to begin printing the specified file.

## Notes

- The file list shows all .gcode files in the gcodes directory
- Files are automatically fetched when switching to the Jobs tab
- Starting a job requires an active connection to the printer
- The selected filename must exist in the printer's G-code files directory
- Mouse scrolling and clicking provide quick navigation and selection
