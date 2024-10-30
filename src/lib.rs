use std::process;

use terminal_size::{terminal_size, Height, Width};

pub mod minipgb {
    // use ansi_term::Color;
    // use terminal_size::{terminal_size, Height, Width};
    //
    use crate::draw;

    pub struct MiniPgb {
        pub value: usize,
    }
    pub fn new() -> MiniPgb {
        MiniPgb { value: 0 }
    }
    impl MiniPgb {
        pub fn start(&self) {
            draw();
            // let size = terminal_size();
            // if let Some((Width(w), Height(h))) = size {
            //     // println!("Your terminal is {} cols wide and {} lines tall", w, h);
            //     let val_progress = "";
            //     let mut text = "".to_string();
            //     let prefix = "download file : test.png";
            //     let template = format!("{}[]", prefix);
            //     for _ in val_progress.len()..(w as usize) - template.len() {
            //         text.insert(0, ' ');
            //     }
            //     println!("{}[{}]", prefix, text)
            // } else {
            //     println!("Unable to get terminal size");
            // }

            // println!("{}", Color::Red.paint("This text is red!"));
        }
        pub fn inc(&mut self, n: usize) {
            self.value += n;
        }

        pub fn finish(self) {}
    }
}

fn draw() {
    let size = terminal_size();
    if let Some((Width(w), Height(_h))) = size {
        // println!("Your terminal is {} cols wide and {} lines tall", w, h);
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
        process::exit(0x0100);
    }
}
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
