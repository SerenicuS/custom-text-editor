# Text Editor

A simple terminal-based text editor written in Rust, designed to be integrated with the MommySuite shell application.

## Features

- Basic text editing capabilities
- File management (open, create, save)
- Buffer management with line-based editing
- Terminal UI with cursor tracking
- Support for multiple file types

## Project Structure

```
text_editor/
├── src/
│   ├── main.rs          # Entry point and application logic
│   ├── editor.rs        # Editor state and main editing operations
│   ├── buffer.rs        # Text buffer implementation
│   ├── terminal.rs      # Terminal UI and input handling
├── Cargo.toml           # Project dependencies and metadata
└── README.md            # This file
```

## Building

To build the project, ensure you have Rust installed, then run:

```bash
cargo build --release
```

The compiled executable will be available at `target/release/text_editor.exe`.

## Running

### From Command Line

```bash
text_editor.exe <save_directory> [filename]
```

**Parameters:**
- `save_directory` - The directory where files will be saved (required)
- `filename` - Optional existing file to open

**Examples:**

```bash
# Start a new file (no file specified)
text_editor.exe D:\Jetbrains_IDE_Projects\rustrover\MommySuite\sandbox

# Open an existing file
text_editor.exe D:\Jetbrains_IDE_Projects\rustrover\MommySuite\sandbox test.mommy
```

### Integration with MommySuite

The text editor is designed to be called by the MommySuite shell. Two main modes are supported:

1. **Start Coding** - Create a new file with no filename specified
2. **Open File** - Open an existing file and save it in the output directory (if `.mommy` file type) or in its original location

## Buffer Management

The `Buffer` struct handles all text content and file operations:

- **New Buffer**: Creates an empty buffer
- **From File**: Loads content from an existing file
- **Save**: Saves buffer to the current filename location
- **Save As**: Saves buffer with a new filename

### Key Methods

- `insert_char(row, col, ch)` - Insert a character at a specific position
- `delete_char(row, col)` - Delete a character
- `insert_newline(row, col)` - Break a line into two
- `delete_newline(row)` - Join a line with the previous one
- `is_modified()` - Check if buffer has unsaved changes
- `set_save_directory(directory)` - Set the default save directory for new files

## Editor Operations

The editor supports standard text editing operations:

- **Navigation**: Arrow keys to move cursor
- **Input**: Type characters to insert
- **Line Breaking**: Enter key to create new lines
- **Backspace**: Delete characters
- **Save**: Keyboard shortcuts to save files
- **Quit**: Exit the editor

## Dependencies

See `Cargo.toml` for the complete list of dependencies.

## Terminal Support

The editor uses terminal operations for:

- Cursor positioning
- Display updates
- Input handling
- Screen clearing

## Notes

- Files are saved with UTF-8 encoding
- Line endings are normalized to `\n`
- The editor tracks unsaved changes with the `modified` flag
- Empty files are initialized with a single empty line

## Troubleshooting

**File Not Found**: Ensure the file exists in the specified directory and the full path is provided correctly.

**Save Issues**: Verify that the save directory exists and has write permissions.

**Display Issues**: Some terminal emulators may have rendering issues. Try maximizing the terminal window or adjusting terminal settings.

