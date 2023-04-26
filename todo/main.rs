use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let mut tasks: Vec<String> = Vec::new();

    //Load Tasks from file
    let file = File::open("tasks.txt");
    if let Ok(file) = file {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(task) = line {
                tasks.push(task);
            }
        }
    }

    loop {
        println!("Current Tasks:");
        for (i, task) in tasks.iter().enumerate() {
            println!("{}: {}", i, task);
        }
        println!("Enter a command: add, remove, or quit");

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        match command.trim() {
            "add" => {
                println!("Enter a task:");
                let mut task = String::new();
                io::stdin()
                    .read_line(&mut task)
                    .expect("Failed to read line");
                tasks.push(task.trim().to_string());
                println!("Task added.");
            }
            "remove" => {
                println!("Enter the index of the task to remove:");
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to read line");
                let index: usize = match index.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                if index < tasks.len() {
                    tasks.remove(index);
                    println!("Task removed.");
                } else {
                    println!("Invalid index.");
                }
            }
            "quit" => {
                //Save tasks to file
                let mut file = File::create("tasks.txt").expect("Failed to create file");
                for task in &tasks {
                    file.write_all(task.as_bytes())
                        .expect("Failed to write to file");
                    file.write_all(b"\n").expect("Failed to write to file");
                }
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid command.");
            }
        }
    }
}

