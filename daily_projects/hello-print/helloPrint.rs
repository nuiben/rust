fn main() {
  println!("{} days", 44);
  println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

  println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

  println!("Base 10:               {}",   28);
  println!("Base 2:                {:b}",   28);
  println!("Base 8 (octal):        {:o}", 28);
  println!("Base 16 (hexadecimal): {:x}", 28); // 1c
  println!("Base 16 (hexadecimal): {:X}", 28); // 1C
  println!("{number:>5}", number = 1);
  println!("{number:0>5}", number = 1); // 00001
  println!("{number:0<5}", number = 1); // 10000
  println!("{number:0>width$}", number=1, width=5);
  print!("HI THERE");
  print!("HI THERE\n");
  println!("My name is {0}, {1} {0}", "Bond", "Jim");

  let number: f64 = 151.0;
  let width: usize = 5;
  println!("{number:>width$}");
}
