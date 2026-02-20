mod terminal;
mod editor;
mod buffer;

use std::io;
use std::env;
use std::path::Path;

fn main() -> io::Result<()> {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    
    let mut editor = if args.len() > 1 {
        let filepath = &args[1]; // Required: full absolute path to file or save directory

        // Determine if this is a file path or a directory
        let path = Path::new(filepath);

        if path.is_file() {
            // openfile: file exists, open it
            eprintln!("DEBUG: Opening existing file at: {}", filepath);

            match editor::Editor::from_file(&filepath) {
                Ok(mut editor) => {
                    // Extract directory for saving
                    if let Some(dir) = path.parent() {
                        editor.set_save_directory(&dir.to_string_lossy().to_string());
                    }
                    editor
                },
                Err(e) => {
                    eprintln!("Error opening file '{}': {}", filepath, e);
                    eprintln!("Starting with empty buffer instead...");
                    let mut editor = editor::Editor::new_with_filename(filepath)?;
                    if let Some(dir) = path.parent() {
                        editor.set_save_directory(&dir.to_string_lossy().to_string());
                    }
                    editor
                }
            }
        } else if path.is_dir() {
            // startcoding: directory provided, no file specified
            eprintln!("Starting coding session in directory: {}", filepath);
            editor::Editor::new_with_save_directory(&filepath)?
        } else {
            // Path doesn't exist yet - could be a new file
            eprintln!("Creating new file at: {}", filepath);

            // Validate parent directory exists
            if let Some(parent_dir) = path.parent() {
                if !parent_dir.as_os_str().is_empty() && !parent_dir.is_dir() {
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        format!("Parent directory does not exist: {}", parent_dir.display())
                    ));
                }
            }

            let mut editor = editor::Editor::new_with_filename(filepath)?;
            if let Some(dir) = path.parent() {
                editor.set_save_directory(&dir.to_string_lossy().to_string());
            }
            editor
        }
    } else {
        eprintln!("Usage: text_editor <path>");
        eprintln!("  path: Full path to file to open/create, or directory for coding mode");
        editor::Editor::new()
    };
    
    editor.run()?;
    Ok(())
}
