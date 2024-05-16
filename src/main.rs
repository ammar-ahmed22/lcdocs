// pub mod config;
use anyhow::Context;
use colored::*;
use leetcode_notes::config;

fn try_main() -> anyhow::Result<()> {
    config::initialize_logger();
    let cmd = config::parse_command();
    match cmd {
        config::Commands::Create { problem, difficulty  } => {
            let parsed_name = leetcode_notes::to_snake_case(problem.as_str());
            leetcode_notes::create_problem_pkg(&parsed_name, difficulty.clone())
                .with_context(|| format!("trying to create '{}'", problem.blue()))?;
            leetcode_notes::create_problem_docs(&parsed_name, difficulty.clone())?;
            Ok(())
        },
        config::Commands::Run { problem: _ } => {
            todo!("Implement the run command!")
        }
    }
}

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}: {}","Error".red(), err);
        if err.chain().len() > 1 {
            eprintln!("\n{}", "Caused by:".yellow());
            err.chain().skip(1).enumerate().for_each(|(i, cause)| eprintln!("\t{}: {}", i, cause));
        }
        std::process::exit(1);
    }  
}
