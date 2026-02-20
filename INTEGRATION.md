# Text Editor - Parameter Integration Guide

## ✅ Ready for External Program Calls!

Your text editor now **fully supports** being called by other executables with file parameters.

## Quick Start

### Build the Editor
```bash
cargo build --release
```

The executable will be at: `./target/release/text_editor.exe`

### Test It!

#### 1. From Command Line:
```bash
# Open existing file
./target/release/text_editor.exe README.md

# Create new file
./target/release/text_editor.exe mynewfile.txt

# Start empty (will prompt for filename on save)
./target/release/text_editor.exe
```

#### 2. From Another Rust Program:
```rust
use std::process::Command;

// Open text editor with a file
Command::new("text_editor.exe")
    .arg("config.txt")
    .status()
    .expect("Failed to open editor");
```

#### 3. Test the Example Shell:
```bash
# Compile and run the example shell
rustc example_shell.rs -o example_shell.exe
./example_shell.exe

# Then in the shell:
shell> edit test.txt
```

## How It Works

The text editor accepts the **first command-line argument** as the file path:

```
text_editor.exe <filepath>
```

### Behavior:

1. **File exists**: Opens the file for editing
2. **File doesn't exist**: Creates a new buffer with the filename preset (file created on first save)
3. **No argument**: Starts empty, prompts for filename when saving

### Smart Features:

- ✅ **Absolute paths**: `D:\projects\myfile.txt`
- ✅ **Relative paths**: `./config.txt`, `../other.txt`
- ✅ **Spaces in paths**: `"My Document.txt"`
- ✅ **Error handling**: Falls back gracefully if file can't be opened
- ✅ **File creation**: Automatically creates new files when saving

## Integration Examples

### From Your Shell Program

```rust
use std::process::Command;

pub fn open_editor(filepath: &str) -> Result<(), String> {
    let status = Command::new("text_editor.exe")
        .arg(filepath)
        .status()
        .map_err(|e| format!("Failed to launch editor: {}", e))?;
    
    if status.success() {
        Ok(())
    } else {
        Err(format!("Editor exited with error: {}", status))
    }
}

// Usage in your shell:
match user_command {
    "edit" => {
        if let Err(e) = open_editor(&filename) {
            eprintln!("Error: {}", e);
        }
    },
    // ... other commands
}
```

### From Python

```python
import subprocess
import sys

def edit_file(filepath):
    """Open file in text editor"""
    try:
        result = subprocess.run(
            ["text_editor.exe", filepath],
            check=True
        )
        return result.returncode == 0
    except subprocess.CalledProcessError as e:
        print(f"Editor error: {e}")
        return False
    except FileNotFoundError:
        print("text_editor.exe not found!")
        return False

# Usage
if edit_file("config.txt"):
    print("File edited successfully")
```

### From C/C++

```c
#include <stdlib.h>
#include <stdio.h>

int open_editor(const char* filepath) {
    char command[1024];
    snprintf(command, sizeof(command), "text_editor.exe \"%s\"", filepath);
    return system(command) == 0;
}

// Usage
if (open_editor("myfile.txt")) {
    printf("File edited successfully\n");
}
```

## File Operations

### The editor handles:
- **Reading** files when opening
- **Writing** files when saving (Ctrl-S)
- **Creating** new files that don't exist
- **Error messages** if file operations fail

### Exit behavior:
- Editor runs in **blocking mode** (caller waits)
- Returns **exit code 0** on success
- Prints errors to **stderr** on failure

## Testing the Integration

Run the example shell to see it in action:

```bash
# Build the text editor
cargo build --release

# Compile the example shell
rustc example_shell.rs -o example_shell.exe

# Run it
./example_shell.exe

# Try these commands:
shell> edit test.txt     # Opens text editor
shell> list              # List files
shell> exit              # Exit shell
```

## API Summary

### Command-Line Interface
```
text_editor.exe [filepath]
```

**Parameters:**
- `filepath` (optional): Path to file to open/create

**Exit Codes:**
- `0`: Success
- `!= 0`: Error

**Environment:**
- Runs in raw terminal mode
- Captures all keyboard input while running
- Restores terminal state on exit

## Notes for Integration

1. **Blocking Call**: The calling process will pause until the editor exits
2. **Full Screen**: Editor takes over the entire terminal
3. **File Locking**: Files are not locked - be careful with concurrent access
4. **Terminal State**: Terminal is restored to normal mode on exit
5. **Relative Paths**: Resolved relative to the calling process's working directory

## Keyboard Shortcuts (For Your Users)

- **Ctrl-Q**: Quit editor (return to caller)
- **Ctrl-S**: Save file
- **Ctrl-A**: Save as (change filename)
- **Arrow Keys**: Navigate text
- **Home/End**: Jump to line start/end
- **Backspace/Delete**: Remove characters
- **Enter**: New line

---

**Your text editor is now ready to be called by your shell or any other program!** 🎉

