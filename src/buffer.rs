use std::fs;
use std::io;
use std::path::PathBuf;

pub struct Buffer {
    lines: Vec<String>,
    filename: Option<PathBuf>,
    save_directory: Option<PathBuf>,
    modified: bool,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            lines: vec![String::new()],
            filename: None,
            save_directory: None,
            modified: false,
        }
    }

    pub fn from_file(path: PathBuf) -> io::Result<Self> {
        let content = fs::read_to_string(&path)?;
        let lines: Vec<String> = if content.is_empty() {
            vec![String::new()]
        } else {
            content.lines().map(|s| s.to_string()).collect()
        };

        Ok(Buffer {
            lines,
            filename: Some(path),
            save_directory: None,
            modified: false,
        })
    }

    pub fn save(&mut self) -> io::Result<()> {
        if let Some(path) = &self.filename {
            let content = self.lines.join("\n");
            fs::write(path, content)?;
            self.modified = false;
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "No filename"))
        }
    }

    pub fn save_as(&mut self, path: PathBuf) -> io::Result<()> {
        // If we're in "start coding" mode (only save_directory set), construct full path
        let full_path = if self.filename.is_none() && self.save_directory.is_some() {
            let dir = self.save_directory.as_ref().unwrap();
            dir.join(&path)
        } else {
            path
        };
        
        self.filename = Some(full_path);
        self.save()
    }

    pub fn set_save_directory(&mut self, directory: PathBuf) {
        self.save_directory = Some(directory);
    }

    pub fn get_line(&self, index: usize) -> Option<&str> {
        self.lines.get(index).map(|s| s.as_str())
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn insert_char(&mut self, row: usize, col: usize, ch: char) {
        if row < self.lines.len() {
            if col <= self.lines[row].len() {
                self.lines[row].insert(col, ch);
                self.modified = true;
            }
        }
    }

    pub fn delete_char(&mut self, row: usize, col: usize) {
        if row < self.lines.len() && col < self.lines[row].len() {
            self.lines[row].remove(col);
            self.modified = true;
        }
    }

    pub fn insert_newline(&mut self, row: usize, col: usize) {
        if row < self.lines.len() {
            let current_line = &self.lines[row];
            let new_line = current_line[col..].to_string();
            self.lines[row].truncate(col);
            self.lines.insert(row + 1, new_line);
            self.modified = true;
        }
    }

    pub fn delete_newline(&mut self, row: usize) {
        if row > 0 && row < self.lines.len() {
            let current = self.lines.remove(row);
            self.lines[row - 1].push_str(&current);
            self.modified = true;
        }
    }

    pub fn is_modified(&self) -> bool {
        self.modified
    }

    pub fn filename(&self) -> Option<&PathBuf> {
        self.filename.as_ref()
    }

    pub fn set_filename(&mut self, path: PathBuf) {
        self.filename = Some(path);
    }

    pub fn line_len(&self, row: usize) -> usize {
        self.lines.get(row).map(|l| l.len()).unwrap_or(0)
    }
}

