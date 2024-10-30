pub mod minipgb {
    pub struct MiniPgb {
        pub value: i32,
    }
    pub fn new() -> MiniPgb {
        MiniPgb { value: 0 }
    }
    impl MiniPgb {
        fn start() {}
        fn finish() {}
    }
}

fn main() {
    println!("hello, world");
}
