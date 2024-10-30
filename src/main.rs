pub mod minipgb {
    use ansi_term::Color;
    use terminal_size::{terminal_size, Height, Width};

    pub struct MiniPgb {
        pub value: i32,
    }
    pub fn new() -> MiniPgb {
        MiniPgb { value: 0 }
    }
    impl MiniPgb {
        pub fn start(self) {
            let size = terminal_size();
            if let Some((Width(w), Height(h))) = size {
                println!("Your terminal is {} cols wide and {} lines tall", w, h);
                let val_progress = "";
                let mut text = "".to_string();
                let prefix = "download file : test.png";
                let template = format!("{}[]", prefix);
                for _ in val_progress.len()..(w as usize) - template.len() {
                    text.insert(0, ' ');
                }
                println!("{}[{}]", prefix, text)
            } else {
                println!("Unable to get terminal size");
            }

            println!("{}", Color::Red.paint("This text is red!"));
        }
        pub fn finish(self) {}
    }
}

fn main() {
    println!("hello, world");
    let pg = minipgb::new();
    pg.start();
}
