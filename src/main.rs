//!
//!
//!  This is the main entry point for the terminal-based text editor application.
//!
//!
//!
//!


mod terminal;
mod editor;
mod buffer;
mod constants;
mod editor_response;

use std::io;
use std::env;
use std::path::{Path, PathBuf};


/// Represents different modes for launching the editor
#[derive(Debug)]
enum LaunchMode {
    /// Open an existing file at the specified path
    OpenFile(PathBuf),
    /// Create a new file at the specified path
    CreateFile(PathBuf),
    StartCoding(PathBuf),
    EmptyEditor,
}

/// Configuration for initializing the editor
struct EditorConfig {
    mode: LaunchMode,
    save_directory: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    let config = parse_launch_config(&args)?;
    let mut editor = create_editor_from_config(config)?;

    editor.run()?;
    Ok(())
}

/// Parse command-line arguments and determine the launch mode
fn parse_launch_config(args: &[String]) -> io::Result<EditorConfig> {
    if args.len() <= 1 {
        return Ok(EditorConfig {
            mode: LaunchMode::EmptyEditor,
            save_directory: None,
        });
    }

    let path = PathBuf::from(&args[1]);

    let (mode, save_dir) = determine_launch_mode(&path)?;

    Ok(EditorConfig {
        mode,
        save_directory: save_dir,
    })
}

/// Analyze the path and determine the appropriate launch mode
fn determine_launch_mode(path: &Path) -> io::Result<(LaunchMode, Option<PathBuf>)> {
    if path.is_file() {
        // Existing file - open it
        let save_dir = path.parent().map(|p| p.to_path_buf());
        Ok((LaunchMode::OpenFile(path.to_path_buf()), save_dir))
    } else if path.is_dir() {
        // Directory - start coding mode
        eprintln!("Starting coding session in directory: {}", path.display());
        Ok((LaunchMode::StartCoding(path.to_path_buf()), Some(path.to_path_buf())))
    } else {
        // Path doesn't exist - validate parent and create new file
        validate_parent_directory(path)?;
        eprintln!("Creating new file at: {}", path.display());

        let save_dir = path.parent().map(|p| p.to_path_buf());
        Ok((LaunchMode::CreateFile(path.to_path_buf()), save_dir))
    }
}

/// Validate that the parent directory exists for a new file
fn validate_parent_directory(path: &Path) -> io::Result<()> {
    if let Some(parent_dir) = path.parent() {
        if !parent_dir.as_os_str().is_empty() && !parent_dir.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("{}", editor_response::EditorWithArgsResponse::ParentDirDoesNotExist(parent_dir.to_path_buf())),
            ));
        }
    }
    Ok(())
}

/// Create and initialize an Editor based on the configuration
fn create_editor_from_config(config: EditorConfig) -> io::Result<editor::Editor> {
    let mut editor = match config.mode {
        LaunchMode::OpenFile(path) => {
            open_existing_file(&path)?
        },
        LaunchMode::CreateFile(path) => {
            create_new_file_editor(&path)?
        },
        LaunchMode::StartCoding(dir) => {
            editor::Editor::new_with_save_directory(&dir.to_string_lossy())?
        },
        LaunchMode::EmptyEditor => {
            editor::Editor::new()
        },
    };

    // Apply save directory if specified
    if let Some(save_dir) = config.save_directory {
        editor.set_save_directory(&save_dir.to_string_lossy().to_string());
    }

    Ok(editor)
}

/// Open an existing file, with fallback to creating it if opening fails
fn open_existing_file(path: &Path) -> io::Result<editor::Editor> {
    match editor::Editor::from_file(&path.to_string_lossy()) {
        Ok(editor) => Ok(editor),
        Err(e) => {
            eprintln!("Error opening file '{}': {}", path.display(), e);
            eprintln!("Starting with empty buffer instead...");
            editor::Editor::new_with_filename(&path.to_string_lossy())
        }
    }
}

/// Create an editor for a new file
fn create_new_file_editor(path: &Path) -> io::Result<editor::Editor> {
    editor::Editor::new_with_filename(&path.to_string_lossy())
}

