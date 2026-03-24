use crate::exercises::Exercises;
use std::io::{self, Read};

pub enum Command {
    Newton,
    EulerNewton,
    LogisticData,
    EulerLogistic,
    Learned,
    Exit,
    Invalid,
}

impl Command {
    pub fn from_input(input: &str) -> Self {
        match input {
            "1" => Command::Newton,
            "2" => Command::EulerNewton,
            "3" => Command::LogisticData,
            "4" => Command::EulerLogistic,
            "5" => Command::Learned,
            "0" => Command::Exit,
            _ => Command::Invalid,
        }
    }
}

impl Command {
    fn success_and_wait() {
        println!("\nExecution completed successfully.");
        println!("Press ENTER to return to menu...");

        let _ = io::stdin().read(&mut [0u8]).unwrap();
    }
    pub fn execute(&self) {
        match self {
            Command::Newton => {
                Exercises::test_newton().unwrap();
                Self::success_and_wait();
            }
            Command::EulerNewton => {
                Exercises::test_euler_newton();
                Self::success_and_wait();
            }
            Command::LogisticData => {
                Exercises::generate_experimental_data_logistic_model();
                Self::success_and_wait();
            }
            Command::EulerLogistic => {
                Exercises::test_euler_logistic_model();
                Self::success_and_wait();
            }
            Command::Learned => {
                Exercises::test_learned_model();
                Self::success_and_wait();
            }
            Command::Exit => {}
            Command::Invalid => {
                println!("Invalid option");
            }
        }
    }
}