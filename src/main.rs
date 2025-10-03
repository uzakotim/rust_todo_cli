use inquire::{Select, Text};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    text: String,
    completed: bool,
}

fn load_todos(path: &str) -> Vec<Todo> {
    if Path::new(path).exists() {
        let data = fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn save_todos(path: &str, todos: &Vec<Todo>) {
    if let Ok(json) = serde_json::to_string_pretty(todos) {
        let _ = fs::write(path, json);
    }
}

fn main() {
    let file_path = "todos.json";
    let mut todos: Vec<Todo> = load_todos(file_path);

    loop {
        let options = vec![
            "Make todo",
            "View all todos",
            "Mark todo completed/uncompleted",
            "Delete a todo",
            "Exit",
        ];

        let choice = Select::new("Choose an action:", options.clone()).prompt();

        match choice {
            Ok(selection) => match selection {
                "Make todo" => {
                    let todo = Text::new("Enter your todo:").prompt();
                    match todo {
                        Ok(t) => {
                            todos.push(Todo {
                                text: t,
                                completed: false,
                            });
                            println!("Added a new todo!");
                            save_todos(file_path, &todos);
                        }
                        Err(_) => println!("Failed to read todo."),
                    }
                }
                "View all todos" => {
                    if todos.is_empty() {
                        println!("No todos yet.");
                    } else {
                        println!("Your todos:");
                        for (i, todo) in todos.iter().enumerate() {
                            let status = if todo.completed { "✅" } else { "⬜" };
                            println!("{}. {} {}", i + 1, status, todo.text);
                        }
                    }
                }
                "Mark todo completed/uncompleted" => {
                    if todos.is_empty() {
                        println!("No todos to mark.");
                    } else {
                        let display: Vec<String> = todos
                            .iter()
                            .enumerate()
                            .map(|(i, t)| {
                                let status = if t.completed { "✅" } else { "⬜" };
                                format!("{}. {} {}", i + 1, status, t.text)
                            })
                            .collect();

                        let choice = Select::new("Select a todo to toggle:", display).prompt();

                        if let Ok(selected) = choice {
                            if let Some(index) =
                                selected.split('.').next().and_then(|n| n.parse::<usize>().ok())
                            {
                                let idx = index - 1;
                                todos[idx].completed = !todos[idx].completed;
                                println!(
                                    "Toggled: {} → {}",
                                    todos[idx].text,
                                    if todos[idx].completed {
                                        "✅ completed"
                                    } else {
                                        "⬜ uncompleted"
                                    }
                                );
                                save_todos(file_path, &todos);
                            }
                        }
                    }
                }
                "Delete a todo" => {
                    if todos.is_empty() {
                        println!("No todos to delete.");
                    } else {
                        let display: Vec<String> = todos
                            .iter()
                            .enumerate()
                            .map(|(i, t)| {
                                let status = if t.completed { "✅" } else { "⬜" };
                                format!("{}. {} {}", i + 1, status, t.text)
                            })
                            .collect();

                        let choice = Select::new("Select a todo to delete:", display).prompt();

                        if let Ok(selected) = choice {
                            if let Some(index) =
                                selected.split('.').next().and_then(|n| n.parse::<usize>().ok())
                            {
                                let idx = index - 1;
                                println!("Deleted: {}", todos[idx].text);
                                todos.remove(idx);
                                save_todos(file_path, &todos);
                            }
                        }
                    }
                }
                "Exit" => {
                    println!("Saving todos... Goodbye!");
                    save_todos(file_path, &todos);
                    break;
                }
                _ => {}
            },
            Err(_) => {
                println!("There was an error reading your input.");
                break;
            }
        }
    }
}
