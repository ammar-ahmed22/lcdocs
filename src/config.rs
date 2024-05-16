use colored::*;
use std::io::Write;
use clap::Parser;

#[derive(Debug, clap::Parser)]
#[clap(name = "leetcode_notes", about = "A CLI to help me organize my leetcode solutions!")]
pub struct CLIArgs {
  #[clap(subcommand)]
  command: Commands
}

#[derive(clap::Subcommand, Debug, serde::Serialize, serde::Deserialize)]
pub enum Commands {
  /// Create a problem
  Create {
    /// The name of the problem
    problem: String,
    /// The difficulty of the problem
    #[clap(short, long, value_enum)]
    difficulty: Difficulty
  },
  /// Run a problem
  Run {
    /// The name of the problem
    problem: String,
    /// The difficulty of the problem
    #[clap(short, long, value_enum)]
    difficulty: Difficulty,
  },
  /// Remove a problem
  Remove {
    /// The name of the problem
    problem: String,
    /// The difficulty of the problem
    #[clap(short, long, value_enum)]
    difficulty: Difficulty,
  }
}

#[derive(clap::ValueEnum, Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum Difficulty {
  Easy,
  Medium,
  Hard
}

impl std::fmt::Display for Difficulty {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let difficulty_str = match self {
          Difficulty::Easy => "easy",
          Difficulty::Medium => "medium",
          Difficulty::Hard => "hard",
      };
      write!(f, "{}", difficulty_str)
  }
}

pub fn initialize_logger() {
  env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
      .format(|buf, record| {
          let colored_level = match record.level() {
              log::Level::Info => "INFO".blue(),
              log::Level::Debug => "DEBUG".green(),
              log::Level::Warn => "WARN".yellow(),
              log::Level::Error => "ERROR".red(),
              log::Level::Trace => "TRACE".magenta()
          };
          writeln!(buf, "[{}]: {}", colored_level, record.args())
      })
      .init();
}

pub fn parse_command() -> Commands {
  CLIArgs::parse().command
}