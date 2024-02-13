use std::io::{self, Write};

fn main() {
    let mut habits = vec![];
    loop {
        print_menu();
        let choice = get_choice();
        match choice {
            1 => add_habit(&mut habits),
            2 => list_habits(&habits),
            3 => break,
            _ => println!("Invalid choice"),
        }
    }
}

fn print_menu() {
    println!("Habit Tracker");
    println!("1. Add Habit");
    println!("2. List Habits");
    println!("3. Quit");
}

fn get_choice() -> u32 {
    let mut choice = String::new();
    print!("Enter your choice: ");
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    choice.trim().parse().expect("Invalid choice")
}

fn add_habit(habits: &mut Vec<String>) {
    let mut habit = String::new();
    print!("Enter habit name: ");
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut habit).expect("Failed to read input");
    habits.push(habit.trim().to_string());
    println!("Habit added successfully");
}

fn list_habits(habits: &Vec<String>) {
    if habits.is_empty() {
        println!("No habits found");
    } else {
        println!("Habits:");
        for (i, habit) in habits.iter().enumerate() {
            println!("{}. {}", i + 1, habit);
        }
    }
}

