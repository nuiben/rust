fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let spaces = "   ";
    {
        let spaces = spaces.len();
        println!("The value of spaces is: {spaces}");
    }
    println!("The value of spaces is: {spaces}");
    let spaces = "s p a c e s";

    println!("The value of spaces is: {spaces}");

    // let number: u8 = 257;
    // let other_number: u8 = 0;

    // println!("This is an overflow: {}", number + other_number);
}
