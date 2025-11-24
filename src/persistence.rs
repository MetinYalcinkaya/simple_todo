use crate::cli::TodoError;
use crate::model::TodoList;
use std::fs;

pub const PATH: &str = "src/todo.json";

pub fn load_todo_list(path: &str) -> TodoList {
    match fs::read_to_string(path) {
        Ok(contents) => serde_json::from_str::<TodoList>(&contents).unwrap_or_default(),
        Err(_) => TodoList::default(),
    }
}

pub fn save_todo_list(path: &str, list: &TodoList) -> Result<(), TodoError> {
    let json = serde_json::to_string_pretty(list).map_err(|_| TodoError::SaveError)?;
    std::fs::write(path, json).map_err(|_| TodoError::SaveError)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{execute_command, parse_command};

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
        execute_command(cmd, &mut task_list)?;
        let _ = save_todo_list(path, &task_list);
        let saved = load_todo_list(path);
        assert_eq!(1, saved.tasks.len());
        // cleanup
        let _ = std::fs::remove_file(path);
        Ok(())
    }
}
