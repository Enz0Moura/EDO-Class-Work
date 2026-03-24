use crate::exercises::Exercises;
use std::io::{self, Read};

pub enum MainCommand {
    Problem1,
    Problem3,
    Exit,
    Invalid,
}

pub enum NewtonCommand {
    Analytical,
    Euler,
    Compare,
    Back,
    Invalid,
}

pub enum LogisticCommand {
    Data,
    Euler,
    Learned,
    LearnedNoNoise,
    Back,
    Invalid,
}


impl MainCommand {
    pub fn from_input(input: &str) -> Self {
        match input {
            "1" => MainCommand::Problem1,
            "3" => MainCommand::Problem3,
            "0" => MainCommand::Exit,
            _ => MainCommand::Invalid,
        }
    }
}

impl NewtonCommand {
    pub fn from_input(input: &str) -> Self {
        match input {
            "1" => NewtonCommand::Analytical,
            "2" => NewtonCommand::Euler,
            "3" => NewtonCommand::Compare,
            "0" => NewtonCommand::Back,
            _ => NewtonCommand::Invalid,
        }
    }
}

impl LogisticCommand {
    pub fn from_input(input: &str) -> Self {
        match input {
            "1" => LogisticCommand::Data,
            "2" => LogisticCommand::Euler,
            "3" => LogisticCommand::Learned,
            "4" => LogisticCommand::LearnedNoNoise,
            "0" => LogisticCommand::Back,
            _ => LogisticCommand::Invalid,
        }
    }
}

