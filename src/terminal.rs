use std::{
    error::Error,
    io::{self, Stdout},
};

use crossterm::{
    cursor::{Hide, Show},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
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
