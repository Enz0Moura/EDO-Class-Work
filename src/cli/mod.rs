pub mod command;
pub struct CLI;

impl CLI {
    pub fn clear() {
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn pause() {
        use std::io::{self, Read};

        println!("\nPress ENTER to continue...");
        let _ = io::stdin().read(&mut [0u8]).unwrap();
    }
    
    pub fn prompt() -> String {
        use std::io::{self, Write};

        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        input.trim().to_string()
    }


    pub fn menu() {
        println!("==== MENU ====");
        println!("1 - Newton Analytical");
        println!("2 - Euler Newton");
        println!("3 - Logistic Data");
        println!("4 - Euler Logistic");
        println!("5 - Learned Model");
        println!("0 - Exit");
    }


}

