use std::io::{self, Write};

struct Character {
    name: String,
    class: Class,
    level: u32,
    health: u32,
}

enum Class {
    Warrior,
    Mage,
    Rogue,
    Bard,
    Cleric,
}

impl Class {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "Warrior" => Some(Class::Warrior),
            "Mage" => Some(Class::Mage),
            "Rogue" => Some(Class::Rogue),
            "Bard" => Some(Class::Bard),
            "Cleric" => Some(Class::Cleric),
            _ => None,
        }
    }
}

fn main() {
    let mut characters = vec![];
    loop {
        print_menu();
        let choice = get_choice();
        match choice {
            1 => add_character(&mut characters),
            2 => list_characters(&characters),
            3 => break,
            _ => println!("Invalid choice"),
        }
    }
}

fn print_menu() {
    println!("RPG Character Creator");
    println!("1. Add Character");
    println!("2. List Characters");
    println!("3. Quit");
}

fn get_choice() -> u32 {
    let mut choice = String::new();
    print!("Enter your choice: ");
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    choice.trim().parse().expect("Invalid choice")
}

fn add_character(characters: &mut Vec<Character>) {
    let mut name = String::new();
    print!("Enter character name: ");
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut name).expect("Failed to read input");

    let class_str = get_class_from_user();
    let class = Class::from_str(&class_str).unwrap_or_else(|| {
        println!("Invalid class");
        std::process::exit(1);
    });

    let level = get_level_from_user();
    let health = get_health_from_user();

    let character = Character {
        name: name.trim().to_string(),
        class,
        level,
        health,
    };
    characters.push(character);
    println!("Character added successfully");
}

fn get_class_from_user() -> String {
    let mut class = String::new();
    print!("Enter character class (Warrior, Mage, Rogue, Bard, Cleric): ");
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut class).expect("Failed to read input");
    class.trim().to_string()
}

fn get_level_from_user() -> u32 {
    let mut level = String::new();
    print!("Enter character level: ");
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut level).expect("Failed to read input");
    level.trim().parse().expect("Invalid level")
}

fn get_health_from_user() -> u32 {
    let mut health = String::new();
    print!("Enter character health: ");
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut health).expect("Failed to read input");
    health.trim().parse().expect("Invalid health")
}

fn list_characters(characters: &Vec<Character>) {
    if characters.is_empty() {
        println!("No characters found");
    } else {
        println!("Characters:");
        for i in characters {
            println!("Name: {}, Class: , Level: {}, Health: {}", i.name, i.level, i.health);
        }
    }
}
