use anyhow::Context;
use colored::*;
use lcdocs::{config, utils};

fn try_main() -> anyhow::Result<()> {
    config::initialize_logger();
    let cmd = config::parse_command();
    match cmd {
        config::Commands::Create {
            problem,
            difficulty,
        } => {
            // Parse the passed name to snake_case
            let parsed_name = utils::string::to_snake_case(problem.as_str());
            // Create the Rust package for the problem
            lcdocs::create_problem_file(&parsed_name, difficulty.clone())
                .with_context(|| format!("trying to create '{}'", problem.magenta()))?;
            // Create the docs page for the problem
            lcdocs::create_problem_docs(&parsed_name, difficulty.clone())?;
            Ok(())
        }
        config::Commands::Run {
            problem,
            difficulty,
        } => {
            let parsed_name = utils::string::to_snake_case(&problem);
            utils::problems::find_problem_difficulty(&parsed_name, difficulty).with_context(
                || anyhow::anyhow!("problem '{}' does not exist!", parsed_name.magenta()),
            )?;

            log::info!("Running '{}':", parsed_name.magenta());
            std::process::Command::new("cargo")
                .arg("run")
                .arg("--example")
                .arg(&parsed_name)
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .status()?;
            log::info!("Finished running '{}'", parsed_name.magenta());
            Ok(())
        }
        config::Commands::Test {
            problem,
            difficulty,
        } => {
            let parsed_name = utils::string::to_snake_case(&problem);
            utils::problems::find_problem_difficulty(&parsed_name, difficulty).with_context(
                || anyhow::anyhow!("problem '{}' does not exist!", parsed_name.magenta()),
            )?;
            log::info!("Testing '{}'", parsed_name.magenta());
            std::process::Command::new("cargo")
                .arg("test")
                .arg("--example")
                .arg(&parsed_name)
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .status()?;
            log::info!("Finished testing '{}'", parsed_name.magenta());
            Ok(())
        }
        config::Commands::Remove {
            problem,
            difficulty,
        } => {
            let parsed_name = utils::string::to_snake_case(problem.as_str());
            let parsed_diff = utils::problems::find_problem_difficulty(&parsed_name, difficulty)?;
            lcdocs::remove_problem(&parsed_name, parsed_diff)
        }
    }
}

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}: {}", "Error".red(), err);
        if err.chain().len() > 1 {
            eprintln!("\n{}", "Caused by:".yellow());
            err.chain()
                .skip(1)
                .enumerate()
                .for_each(|(i, cause)| eprintln!("\t{}: {}", i, cause));
        }
        std::process::exit(1);
    }
}
