use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::io::{self};
use std::time::Duration;

fn main() -> Result<(), io::Error> {
    'main: loop {
        if event::poll(Duration::default())? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Release {
                    match key.code {
                        KeyCode::Esc => break 'main,
                        KeyCode::Char(c) => {
                            eprint!("{}", c)
                        }

                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}
