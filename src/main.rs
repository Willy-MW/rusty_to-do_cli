use anyhow::Result;
use to_do_cli::input::{clear_screen, handle_user_input, print_header, process_input, Action};
use to_do_cli::task_list::TaskList;

fn main() -> Result<()> {
    let mut input = String::new();
    let mut task_list = TaskList::default();

    print_header();

    'main: loop {
        match handle_user_input(&mut input)? {
            Some(Action::ProcessInput) => {
                clear_screen();
                process_input(&input, &mut task_list)?;
                input.clear();
                print_header();
                print!("{}", task_list);
            }
            Some(Action::Exit) => break 'main,
            None => {}
        }
    }

    Ok(())
}
