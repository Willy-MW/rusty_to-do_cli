use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{Clear, ClearType};
use crossterm::ExecutableCommand;
use std::io::{self};

fn main() -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    let mut input = String::new();

    // Main loop to handle events and input
    'main: loop {
        // Clear the terminal
        stdout.execute(Clear(ClearType::All))?;
        println!("Rusty ToDo List!");
        println!("Type something or press ESC to quit.\n");

        if !input.is_empty() {
            println!("{}", input);
        }

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => break 'main, // Exit the program
                KeyCode::Enter => input.clear(),
                KeyCode::Char(c) => {
                    // Append typed characters to the input
                    input.push(c);
                }
                KeyCode::Backspace => {
                    // Handle backspace (remove last character)
                    input.pop();
                }
                _ => (),
            }
        }
    }

    Ok(())
}
