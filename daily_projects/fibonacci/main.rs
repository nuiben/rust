use std::io;

fn main() {
    println!("_______________");
    println!("Fibonacci Time!");
    println!("This algorithm takes a number N, and returns it as the sum of");
    println!("the previous two values before it.");
    println!("The sequence looks like this:");
    println!("1, 1, 2, 3, 5, 8, 11, 19...");
    println!("_______________");
    println!("Now you enter a number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number: u32 = input.trim().parse::<u32>().unwrap();

    println!("{}", fibonacci(number));
}

fn fibonacci(num: u32) -> u64 {
    if num == 0 {
        return 0
    }
    if num <= 2 {
        return 1
    }

    return fibonacci(num - 1) + fibonacci(num - 2)
}

#[test]
fn one() {
    assert_eq!(fibonacci(1), 1);
}

#[test]
fn two() {
    assert_eq!(fibonacci(2), 1);
}

#[test]
fn five() {
    assert_eq!(fibonacci(5), 5);
}

#[test]
fn ten() {
    assert_eq!(fibonacci(10), 55);
}

#[test]
fn fourteen() {
    assert_eq!(fibonacci(14), 377);
}

#[test]
fn fortyTwo() {
    assert_eq!(fibonacci(42), 267914296);
}
