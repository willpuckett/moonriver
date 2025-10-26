# Job History Browser

The Job History Browser allows you to view your print history and start new print jobs directly from the TUI interface.

## Features

### View Print History
- Browse through up to 50 recent print jobs
- See job status at a glance (completed ✓, failed ✗, cancelled ✗)
- View print duration, filament used, and completion time
- Automatic refresh when entering the Jobs tab

### Start Print Jobs
- Select any completed job from history
- Press Enter to start a new print with the same file
- Real-time feedback in the console

### Job Information Display
Each job entry shows:
- **Status Icon**: Visual indicator (✓ for completed, ✗ for failed/cancelled)
- **Filename**: Name of the G-code file
- **Duration**: Total print time (hours, minutes, seconds)
- **Filament Used**: Amount of filament consumed (in mm)
- **Completion Time**: Date and time when the job finished

## Keyboard Controls

| Key | Action |
|-----|--------|
| `j` | Switch to Jobs tab |
| `↑` / `↓` | Navigate through job list |
| `j` / `k` | Alternative navigation (vim-style) |
| `Enter` | Start print with selected job |
| `r` | Refresh job list |
| `m` | Return to Main dashboard |
| `q` | Quit application |

## Usage

### Viewing Job History

1. Press `j` to open the Jobs tab
2. The job list automatically fetches when you enter the tab
3. Use arrow keys or `j`/`k` to navigate through the list
4. Selected job is highlighted with a `▶` indicator

### Starting a Print Job

1. Navigate to the desired job using arrow keys
2. Press `Enter` to start the print
3. Check the console (press `c`) for confirmation

### Refreshing the List

Press `r` at any time while on the Jobs tab to fetch the latest job history from Moonraker.

## States

### Not Connected
When not connected to a printer, the Jobs tab displays:
```
Not connected to printer
Connect to view job history
```

### Empty History
If no jobs have been printed yet:
```
No print jobs found
Job history will appear here once you start printing
```

### Loading
While fetching jobs:
```
Fetching jobs from Moonraker...
```

## Technical Details

### Data Source
Job history is fetched from Moonraker's `/server/history/list` API endpoint with:
- Limit: 50 jobs
- Order: Descending (most recent first)

### Job Start Method
Jobs are started using the `SDCARD_PRINT_FILE` GCode command, which instructs Klipper to begin printing the specified file from the virtual SD card.

## Notes

- The job list shows a maximum of 50 recent jobs
- Jobs are automatically fetched when switching to the Jobs tab
- Manual refresh is available via the `r` key
- Starting a job requires an active connection to the printer
- The selected filename must exist in the printer's G-code files directory
