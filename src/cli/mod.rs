pub mod command;

use crate::exercises::Exercises;
use command::{LogisticCommand, MainCommand, NewtonCommand};

pub struct CLI;

impl CLI {
    pub fn clear() {
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn pause() {
        use std::io;

        println!("\nPress ENTER to continue...");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }

    pub fn success_and_wait() {
        println!("\nExecution completed successfully.");
        Self::pause();
    }

    pub fn prompt() -> String {
        use std::io::{self, Write};

        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        input.trim().to_string()
    }

    pub fn main_menu() {
        loop {
            Self::clear();

            println!("==== MAIN MENU ====");
            println!("1 - Problem 1 (Newton)");
            println!("3 - Problem 3 (Logistic)");
            println!("0 - Exit");

            let input = Self::prompt();
            let command = MainCommand::from_input(&input);

            match command {
                MainCommand::Problem1 => Self::newton_menu(),
                MainCommand::Problem3 => Self::logistic_menu(),
                MainCommand::Exit => {
                    println!("Goodbye!");
                    break;
                }
                MainCommand::Invalid => {
                    println!("Invalid option");
                    Self::pause();
                }
            }
        }
    }

    fn newton_menu() {
        loop {
            Self::clear();

            println!("==== PROBLEM 1: NEWTON ====");
            println!("1 - Analytical");
            println!("2 - Euler");
            println!("3 - Compare");
            println!("0 - Back");

            let input = Self::prompt();
            let command = NewtonCommand::from_input(&input);

            match command {
                NewtonCommand::Analytical => {
                    Exercises::test_newton().unwrap();
                    Self::success_and_wait();
                }
                NewtonCommand::Euler => {
                    Exercises::test_euler_newton();
                    Self::success_and_wait();
                }
                NewtonCommand::Compare => {
                    Exercises::compare_analytical_vs_euler();
                    Self::success_and_wait();
                }
                NewtonCommand::Back => break,
                NewtonCommand::Invalid => {
                    println!("Invalid option");
                    Self::pause();
                }
            }
        }
    }

    fn logistic_menu() {
        loop {
            Self::clear();

            println!("==== PROBLEM 3: LOGISTIC ====");
            println!("1 - Generate Data");
            println!("2 - Euler");
            println!("3 - Learned Model");
            println!("4 - Learned Model No Noise");
            println!("0 - Back");

            let input = Self::prompt();
            let command = LogisticCommand::from_input(&input);

            match command {
                LogisticCommand::Data => {
                    Exercises::generate_experimental_data_logistic_model();
                    Self::success_and_wait();
                }
                LogisticCommand::Euler => {
                    Exercises::test_euler_logistic_model();
                    Self::success_and_wait();
                }
                LogisticCommand::Learned => {
                    Exercises::test_learned_model();
                    Self::success_and_wait();
                }
                LogisticCommand::LearnedNoNoise => {
                    Exercises::test_learned_model_no_noise();
                    Self::success_and_wait();
                }
                LogisticCommand::Back => break,
                LogisticCommand::Invalid => {
                    println!("Invalid option");
                    Self::pause();
                }
            }
        }
    }
}
