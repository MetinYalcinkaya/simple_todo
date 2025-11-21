use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Deserialize, Serialize)]
struct Task {
    id: u32,
    text: String,
    done: bool,
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if self.done { "[x]" } else { "[ ]" };
        write!(f, "{status} {}: {}", self.id, self.text)
    }
}

#[derive(Deserialize, Serialize)]
struct TodoList {
    tasks: Vec<Task>,
    next_id: u32,
}

impl Default for TodoList {
    fn default() -> Self {
        Self {
            tasks: Default::default(),
            next_id: 1,
        }
    }
}

impl TodoList {
    fn add(&mut self, text: String) -> &Task {
        let id = self.next_id;
        self.tasks.push(Task {
            id,
            text,
            done: false,
        });
        self.next_id = id + 1;
        self.tasks.last().unwrap()
    }

    fn print_list(&self) {
        for task in &self.tasks {
            println!("{task}");
        }
    }

    fn mark_done(&mut self, id: u32) -> Result<&Task, TodoError> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.done = true;
            Ok(task)
        } else {
            Err(TodoError::TaskNotFound)
        }
    }

    fn print_done(&self) {
        for task in self.tasks.iter().filter(|t| t.done) {
            println!("{task}");
        }
    }

    fn print_todo(&self) {
        for task in self.tasks.iter().filter(|t| !t.done) {
            println!("{task}");
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Command {
    Add { text: String },
    List,
    ListDone,
    ListTodo,
    Done { id: u32 },
    Help,
}

#[derive(Debug)]
enum TodoError {
    CommandError,
    TaskNotFound,
    InvalidId,
    SaveError,
}

impl From<std::num::ParseIntError> for TodoError {
    fn from(_: std::num::ParseIntError) -> Self {
        TodoError::InvalidId
    }
}

const PATH: &str = "src/todo.json";

fn main() -> Result<(), TodoError> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd: Command = parse_command(args)?;
    let mut task_list: TodoList = load_todo_list(PATH);
    run(cmd, &mut task_list)?;
    // save
    save_todo_list(PATH, &task_list)?;
    Ok(())
}

fn run(cmd: Command, todo_list: &mut TodoList) -> Result<(), TodoError> {
    match cmd {
        Command::Add { text } => {
            todo_list.add(text);
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
        Command::Done { id } => {
            println!("Marking {id} as done...");
            todo_list.mark_done(id)?;
            Ok(())
        }
        Command::Help => {
            println!("Available commands: Add, List, Help");
            Ok(())
        }
    }
}

fn parse_command(args: Vec<String>) -> Result<Command, TodoError> {
    let mut args_iter = args.into_iter();
    let sub = args_iter.next().ok_or(TodoError::CommandError)?;

    match sub.as_str() {
        "add" => {
            let text = args_iter.next().ok_or(TodoError::CommandError)?;
            Ok(Command::Add { text })
        }
        "done" => {
            let id: u32 = args_iter.next().ok_or(TodoError::CommandError)?.parse()?;
            Ok(Command::Done { id })
        }
        "list" => Ok(Command::List),
        "list-done" => Ok(Command::ListDone),
        "list-todo" => Ok(Command::ListTodo),
        "help" => Ok(Command::Help),
        _ => Err(TodoError::CommandError),
    }
}

fn load_todo_list(path: &str) -> TodoList {
    match fs::read_to_string(path) {
        Ok(contents) => serde_json::from_str::<TodoList>(&contents).unwrap_or_default(),
        Err(_) => TodoList::default(),
    }
}

fn save_todo_list(path: &str, list: &TodoList) -> Result<(), TodoError> {
    let json = serde_json::to_string_pretty(list).map_err(|_| TodoError::SaveError)?;
    std::fs::write(path, json).map_err(|_| TodoError::SaveError)?;
    Ok(())
}

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
    run(cmd1, &mut task_list)?;
    run(cmd2, &mut task_list)?;
    assert_eq!(task_list.tasks.len(), 2);
    Ok(())
}

#[test]
fn test_mark_done() -> Result<(), TodoError> {
    let args: Vec<String> = vec![String::from("add"), String::from("hello there")];
    let cmd = parse_command(args)?;
    let mut task_list: TodoList = Default::default();
    run(cmd, &mut task_list)?;
    let res = task_list.mark_done(1)?;
    assert_eq!(res.id, 1);
    assert!(res.done);
    Ok(())
}

#[test]
fn test_load_todo() {
    let list = load_todo_list("tests/data/test.json");
    assert_eq!(3, list.tasks.len());
}

#[test]
fn test_save_todo() -> Result<(), TodoError> {
    let args: Vec<String> = vec![String::from("add"), String::from("helle there")];
    let cmd = parse_command(args)?;
    let mut task_list: TodoList = Default::default();
    let path = "tests/data/save_test.json";
    run(cmd, &mut task_list)?;
    let _ = save_todo_list(path, &task_list);
    let saved = load_todo_list(path);
    assert_eq!(1, saved.tasks.len());
    // cleanup
    let _ = fs::remove_file(path);
    Ok(())
}

#[test]
fn test_print_todo_and_done() -> Result<(), TodoError> {
    let args1: Vec<String> = vec![String::from("add"), String::from("hello there")];
    let args2: Vec<String> = vec![String::from("add"), String::from("goodbye there")];
    let cmd1 = parse_command(args1)?;
    let cmd2 = parse_command(args2)?;
    let mut task_list: TodoList = Default::default();
    run(cmd1, &mut task_list)?;
    run(cmd2, &mut task_list)?;
    // TODO: test on stdout instead of like this
    let _ = task_list.mark_done(2)?;

    assert_eq!(1, task_list.tasks.iter().filter(|t| t.done).count());
    assert_eq!(1, task_list.tasks.iter().filter(|t| !t.done).count());
    Ok(())
}
