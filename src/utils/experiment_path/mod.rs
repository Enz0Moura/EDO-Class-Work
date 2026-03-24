use std::fs;
use std::path::Path;

pub struct ExperimentPath {
    base: String,
}

impl ExperimentPath {
    pub fn new(problem: &str, section: &str) -> Self {
        let base = format!("outputs/{}/{}", problem, section);

        if !Path::new(&base).exists() {
            fs::create_dir_all(&base).unwrap();
        }

        Self { base }
    }

    pub fn file(&self, filename: &str) -> String {
        format!("{}/{}", self.base, filename)
    }
}
