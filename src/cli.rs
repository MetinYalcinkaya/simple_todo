use crate::model::{Priority, TodoList};

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Add { text: String },
    List,
    ListDone,
    ListTodo,
    ListByPriority { priority: Priority },
    Done { id: u32 },
    SetPriority { id: u32, priority: Priority },
    Help,
}

#[derive(Debug)]
pub enum TodoError {
    UnknownCommand,
    MissingArgument,
    TaskNotFound,
    InvalidId,
    SaveError,
    PriorityError,
}

impl From<std::num::ParseIntError> for TodoError {
    fn from(_: std::num::ParseIntError) -> Self {
        TodoError::InvalidId
    }
}

impl std::fmt::Display for TodoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoError::UnknownCommand => write!(f, "invalid command"),
            TodoError::MissingArgument => write!(f, "invalid arguments"),
            TodoError::TaskNotFound => write!(f, "task with that id was not found"),
            TodoError::InvalidId => write!(f, "task id must be a positive integer"),
            TodoError::SaveError => write!(f, "failed to save todo list"),
            TodoError::PriorityError => write!(f, "unknown priority"),
        }
    }
}

pub fn execute_command(cmd: Command, todo_list: &mut TodoList) -> Result<(), TodoError> {
    match cmd {
        Command::Add { text } => {
            let task = todo_list.add(text);
            println!(
                "Added {} as {} with {} priority",
                task.text, task.id, task.priority
            );
            Ok(())
        }
        Command::List => {
            println!("Tasks:");
            todo_list.print_list();
            Ok(())
        }
        Command::ListDone => {
            println!("Tasks Done:");
            todo_list.print_done();
            Ok(())
        }
        Command::ListTodo => {
            println!("Tasks Todo:");
            todo_list.print_todo();
            Ok(())
        }
        Command::ListByPriority { priority } => {
            println!("Tasks with {} priority:", priority);
            todo_list.print_by_priority(priority);
            Ok(())
        }
        Command::Done { id } => {
            println!("Marking {id} as done...");
            let task = todo_list.mark_done(id)?;
            println!("Task {} marked as done.", task.id);
            Ok(())
        }
        Command::SetPriority { id, priority } => {
            let task = todo_list.set_priority(id, priority)?;
            println!("Set task {} to {} priority", task.id, task.priority);
            Ok(())
        }
        Command::Help => {
            println!(
                "Available commands: add, list, list-done, list-todo, list-prio, done, set-prio, help"
            );
            Ok(())
        }
    }
}

pub fn parse_command(args: Vec<String>) -> Result<Command, TodoError> {
    let mut args_iter = args.into_iter();
    let sub = args_iter.next().ok_or(TodoError::UnknownCommand)?;

    match sub.as_str() {
        "add" => {
            // TODO: check for quotation marks?
            let text = args_iter.next().ok_or(TodoError::MissingArgument)?;
            Ok(Command::Add { text })
        }
        "list" => Ok(Command::List),
        "list-done" => Ok(Command::ListDone),
        "list-todo" => Ok(Command::ListTodo),
        "list-prio" => {
            let priority: Priority = args_iter.next().ok_or(TodoError::PriorityError)?.parse()?;
            Ok(Command::ListByPriority { priority })
        }
        "done" => {
            let id: u32 = args_iter
                .next()
                .ok_or(TodoError::MissingArgument)?
                .parse()?;
            Ok(Command::Done { id })
        }
        "set-prio" => {
            let id: u32 = args_iter.next().ok_or(TodoError::InvalidId)?.parse()?;
            let priority: Priority = args_iter.next().ok_or(TodoError::PriorityError)?.parse()?;
            Ok(Command::SetPriority { id, priority })
        }
        "help" => Ok(Command::Help),
        _ => Err(TodoError::UnknownCommand),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmd_parsing() -> Result<(), TodoError> {
        let args: Vec<String> = vec![String::from("add"), String::from("hello there")];
        let expected = Command::Add {
            text: "hello there".to_string(),
        };
        assert_eq!(expected, parse_command(args)?);
        Ok(())
    }

    #[test]
    fn test_add() -> Result<(), TodoError> {
        let args1: Vec<String> = vec![String::from("add"), String::from("hello there")];
        let args2: Vec<String> = vec![String::from("add"), String::from("hello there")];
        let cmd1 = parse_command(args1)?;
        let cmd2 = parse_command(args2)?;
        let mut task_list: TodoList = Default::default();
        execute_command(cmd1, &mut task_list)?;
        execute_command(cmd2, &mut task_list)?;
        assert_eq!(task_list.tasks.len(), 2);
        Ok(())
    }

    #[test]
    fn test_mark_done() -> Result<(), TodoError> {
        let args: Vec<String> = vec![String::from("add"), String::from("hello there")];
        let cmd = parse_command(args)?;
        let mut task_list: TodoList = Default::default();
        execute_command(cmd, &mut task_list)?;
        let res = task_list.mark_done(1)?;
        assert_eq!(res.id, 1);
        assert!(res.done);
        Ok(())
    }

    #[test]
    fn test_print_todo_and_done() -> Result<(), TodoError> {
        let args1: Vec<String> = vec![String::from("add"), String::from("hello there")];
        let args2: Vec<String> = vec![String::from("add"), String::from("goodbye there")];
        let cmd1 = parse_command(args1)?;
        let cmd2 = parse_command(args2)?;
        let mut task_list: TodoList = Default::default();
        execute_command(cmd1, &mut task_list)?;
        execute_command(cmd2, &mut task_list)?;
        // TODO: test on stdout instead of like this
        let _ = task_list.mark_done(2)?;

        assert_eq!(1, task_list.tasks.iter().filter(|t| t.done).count());
        assert_eq!(1, task_list.tasks.iter().filter(|t| !t.done).count());
        Ok(())
    }
}
