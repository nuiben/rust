fn main() {
    let mut x = 1;
    loop {
        println!("{x}");
        x += 1;

        if x > 10 {
            break;
        }
    }

    for i in 1..11 {
        println!("{i}");
    }

    let mut z = 1;
    while z <= 10 {
        println!("{z}");
        z += 1;
    }
}
