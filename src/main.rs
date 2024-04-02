mod sl;

use crossterm::{cursor::MoveTo, queue, style::Print, ExecutableCommand, Result};

pub struct Options {
    accident: bool,
    fly: bool,
    logo: bool,
    c51: bool,
}

impl Options {
    pub fn new() -> Options {
        Options {
            accident: false,
            fly: false,
            logo: false,
            c51: false,
        }
    }

    pub fn set_option(&mut self, str: &str) {
        for c in str.chars() {
            match c {
                'a' => self.accident = true,
                'F' => self.fly = true,
                'l' => self.logo = true,
                'c' => self.c51 = true,
                _ => {}
            }
        }
    }
}

fn my_mvaddstr(y: u16, x: u16, str: &str) -> Result<()> {
    let mut stdout = std::io::stdout();

    for (i, ch) in str.chars().enumerate() {
        if i >= x as usize {
            queue!(stdout, MoveTo(x + i as u16, y), Print(ch))?;
        }
    }

    Ok(())
}

fn main() {
    let mut i: u32;
    let mut x: u32;

    println!("Hello, world!");
}
