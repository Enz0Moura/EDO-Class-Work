mod exercises;
mod solvers;
mod utils;
mod cli;
use cli::CLI;
use cli::command::Command;

fn main() {
    loop {
        CLI::clear();
        CLI::menu();

        let input = CLI::prompt();

        let command = Command::from_input(&input);

        match command {
            Command::Exit => {
                println!("Exiting...");
                break;
            },
            _ => {
                command.execute();
                CLI::pause();
            }
        }
    }
}