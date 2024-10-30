fn main() {
    println!("hello, world");
    let mut pg = minipgb::minipgb::new();
    pg.start();
    for _i in 1..100 {
        pg.inc(1);
    }
}
