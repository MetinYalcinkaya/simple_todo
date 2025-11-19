#[derive(Default, Clone)]
struct Task {
    id: u32,
    text: String,
    done: bool,
}

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
            println!("{}: {} -- Done: {}", task.id, task.text, task.done);
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
}

#[derive(Debug, PartialEq, Clone)]
enum Command {
    Add { text: String },
    List,
    Done { id: u32 },
    Help,
}

#[derive(Debug)]
enum TodoError {
    CommandError,
    TaskNotFound,
}

fn main() -> Result<(), TodoError> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd: Command = parse_command(args)?;
    let mut task_list: TodoList = Default::default();
    run(cmd, &mut task_list)?;
    // println!("{:?}", cmd);
    Ok(())
}

fn run(cmd: Command, todo_list: &mut TodoList) -> Result<(), TodoError> {
    match cmd {
        Command::Add { text } => {
            todo_list.add(text);
            Ok(())
        }
        Command::List => {
            println!("Printing tasks...");
            todo_list.print_list();
            Ok(())
        }
        Command::Done { id } => {
            println!("Marking {id} as done...");
            match todo_list.mark_done(id) {
                Ok(_) => println!("Task id {} marked as done!", id),
                Err(_) => println!("Task id {} not found", id),
            }
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
            let id = args_iter.next().ok_or(TodoError::CommandError)?;
            // TODO: change .unwrap()
            let id: u32 = id.parse().unwrap();

            Ok(Command::Done { id })
        }
        "list" => Ok(Command::List),
        "help" => Ok(Command::Help),
        _ => Err(TodoError::CommandError),
    }
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
    assert!(res.done);
    Ok(())
}
