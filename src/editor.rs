use crate::buffer::Buffer;
use crate::terminal::Terminal;
use std::io::{self, Read, stdin, Write};
use std::path::PathBuf;

const HELP_MESSAGE: &str = "HELP: Ctrl-Q = quit | Ctrl-S = save | Ctrl-A = save as";

pub struct Editor {
    buffer: Buffer,
    _terminal: Terminal,
    cursor_x: usize,
    cursor_y: usize,
    scroll_offset: usize,
    screen_rows: u16,
    screen_cols: u16,
    quit: bool,
    status_message: String,
    save_count: u32,
    message_is_temporary: bool,
}

impl Editor {
    pub fn new() -> Self {
        let terminal = Terminal::new().expect("Failed to initialize terminal");
        let (rows, cols) = Terminal::get_terminal_size().unwrap_or((24, 80));

        Editor {
            buffer: Buffer::new(),
            _terminal: terminal,
            cursor_x: 0,
            cursor_y: 0,
            scroll_offset: 0,
            screen_rows: rows.saturating_sub(2), // Reserve 2 rows for status bar
            screen_cols: cols,
            quit: false,
            status_message: String::from(HELP_MESSAGE),
            save_count: 0,
            message_is_temporary: false,
        }
    }

    pub fn from_file(path: &str) -> io::Result<Self> {
        let terminal = Terminal::new().expect("Failed to initialize terminal");
        let (rows, cols) = Terminal::get_terminal_size().unwrap_or((24, 80));

        let buffer = Buffer::from_file(PathBuf::from(path))?;

        Ok(Editor {
            buffer,
            _terminal: terminal,
            cursor_x: 0,
            cursor_y: 0,
            scroll_offset: 0,
            screen_rows: rows.saturating_sub(2),
            screen_cols: cols,
            quit: false,
            status_message: String::from(HELP_MESSAGE),
            save_count: 0,
            message_is_temporary: false,
        })
    }

    pub fn new_with_filename(path: &str) -> io::Result<Self> {
        let terminal = Terminal::new().expect("Failed to initialize terminal");
        let (rows, cols) = Terminal::get_terminal_size().unwrap_or((24, 80));

        let mut buffer = Buffer::new();
        buffer.set_filename(PathBuf::from(path));

        Ok(Editor {
            buffer,
            _terminal: terminal,
            cursor_x: 0,
            cursor_y: 0,
            scroll_offset: 0,
            screen_rows: rows.saturating_sub(2),
            screen_cols: cols,
            quit: false,
            status_message: String::from(HELP_MESSAGE),
            save_count: 0,
            message_is_temporary: false,
        })
    }

    pub fn new_with_save_directory(directory: &str) -> io::Result<Self> {
        let terminal = Terminal::new().expect("Failed to initialize terminal");
        let (rows, cols) = Terminal::get_terminal_size().unwrap_or((24, 80));

        let mut buffer = Buffer::new();
        buffer.set_save_directory(PathBuf::from(directory));

        Ok(Editor {
            buffer,
            _terminal: terminal,
            cursor_x: 0,
            cursor_y: 0,
            scroll_offset: 0,
            screen_rows: rows.saturating_sub(2),
            screen_cols: cols,
            quit: false,
            status_message: String::from(HELP_MESSAGE),
            save_count: 0,
            message_is_temporary: false,
        })
    }

    pub fn set_save_directory(&mut self, directory: &str) {
        self.buffer.set_save_directory(PathBuf::from(directory));
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            self.refresh_screen()?;

            if self.quit {
                break;
            }

            self.process_keypress()?;
        }

