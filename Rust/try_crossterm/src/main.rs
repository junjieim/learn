use std::io::{stdout, Write};

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
    event,
};

fn main() -> Result<()> {
    // using the macro
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Green),
        Print("Styled text here.\n"),
        ResetColor
    )?;

    // or using functions
    stdout()
        .execute(SetForegroundColor(Color::White))?
        .execute(SetBackgroundColor(Color::Red))?
        .execute(Print("Styled text here.\n"))?
        .execute(ResetColor)?;
    
    Ok(())
}