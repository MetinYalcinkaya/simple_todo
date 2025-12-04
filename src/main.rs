use clap::Parser;
use todo::cli::{Cli, TodoError, execute_command};
use todo::persistence::{PATH, load_todo_list, save_todo_list};

fn main() {
    if let Err(e) = run_todo() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run_todo() -> Result<(), TodoError> {
    let cli = Cli::parse();
    let mut task_list = load_todo_list(PATH);
    execute_command(cli.command, &mut task_list)?;
    // save
    save_todo_list(PATH, &task_list)?;
    Ok(())
}
