use crate::exercises::Exercises;

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
    pub fn execute(&self) {
        match self {
            Command::Newton => {
                Exercises::test_newton().unwrap();
            }
            Command::EulerNewton => {
                Exercises::test_euler_newton();
            }
            Command::LogisticData => {
                Exercises::generate_experimental_data_logistic_model();
            }
            Command::EulerLogistic => {
                Exercises::test_euler_logistic_model();
            }
            Command::Learned => {
                Exercises::test_learned_model();
            }
            Command::Exit => {}
            Command::Invalid => {
                println!("Invalid option");
            }
        }
    }
}