        Terminal::clear_screen()?;
        Terminal::move_cursor(0, 0)?;
        Ok(())
    }

    fn refresh_screen(&self) -> io::Result<()> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor(0, 0)?;

        self.draw_rows()?;
        self.draw_status_bar()?;
        self.draw_message_bar()?;

        // Position cursor (add 5 for line number gutter)
        let screen_y = (self.cursor_y.saturating_sub(self.scroll_offset)) as u16;
        let screen_x = (self.cursor_x + 5) as u16;
        Terminal::move_cursor(screen_y, screen_x)?;

        Terminal::show_cursor()?;

        // Flush stdout to ensure immediate display
        io::stdout().flush()?;

        Ok(())
    }

    fn draw_rows(&self) -> io::Result<()> {
        for row in 0..self.screen_rows {
            Terminal::clear_line()?;

            let file_row = row as usize + self.scroll_offset;

            if file_row >= self.buffer.line_count() {
                // Draw empty rows with line numbers
                if self.buffer.line_count() == 0 && row == self.screen_rows / 3 {
                    let welcome = format!("Text Editor -- version 0.1.0");
                    let padding = (self.screen_cols as usize).saturating_sub(welcome.len()) / 2;
                    if padding > 0 {
                        print!("{:>4} ", file_row + 1);
                        print!("{}{}", " ".repeat(padding.saturating_sub(5)), welcome);
                    } else {
                        print!("{:>4} ", file_row + 1);
                    }
                } else {
                    print!("{:>4} ", file_row + 1);
                }
            } else {
                // Draw actual file content with line number
                print!("{:>4} ", file_row + 1);
                if let Some(line) = self.buffer.get_line(file_row) {
                    let len = std::cmp::min(line.len(), self.screen_cols.saturating_sub(5) as usize);
                    print!("{}", &line[..len]);
                }
            }

            println!("\r");
        }
        Ok(())
    }

    fn draw_status_bar(&self) -> io::Result<()> {
        Terminal::set_bg_color(238, 238, 238)?;
        Terminal::set_fg_color(0, 0, 0)?;

        let filename = self.buffer.filename()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("[No Name]");

        let modified = if self.buffer.is_modified() { " (modified)" } else { "" };
        let left_status = format!(" {} - {} lines{}", filename, self.buffer.line_count(), modified);

        let right_status = format!("{}/{} ", self.cursor_y + 1, self.cursor_x + 1);

        let mut status = left_status.clone();
        let status_len = left_status.len() + right_status.len();

        if status_len < self.screen_cols as usize {
            let padding = self.screen_cols as usize - status_len;
            status.push_str(&" ".repeat(padding));
        } else {
            status.truncate(self.screen_cols as usize - right_status.len());
        }

        status.push_str(&right_status);
        print!("{}", status);

        Terminal::reset_colors()?;
        println!("\r");
        Ok(())
    }

    fn draw_message_bar(&self) -> io::Result<()> {
        Terminal::clear_line()?;
        let msg_len = std::cmp::min(self.status_message.len(), self.screen_cols as usize);
        print!("{}", &self.status_message[..msg_len]);
        Ok(())
    }

    fn process_keypress(&mut self) -> io::Result<()> {
        let mut buffer = [0; 1];
        stdin().read_exact(&mut buffer)?;

        let byte = buffer[0];

        // Reset temporary message to help text if it was temporary
        if self.message_is_temporary {
            self.status_message = String::from(HELP_MESSAGE);
            self.message_is_temporary = false;
        }

        // Handle Ctrl key combinations
        const CTRL_Q: u8 = b'q' & 0x1f; // Ctrl-Q = 17
        const CTRL_S: u8 = b's' & 0x1f; // Ctrl-S = 19
        const CTRL_A: u8 = b'a' & 0x1f; // Ctrl-A = 1

        match byte {
            CTRL_Q => { // Ctrl-Q
                self.quit = true;
            },
            CTRL_S => { // Ctrl-S
                self.save_file();
            },
            CTRL_A => { // Ctrl-A (Save As)
                self.save_file_as();
            },
            0x1b => { // Escape sequence (arrow keys, etc.)
                self.handle_escape_sequence()?;
            },
            13 | 10 => { // Enter/Return
                self.insert_newline();
            },
            127 | 8 => { // Backspace
                self.delete_char();
            },
            byte if byte >= 32 && byte < 127 => { // Printable ASCII
                self.insert_char(byte as char);
            },
            _ => {
                // Ignore other control characters
            }
        }

        Ok(())
    }

    fn handle_escape_sequence(&mut self) -> io::Result<()> {
        let mut buffer = [0; 2];

        // Try to read the next two bytes
        if stdin().read_exact(&mut buffer).is_ok() {
            if buffer[0] == b'[' {
                match buffer[1] {
                    b'A' => self.move_cursor_up(),    // Up arrow
                    b'B' => self.move_cursor_down(),  // Down arrow
                    b'C' => self.move_cursor_right(), // Right arrow
                    b'D' => self.move_cursor_left(),  // Left arrow
                    b'H' => self.move_cursor_home(),  // Home
                    b'F' => self.move_cursor_end(),   // End
                    b'3' => {
                        // Delete key sends ESC[3~
                        let mut tilde = [0; 1];
                        if stdin().read_exact(&mut tilde).is_ok() && tilde[0] == b'~' {
                            self.delete_char_forward();
                        }
                    },
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn move_cursor_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.snap_cursor_to_line();
            self.adjust_scroll();
        }
    }

    fn move_cursor_down(&mut self) {
        // Only move down if there's a line below
        if self.cursor_y + 1 < self.buffer.line_count() {
            self.cursor_y += 1;
            self.snap_cursor_to_line();
            self.adjust_scroll();
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        } else if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = self.buffer.line_len(self.cursor_y);
            self.adjust_scroll();
        }
    }

    fn move_cursor_right(&mut self) {
        let line_len = self.buffer.line_len(self.cursor_y);
        if self.cursor_x < line_len {
            self.cursor_x += 1;
        } else if self.cursor_y < self.buffer.line_count() - 1 {
            // Wrap to next line if it exists
            self.cursor_y += 1;
            self.cursor_x = 0;


            self.adjust_scroll();
        }
    }

    fn move_cursor_home(&mut self) {
        self.cursor_x = 0;
    }

    fn move_cursor_end(&mut self) {
        self.cursor_x = self.buffer.line_len(self.cursor_y);
    }

    fn snap_cursor_to_line(&mut self) {
        let line_len = self.buffer.line_len(self.cursor_y);
        if self.cursor_x > line_len {
            self.cursor_x = line_len;
        }
    }

    fn adjust_scroll(&mut self) {
        if self.cursor_y < self.scroll_offset {
            self.scroll_offset = self.cursor_y;
        }
        if self.cursor_y >= self.scroll_offset + self.screen_rows as usize {
            self.scroll_offset = self.cursor_y - self.screen_rows as usize + 1;
        }
    }

    fn insert_char(&mut self, ch: char) {
        self.buffer.insert_char(self.cursor_y, self.cursor_x, ch);
        self.cursor_x += 1;
    }

    fn insert_newline(&mut self) {
        self.buffer.insert_newline(self.cursor_y, self.cursor_x);
        self.cursor_y += 1;
        self.cursor_x = 0;
        self.adjust_scroll();
    }

    fn delete_char(&mut self) {
        if self.cursor_x > 0 {
            self.buffer.delete_char(self.cursor_y, self.cursor_x - 1);
            self.cursor_x -= 1;
        } else if self.cursor_y > 0 {
            self.cursor_x = self.buffer.line_len(self.cursor_y - 1);
            self.buffer.delete_newline(self.cursor_y);
            self.cursor_y -= 1;
            self.adjust_scroll();
        }
    }

    fn delete_char_forward(&mut self) {
        let line_len = self.buffer.line_len(self.cursor_y);
        if self.cursor_x < line_len {
            self.buffer.delete_char(self.cursor_y, self.cursor_x);
        } else if self.cursor_y < self.buffer.line_count() - 1 {
            self.buffer.delete_newline(self.cursor_y + 1);
        }
    }

    fn save_file(&mut self) {
        // If no filename, prompt for one
        if self.buffer.filename().is_none() {
            if let Some(filename) = self.prompt_for_filename() {
                match self.buffer.save_as(PathBuf::from(&filename)) {
                    Ok(_) => {
                        self.save_count += 1;
                        self.status_message = format!("{} saved! (save #{})", filename, self.save_count);
                        self.message_is_temporary = true;
                    },
                    Err(e) => {
                        self.status_message = format!("Error saving file: {}", e);
                        self.message_is_temporary = true;
                    }
                }
            } else {
                self.status_message = format!("Save aborted");
                self.message_is_temporary = true;
            }
        } else {
            // File already has a name, save directly
            let fname = self.buffer.filename()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "file".to_string());

            self.save_count += 1;
            match self.buffer.save() {
                Ok(_) => {
                    self.status_message = format!("{} saved! (save #{})", fname, self.save_count);
                    self.message_is_temporary = true;
                },
                Err(e) => {
                    self.status_message = format!("Error saving file: {}", e);
                    self.message_is_temporary = true;
                }
            }
        }
    }

    fn save_file_as(&mut self) {
        // Always prompt for filename (even if one exists)
        if let Some(filename) = self.prompt_for_filename() {
            match self.buffer.save_as(std::path::PathBuf::from(filename)) {
                Ok(_) => {
                    self.status_message = format!("File saved as successfully!");
                    self.message_is_temporary = true;
                },
                Err(e) => {
                    self.status_message = format!("Error saving file: {}", e);
                    self.message_is_temporary = true;
                }
            }
        } else {
            self.status_message = format!("Save aborted");
            self.message_is_temporary = true;
        }
    }

    fn prompt_for_filename(&mut self) -> Option<String> {
        let mut filename = String::new();

        loop {
            // Draw the prompt
            Terminal::hide_cursor().ok()?;
            Terminal::move_cursor(self.screen_rows + 1, 0).ok()?;
            Terminal::clear_line().ok()?;
            print!("Save as: {}", filename);
            io::stdout().flush().ok()?;
            Terminal::show_cursor().ok()?;

            // Read a character
            let mut buffer = [0; 1];
            if stdin().read_exact(&mut buffer).is_err() {
                // Clean up before returning
                Terminal::move_cursor(self.screen_rows + 1, 0).ok();
                Terminal::clear_line().ok();
                return None;
            }

            let byte = buffer[0];

            match byte {
                13 | 10 => { // Enter - confirm
                    if !filename.is_empty() {
                        // Clean up the prompt line before returning
                        Terminal::move_cursor(self.screen_rows + 1, 0).ok();
                        Terminal::clear_line().ok();
                        return Some(filename);
                    }
                },
                27 => { // Escape - cancel
                    // Clean up before returning
                    Terminal::move_cursor(self.screen_rows + 1, 0).ok();
                    Terminal::clear_line().ok();
                    return None;
                },
                127 | 8 => { // Backspace
                    filename.pop();
                },
                byte if byte >= 32 && byte < 127 => { // Printable ASCII
                    filename.push(byte as char);
                },
                _ => {}
            }
        }
    }
}

