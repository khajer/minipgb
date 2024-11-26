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
            draw(self.value);            
        }
        pub fn inc(&mut self, n: usize) {
            if self.value < 100 {
                self.value += n;
            }
            draw(self.value);
        }

        pub fn finish(self) {}
    }
}

fn draw(pg_value: usize) {
    let size = terminal_size();
    if let Some((Width(w), Height(_h))) = size {                
        let prefix = "Download File > download.png ";        
        let len = prefix.len();        
        let tatal_txt_100 = w as usize - prefix.len() - "[]".len();        
        let pg  = (pg_value as f32/100.0) * tatal_txt_100 as f32;
        let space = tatal_txt_100 - pg as usize;        
        let  text = format!("{}{}", "#".repeat(pg as usize), " ".repeat(space));        
        print!("\r{}[{}]", prefix, text);        
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
