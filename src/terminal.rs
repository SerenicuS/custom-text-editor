use std::io::{self, Write, stdout};

#[cfg(windows)]
use std::os::windows::io::AsRawHandle;

pub struct Terminal {
    #[cfg(windows)]
    original_mode: u32,
}

impl Terminal {
    pub fn new() -> io::Result<Self> {
        let mut terminal = Terminal {
            #[cfg(windows)]
            original_mode: 0,
        };
        terminal.enable_raw_mode()?;
        Ok(terminal)
    }

    #[cfg(windows)]
    fn enable_raw_mode(&mut self) -> io::Result<()> {
        let handle = io::stdin().as_raw_handle();
        
        // Get current console mode
        unsafe {
            if GetConsoleMode(handle as *mut _, &mut self.original_mode) == 0 {
                return Err(io::Error::last_os_error());
            }
        }
        
        // Enable virtual terminal processing for ANSI escape sequences
        let mut mode = self.original_mode;
        mode &= !(ENABLE_ECHO_INPUT | ENABLE_LINE_INPUT);
        mode |= ENABLE_VIRTUAL_TERMINAL_INPUT;
        
        unsafe {
            if SetConsoleMode(handle as *mut _, mode) == 0 {
                return Err(io::Error::last_os_error());
            }
        }
        
        // Enable VT processing for output
        let out_handle = io::stdout().as_raw_handle();
        let mut out_mode = 0;
        unsafe {
            if GetConsoleMode(out_handle as *mut _, &mut out_mode) != 0 {
                out_mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
                SetConsoleMode(out_handle as *mut _, out_mode);
            }
        }
        
        Ok(())
    }

    #[cfg(not(windows))]
    fn enable_raw_mode(&mut self) -> io::Result<()> {
        // Unix/Linux raw mode would go here
        // For now, we'll focus on Windows since that's your OS
        Ok(())
    }

    pub fn clear_screen() -> io::Result<()> {
        print!("\x1b[2J");
        stdout().flush()
    }

    pub fn move_cursor(row: u16, col: u16) -> io::Result<()> {
        print!("\x1b[{};{}H", row + 1, col + 1);
        stdout().flush()
    }

    pub fn hide_cursor() -> io::Result<()> {
        print!("\x1b[?25l");
        stdout().flush()
    }

    pub fn show_cursor() -> io::Result<()> {
        print!("\x1b[?25h");
        stdout().flush()
    }

    pub fn clear_line() -> io::Result<()> {
        print!("\x1b[2K");
        stdout().flush()
    }

    pub fn get_terminal_size() -> io::Result<(u16, u16)> {
        #[cfg(windows)]
        {
            use std::mem;
            let handle = io::stdout().as_raw_handle();
            let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = unsafe { mem::zeroed() };
            
            unsafe {
                if GetConsoleScreenBufferInfo(handle as *mut _, &mut csbi) == 0 {
                    return Ok((24, 80)); // Default fallback
                }
            }
            
            let width = (csbi.srWindow.Right - csbi.srWindow.Left + 1) as u16;
            let height = (csbi.srWindow.Bottom - csbi.srWindow.Top + 1) as u16;
            Ok((height, width))
        }
        
        #[cfg(not(windows))]
        {
            Ok((24, 80)) // Default fallback for non-Windows
        }
    }

    pub fn set_fg_color(r: u8, g: u8, b: u8) -> io::Result<()> {
        print!("\x1b[38;2;{};{};{}m", r, g, b);
        stdout().flush()
    }

    pub fn set_bg_color(r: u8, g: u8, b: u8) -> io::Result<()> {
        print!("\x1b[48;2;{};{};{}m", r, g, b);
        stdout().flush()
    }

    pub fn reset_colors() -> io::Result<()> {
        print!("\x1b[0m");
        stdout().flush()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        #[cfg(windows)]
        {
            let handle = io::stdin().as_raw_handle();
            unsafe {
                SetConsoleMode(handle as *mut _, self.original_mode);
            }
        }
        let _ = Terminal::show_cursor();
        let _ = Terminal::reset_colors();
    }
}

// Windows API declarations
#[cfg(windows)]
type HANDLE = *mut std::ffi::c_void;

#[cfg(windows)]
const ENABLE_ECHO_INPUT: u32 = 0x0004;
#[cfg(windows)]
const ENABLE_LINE_INPUT: u32 = 0x0002;
#[cfg(windows)]
const ENABLE_VIRTUAL_TERMINAL_INPUT: u32 = 0x0200;
#[cfg(windows)]
const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

#[cfg(windows)]
#[repr(C)]
#[allow(non_snake_case)]
struct COORD {
    X: i16,
    Y: i16,
}

#[cfg(windows)]
#[repr(C)]
#[allow(non_snake_case, non_camel_case_types)]
struct SMALL_RECT {
    Left: i16,
    Top: i16,
    Right: i16,
    Bottom: i16,
}

#[cfg(windows)]
#[repr(C)]
#[allow(non_snake_case, non_camel_case_types)]
struct CONSOLE_SCREEN_BUFFER_INFO {
    dwSize: COORD,
    dwCursorPosition: COORD,
    wAttributes: u16,
    srWindow: SMALL_RECT,
    dwMaximumWindowSize: COORD,
}

#[cfg(windows)]
#[allow(non_snake_case)]
unsafe extern "system" {
    fn GetConsoleMode(hConsoleHandle: HANDLE, lpMode: *mut u32) -> i32;
    fn SetConsoleMode(hConsoleHandle: HANDLE, dwMode: u32) -> i32;
    fn GetConsoleScreenBufferInfo(hConsoleHandle: HANDLE, lpConsoleScreenBufferInfo: *mut CONSOLE_SCREEN_BUFFER_INFO) -> i32;
}

