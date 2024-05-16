use anyhow::Context;
use colored::*;
use leetcode_notes::{config, remove_problem};

fn try_main() -> anyhow::Result<()> {
    config::initialize_logger();
    let cmd = config::parse_command();
    match cmd {
        config::Commands::Create { problem, difficulty  } => {
            let parsed_name = leetcode_notes::to_snake_case(problem.as_str());
            // let parsed_diff = leetcode_notes::find_problem_difficulty(&parsed_name, difficulty)?;
            leetcode_notes::create_problem_pkg(&parsed_name, difficulty.clone())
                .with_context(|| format!("trying to create '{}'", problem.blue()))?;
            leetcode_notes::create_problem_docs(&parsed_name, difficulty.clone())?;
            Ok(())
        },
        config::Commands::Run { problem, difficulty } => {
            let parsed_name = leetcode_notes::to_snake_case(&problem);
            let parsed_diff = leetcode_notes::find_problem_difficulty(&parsed_name, difficulty)?;
            let target = format!("./{}/{}", parsed_diff.clone(), parsed_name);
            let target_path = std::path::Path::new(&target);
            if !target_path.exists() {
                return Err(anyhow::anyhow!("problem '{}' does not exist!", &parsed_name));
            }
            // Set the working directory to the problem
            std::env::set_current_dir(target_path)?;
            // Set env variables
            std::env::set_var("CARGO_TERM_COLOR", "always");
            
            // Compile the problem 
            let compile_state = std::process::Command::new("cargo")
                .arg("build")
                .arg("--release")
                .output()?;

            // Throw error if compilation failed
            if compile_state.status.success() {
                log::info!("Compiled '{}'", parsed_name.magenta());
            } else {
                log::error!("Failed to compile '{}'. See output below:\n{}", parsed_name.magenta(), String::from_utf8_lossy(&compile_state.stderr));
                return Err(anyhow::anyhow!("Compilation error in '{}'", parsed_name.magenta()));
            }
            
            // Run the problem
            log::info!("Running '{}'", parsed_name.magenta());
            std::process::Command::new("./target/release/_template_")
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .status()?;

            log::info!("Finished running '{}'", parsed_name.magenta());
            Ok(())
        },
        config::Commands::Remove { problem, difficulty } => {
            let parsed_name = leetcode_notes::to_snake_case(problem.as_str());
            let parsed_diff = leetcode_notes::find_problem_difficulty(&parsed_name, difficulty)?;
            remove_problem(&parsed_name, parsed_diff)
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
