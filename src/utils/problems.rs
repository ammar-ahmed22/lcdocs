use crate::config;
use colored::*;
use std::io::Write;

// TODO move to utils/problem
pub static PROBLEMS_JSON: &str = "./problems.json";

// TODO move to utils/problem
#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Problems {
    pub easy: std::collections::HashMap<String, i32>,
    pub medium: std::collections::HashMap<String, i32>,
    pub hard: std::collections::HashMap<String, i32>,
}

impl Problems {
    pub fn write_to_file(&self, path: &str) -> anyhow::Result<()> {
        let mut file = std::fs::File::create(path)?;
        let json = serde_json::to_string_pretty(self)?;
        write!(file, "{}", json)?;
        Ok(())
    }

    pub fn read_from_file(path: &str) -> anyhow::Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let problems: Self = serde_json::from_reader(reader)?;
        Ok(problems)
    }

    pub fn extract(path: &str) -> anyhow::Result<Self> {
        let problems: Self;
        let std_path = std::path::Path::new(path);
        if std_path.exists() {
            problems = Self::read_from_file(path)?;
        } else {
            problems = Self::default();
        }
        Ok(problems)
    }

    pub fn find_problem(&self, problem_name: &str) -> Option<(config::Difficulty, String)> {
        for difficulty in &config::ALL_DIFFICULTY {
            match difficulty {
                &config::Difficulty::Easy => {
                    if self.easy.contains_key(problem_name) {
                        return Some((config::Difficulty::Easy, String::from(problem_name)));
                    }
                }
                &config::Difficulty::Medium => {
                    if self.medium.contains_key(problem_name) {
                        return Some((config::Difficulty::Medium, String::from(problem_name)));
                    }
                }
                &config::Difficulty::Hard => {
                    if self.hard.contains_key(problem_name) {
                        return Some((config::Difficulty::Hard, String::from(problem_name)));
                    }
                }
            }
        }

        return None;
    }
}

// TODO move to utils/problem
pub fn find_max_problem_idx(problems: &std::collections::HashMap<String, i32>) -> i32 {
    let mut max = 0;
    for (_key, value) in problems.iter() {
        if value > &max {
            max = *value;
        }
    }
    return max;
}

// TODO move to utils/problem
pub fn find_problem_difficulty(
    problem_name: &str,
    difficulty: Option<config::Difficulty>,
) -> anyhow::Result<config::Difficulty> {
    let problems = Problems::extract(PROBLEMS_JSON)?;
    match difficulty {
        Some(diff) => Ok(diff),
        None => {
            let result = problems.find_problem(&problem_name);
            match result {
                Some((diff, _)) => {
                    log::info!(
                        "Found '{}' problem '{}'",
                        format!("{}", diff).magenta(),
                        problem_name.magenta()
                    );
                    Ok(diff)
                }
                None => Err(anyhow::anyhow!(
                    "Cannot find problem '{}'",
                    problem_name.magenta()
                )),
            }
        }
    }
}
