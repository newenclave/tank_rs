use std::io::{Stdout, Write};
use crossterm::{
    QueueableCommand, 
    style::{
        SetBackgroundColor, Color
    }, 
    terminal:: {
        Clear,
        ClearType
    },
    cursor::{
        MoveTo,
    }
};
use crate::{braille_canvas::BrailleCanvas, braille};

pub fn render(stdout: &mut Stdout, last_frame: &BrailleCanvas, current_frame: &BrailleCanvas, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }
    for (x, col) in current_frame.area().iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            if *s != last_frame.area()[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", braille::to_char(*s));
            }
        }
    }
    stdout.flush().unwrap();
} 
