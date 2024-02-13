fn main() {
    for i in 1..13 {
        println!("On the {} Day of Christmas my true love gave to me...", i);
        twelveDays(i);
    }
}

fn twelveDays(day: i32) {
    if day == 1 {
        println!("A partridge in a pear tree");
    }
    if day == 2 {
        println!("Two turtle doves and");
        twelveDays(day - 1);
    }
    if day == 3 {
        println!("Three French Hens");
        twelveDays(day - 1);
    }
    if day == 4 {
        println!("Four Calling Birds");
        twelveDays(day - 1);
    }
    if day == 5 {
        println!("FiiIIvee...GooolDen Rings");
        twelveDays(day - 1);
    }
    if day == 6 {
        println!("Six Geese a-laying");
        twelveDays(day - 1);
    }
    if day == 7 {
        println!("Seven Swans a-swimming");
        twelveDays(day - 1);
    }
    if day == 8 {
        println!("Eight Maids a-milking");
        twelveDays(day - 1);
    }
    if day == 9 {
        println!("Nine Ladies Dancing");
        twelveDays(day - 1);
    }
    if day == 10 {
        println!("Ten Lords a-leaping");
        twelveDays(day - 1);
    }
    if day == 11 {
        println!("Eleven Pipers Piping");
        twelveDays(day - 1);
    }
    if day == 12 {
        println!("Twelve Drummers Drumming");
        twelveDays(day - 1);
    }

}
