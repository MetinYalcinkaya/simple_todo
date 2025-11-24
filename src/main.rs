mod cli;
mod model;
mod persistence;

use crate::cli::{Command, TodoError, execute_command, parse_command};
use crate::model::TodoList;
use crate::persistence::{PATH, load_todo_list, save_todo_list};

fn main() {
    if let Err(e) = run_todo() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run_todo() -> Result<(), TodoError> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd: Command = parse_command(args)?;
    let mut task_list: TodoList = load_todo_list(PATH);
    execute_command(cmd, &mut task_list)?;
    // save
    save_todo_list(PATH, &task_list)?;
    Ok(())
}
