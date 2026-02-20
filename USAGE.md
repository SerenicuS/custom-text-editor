# Text Editor - Usage Guide

## Command-Line Parameters

The text editor accepts file paths as command-line arguments and can be called by other programs.

### Basic Usage

#### 1. Open an existing file:
```bash
text_editor.exe path/to/file.txt
```

#### 2. Create a new file:
```bash
text_editor.exe newfile.txt
```
- If the file doesn't exist, it will be created when you save (Ctrl-S)

#### 3. Start with empty buffer:
```bash
text_editor.exe
```
- You'll be prompted for a filename when you save (Ctrl-S)

### Calling from Another Executable

#### From Rust:
```rust
use std::process::Command;

fn open_editor(filepath: &str) {
    Command::new("text_editor.exe")
        .arg(filepath)
        .status()
        .expect("Failed to open text editor");
}

// Example usage:
open_editor("config.txt");
```

#### From C/C++:
```c
#include <stdlib.h>

void open_editor(const char* filepath) {
    char command[512];
    sprintf(command, "text_editor.exe %s", filepath);
    system(command);
}

// Example usage:
open_editor("config.txt");
```

#### From Python:
```python
import subprocess

def open_editor(filepath):
    subprocess.run(["text_editor.exe", filepath])

# Example usage:
open_editor("config.txt")
```

#### From PowerShell:
```powershell
# Open existing file
& ".\text_editor.exe" "myfile.txt"

# Create new file
& ".\text_editor.exe" "newfile.txt"

# With full path
& "D:\path\to\text_editor.exe" "D:\path\to\file.txt"
```

#### From Batch/CMD:
```batch
REM Open file
text_editor.exe myfile.txt

REM With full path
"D:\path\to\text_editor.exe" "D:\path\to\file.txt"
```

### Path Handling

The editor supports:
- **Relative paths**: `text_editor.exe ./config.txt`
- **Absolute paths**: `text_editor.exe D:\projects\myfile.txt`
- **Paths with spaces**: `text_editor.exe "My Document.txt"`

### Keyboard Shortcuts

- **Ctrl-Q**: Quit
- **Ctrl-S**: Save
- **Ctrl-A**: Save As
- **Arrow Keys**: Navigate
- **Home/End**: Jump to start/end of line
- **Backspace**: Delete character before cursor
- **Delete**: Delete character after cursor
- **Enter**: New line

### Integration Example

Here's a complete example of a shell that calls the text editor:

```rust
// shell.rs
use std::process::Command;
use std::io::{self, Write};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let parts: Vec<&str> = input.split_whitespace().collect();
        
        match parts.get(0) {
            Some(&"edit") => {
                if let Some(filename) = parts.get(1) {
                    // Call the text editor
                    Command::new("./text_editor.exe")
                        .arg(filename)
                        .status()
                        .expect("Failed to open text editor");
                    
                    println!("Editor closed, returning to shell...");
                } else {
                    println!("Usage: edit <filename>");
                }
            },
            Some(&"exit") => break,
            _ => println!("Unknown command. Try: edit <filename> or exit"),
        }
    }
}
```

### Exit Codes

- **0**: Successfully edited and saved
- **Non-zero**: Error occurred (file permission issues, etc.)

### Notes

- The editor runs in **blocking mode** - the calling process will wait until the editor is closed
- File changes are **persisted immediately** when you press Ctrl-S
- The editor handles **file creation** automatically if the file doesn't exist
- **Error messages** are printed to stderr if file operations fail

