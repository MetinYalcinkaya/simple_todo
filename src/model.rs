use crate::cli::TodoError;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct Task {
    pub id: u32,
    pub text: String,
    pub done: bool,
    pub priority: Priority,
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if self.done { "[x]" } else { "[ ]" };
        write!(f, "{status} {} {}: {}", self.priority, self.id, self.text)
    }
}

#[derive(Deserialize, Serialize)]
pub struct TodoList {
    pub tasks: Vec<Task>,
    pub next_id: u32,
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
    pub fn add(&mut self, text: String) -> &Task {
        let id = self.next_id;
        self.tasks.push(Task {
            id,
            text,
            done: false,
            priority: Priority::default(),
        });
        self.next_id = id + 1;
        self.tasks.last().unwrap()
    }

    pub fn print_list(&self) {
        for task in &self.tasks {
            println!("{task}");
        }
    }

    pub fn mark_done(&mut self, id: u32) -> Result<&Task, TodoError> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.done = true;
            Ok(task)
        } else {
            Err(TodoError::TaskNotFound)
        }
    }

    pub fn print_done(&self) {
        for task in self.tasks.iter().filter(|t| t.done) {
            println!("{task}");
        }
    }

    pub fn print_todo(&self) {
        for task in self.tasks.iter().filter(|t| !t.done) {
            println!("{task}");
        }
    }

    pub fn print_by_priority(&self, priority: Priority) {
        for task in self.tasks.iter().filter(|t| t.priority == priority) {
            println!("{task}");
        }
    }

    pub fn set_priority(&mut self, id: u32, priority: Priority) -> Result<&Task, TodoError> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.priority = priority;
            Ok(task)
        } else {
            Err(TodoError::TaskNotFound)
        }
    }
}

#[derive(Clone, Copy, Default, Deserialize, Serialize, Debug, PartialEq)]
pub enum Priority {
    #[default]
    Low,
    Medium,
    High,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "(L)"),
            Priority::Medium => write!(f, "(M)"),
            Priority::High => write!(f, "(H)"),
        }
    }
}

impl std::str::FromStr for Priority {
    type Err = TodoError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("low") || s.eq_ignore_ascii_case("l") {
            Ok(Priority::Low)
        } else if s.eq_ignore_ascii_case("medium")
            || s.eq_ignore_ascii_case("med")
            || s.eq_ignore_ascii_case("m")
        {
            Ok(Priority::Medium)
        } else if s.eq_ignore_ascii_case("high") || s.eq_ignore_ascii_case("h") {
            Ok(Priority::High)
        } else {
            Err(TodoError::PriorityError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::Command;
    use crate::execute_command;
    use crate::model::Priority;

    #[test]
    fn test_priority_counts() -> Result<(), TodoError> {
        let commands = [
            Command::Add {
                text: String::from("eat mango"),
            },
            Command::Add {
                text: String::from("walk dog"),
            },
            Command::Add {
                text: String::from("pet ferris"),
            },
        ];

        let mut task_list: TodoList = Default::default();

        for command in commands {
            execute_command(command, &mut task_list)?;
        }

        task_list.set_priority(2, Priority::Medium)?;
        task_list.set_priority(3, Priority::High)?;

        assert_eq!(
            1,
            task_list
                .tasks
                .iter()
                .filter(|t| t.priority == Priority::Low)
                .count()
        );

        assert_eq!(
            1,
            task_list
                .tasks
                .iter()
                .filter(|t| t.priority == Priority::Medium)
                .count()
        );

        assert_eq!(
            1,
            task_list
                .tasks
                .iter()
                .filter(|t| t.priority == Priority::High)
                .count()
        );

        Ok(())
    }
}
