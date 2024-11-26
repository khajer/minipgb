use std::{thread};

fn main() {
    println!("hello, world");
    let mut pg = minipgb::minipgb::new();
    pg.start();
    
    for _i in 1..101 {
        pg.inc(1);
        // thread::sleep_ms(100);
        

    }
}
