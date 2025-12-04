use crate::model::{Priority, TodoList};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "a simple cli todo manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug, PartialEq, Clone)]
pub enum Command {
    /// adds a new task
    Add {
        /// contents of the task
        text: String,
    },
    /// lists all tasks
    List,
    /// lists tasks you've completed
    ListDone,
    /// lists tasks that need to be completed
    ListTodo,
    /// lists tasks by given priority
    ListByPriority {
        /// priority level [low|med|high]
        priority: Priority,
    },
    /// mark task as done
    Done {
        /// task id
        id: u32,
    },
    /// set priority of a task
    SetPriority {
        /// task id
        id: u32,
        /// priority level [low|med|high]
        priority: Priority,
    },
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() -> Result<(), TodoError> {
        let cmd1 = Command::Add {
            text: String::from("hello there"),
        };
        let cmd2 = Command::Add {
            text: String::from("hello there"),
        };
        let mut task_list: TodoList = Default::default();
        execute_command(cmd1, &mut task_list)?;
        execute_command(cmd2, &mut task_list)?;
        assert_eq!(task_list.tasks.len(), 2);
        Ok(())
    }

    #[test]
    fn test_mark_done() -> Result<(), TodoError> {
        let cmd = Command::Add {
            text: String::from("hello there"),
        };
        let mut task_list: TodoList = Default::default();
        execute_command(cmd, &mut task_list)?;
        let res = task_list.mark_done(1)?;
        assert_eq!(res.id, 1);
        assert!(res.done);
        Ok(())
    }

    #[test]
    fn test_print_todo_and_done() -> Result<(), TodoError> {
        let cmd1 = Command::Add {
            text: String::from("hello there"),
        };
        let cmd2 = Command::Add {
            text: String::from("goodbye there"),
        };
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
