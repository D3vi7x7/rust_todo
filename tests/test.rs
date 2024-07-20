

#[path = "../src/main.rs"]
mod main;

use std::collections::hash_map;
use main::Todo;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Todo {
        let mut todo = Todo::new().unwrap();
        todo.insert("test_task".to_string());
        todo.insert("another_task".to_string());
        todo
    }

    #[test]
    fn test_insert() {
        let mut todo = setup();
        todo.insert("new_task".to_string());
        assert!(todo.get_map().contains_key("new_task"));
    }

    #[test]
    fn test_complete() {
        let mut todo = setup();
        todo.complete(&"test_task".to_string());
        assert_eq!(todo.get_map().get("test_task"), Some(&false));
    }

    #[test]
    fn test_show() {
        let todo = setup();
        todo.show();
        // We can't directly test output here, but we can verify the state
        assert!(todo.get_map().contains_key("test_task"));
    }

    #[test]
    fn test_remove() {
        let mut todo = setup();
        todo.remove(&"test_task".to_string());
        assert!(!todo.get_map().contains_key("test_task"));
    }

    #[test]
    fn test_new() {
        let todo = Todo::new().unwrap();
        assert!(todo.get_map().is_empty());
    }

    #[test]
    fn test_save_and_load() {
        let mut todo = setup();
        todo.save().unwrap();
        let loaded_todo = Todo::new().unwrap();
        assert!(loaded_todo.get_map().contains_key("test_task"));
    }
}