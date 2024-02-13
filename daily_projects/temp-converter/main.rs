use std::io;

fn main() {
    // Get the number of the Temperature
    println!("Enter a temperature to convert: ");
    let mut temp_input = String::new();
    io::stdin().read_line(&mut temp_input).expect("Failed to read line");
    let temp: f32 = temp_input.trim().parse::<f32>().unwrap();


    // Establish if the User input a Temperature of Celsius or Fahrenheit
    println!("Enter Celsius [C] or Fahrenheit [F]");
    let mut temp_type = String::new();
    io::stdin().read_line(&mut temp_type).expect("Failed to read line");
    let mut temp_type: &str = temp_type.as_str().trim();

    let mut base_temperature: f32 = 0.0;

    if temp_type == "c" || temp_type == "C" {
        base_temperature = 9.0 * (temp + 32.0) / 5.0;
        temp_type = "Fahrenheit";

    } else if temp_type == "f" || temp_type == "F" {
        base_temperature = 5.0 * (temp - 32.0) / 9.0;
        temp_type = "Celsius";

    } else {
        println!("Invalid Input");
        temp_type = "";
    }

    println!("The temperature in {} is {:.2}", temp_type, base_temperature);
}
