#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}

const HEIGHT: usize = 25;
const WIDTH: usize = 80;

use core::fmt;

use volatile::Volatile;
// Define Buffer using a single-dimensional array for ease of use with indexing.
struct Buffer {
    chars: [Volatile<ScreenChar>; HEIGHT * WIDTH],
}

pub struct Printer {
    col: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Printer {
    pub fn print_ch(&mut self, byte: u8) {
        match byte {
            b'\n' => self.ch_line(),
            byte => {
                if self.col >= WIDTH {
                    self.ch_line();
                }

                let row = HEIGHT - 1;
                let col = self.col;

                let color_code = self.color_code;
                self.buffer.chars[row * WIDTH + col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: color_code,
                });
                self.col += 1;
            }
        }
    }

    fn ch_line(&mut self) {
        for row in 1..HEIGHT {
            for col in 0..WIDTH {
                let character = self.buffer.chars[row * WIDTH + col].read();
                self.buffer.chars[(row - 1) * WIDTH + col].write(character);
            }
        }
        self.clear_row(HEIGHT - 1);
        self.col = 0;
    }
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..WIDTH {
            self.buffer.chars[row * WIDTH + col].write(blank);
        }
    }

    pub fn print_str(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // 可以是能打印的 ASCII 码字节，也可以是换行符
                0x20..=0x7e | b'\n' => self.print_ch(byte),
                // 不包含在上述范围之内的字节
                _ => self.print_ch(0xfe),
            }
        }
    }
}

impl fmt::Write for Printer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print_str(s);
        Ok(())
    }
}

// pub fn print_something() {
//     let mut printer = Printer {
//         col: 0,
//         color_code: ColorCode::new(Color::Yellow, Color::Black),
//         buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
//     };

//     printer.print_ch(b'H');
//     printer.print_str("ello ");
//     printer.print_str("Wörld!");
// }

// in src/vga_buffer.rs

use lazy_static::lazy_static;
use spin::Mutex;
lazy_static! {
    pub static ref WRITER: Mutex<Printer> = Mutex::new(Printer {
        col: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
