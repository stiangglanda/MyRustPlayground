use std::io;
use std::io::Write;

struct Task {
    description: String,
    completed: bool,
}

struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: String) {
        let task = Task {
            description,
            completed: false,
        };
        self.tasks.push(task);
    }

    fn mark_task_as_completed(&mut self, task_index: usize) {
        if task_index < self.tasks.len() {
            self.tasks[task_index].completed = true;
        }
    }

    fn list_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.completed { "[X]" } else { "[ ]" };
            println!("{} {} - {}", index, status, task.description);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        print_menu();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let command = input.trim().to_lowercase();

        match command.as_str() {
            "add" => {
                println!("Enter the description of the task:");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read line");
                todo_list.add_task(description.trim().to_string());
            }
            "list" => {
                println!("Tasks:");
                todo_list.list_tasks();
            }
            "complete" => {
                println!("Enter the index of the task you want to mark as completed:");
                let mut index_input = String::new();
                io::stdin().read_line(&mut index_input).expect("Failed to read line");
                if let Ok(task_index) = index_input.trim().parse::<usize>() {
                    todo_list.mark_task_as_completed(task_index);
                } else {
                    println!("Invalid index.");
                }
            }
            "quit" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid command. Please try again.");
            }
        }
    }
}

fn print_menu() {
    println!("Todo List Menu:");
    println!("  - Add: Add a new task");
    println!("  - List: List all tasks");
    println!("  - Complete: Mark a task as completed");
    println!("  - Quit: Exit the program");
    print!("Enter your command: ");
    io::stdout().flush().expect("Failed to flush");
}