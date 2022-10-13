use std::{io::{self, Stdout}, error::Error};

use crossterm::{
    terminal::{self, LeaveAlternateScreen, EnterAlternateScreen}, 
    cursor::{Hide, Show}, 
    ExecutableCommand
};

pub fn init() -> Result<Stdout, Box<dyn Error>> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;
    Ok(stdout)
}

pub fn deinit(stdout: &mut Stdout) -> Result<(), Box<dyn Error>> {
    terminal::disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;
    stdout.execute(Show)?;
    Ok(())
}
