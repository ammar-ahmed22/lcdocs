use anyhow::Context;
use colored::*;
use std::io::Write;

pub mod config;
pub mod utils;

/// Creates a package that can be run for the problem
pub fn create_problem_pkg(
    problem_name: &str,
    difficulty: config::Difficulty,
) -> anyhow::Result<()> {
    let src = std::path::Path::new("./_template_");
    let str_diff = format!("{}", difficulty);
    let dest_path = format!("./{}/{}", str_diff, problem_name)
        .as_str()
        .to_owned();

    // Updating the problems.json file
    let mut problems: utils::problems::Problems =
        utils::problems::Problems::extract(utils::problems::PROBLEMS_JSON)?;
    match difficulty {
        config::Difficulty::Easy => {
            if problems.easy.contains_key(problem_name) {
                return Err(anyhow::anyhow!(
                    "problem '{}' already exists in JSON!",
                    problem_name
                ));
            }
            let max_idx = utils::problems::find_max_problem_idx(&problems.easy);
            problems.easy.insert(problem_name.to_string(), max_idx + 1);
        }
        config::Difficulty::Medium => {
            if problems.medium.contains_key(problem_name) {
                return Err(anyhow::anyhow!(
                    "problem '{}' already exists in JSON!",
                    problem_name
                ));
            }
            let max_idx = utils::problems::find_max_problem_idx(&problems.medium);
            problems
                .medium
                .insert(problem_name.to_string(), max_idx + 1);
        }
        config::Difficulty::Hard => {
            if problems.hard.contains_key(problem_name) {
                return Err(anyhow::anyhow!(
                    "problem '{}' already exists in JSON!",
                    problem_name
                ));
            }
            let max_idx = utils::problems::find_max_problem_idx(&problems.hard);
            problems.hard.insert(problem_name.to_string(), max_idx + 1);
        }
    }

    problems.write_to_file(utils::problems::PROBLEMS_JSON)?;

    let dest = std::path::Path::new(&dest_path);
    utils::fs::copy_dir(src, dest).with_context(|| format!("Copying directory failed"))?;
    log::info!(
        "created {} problem package '{}'",
        format!("{}", difficulty).magenta(),
        problem_name.magenta()
    );
    log::info!(
        "Run with {}",
        format!("{}", format!("leetcode_nodes run {}", problem_name).cyan())
    );
    Ok(())
}

/// Creates the template docs page for the problem
pub fn create_problem_docs(
    problem_name: &str,
    difficulty: config::Difficulty,
) -> anyhow::Result<()> {
    // Path for the docs directory
    let docs_path_str = format!("./docs/docs/{}", difficulty);
    let docs_path = std::path::Path::new(&docs_path_str);

    if docs_path.exists() {
        // Path to the template file
        let template_path = docs_path.join("./_template.md");

        let problems: utils::problems::Problems =
            utils::problems::Problems::extract(utils::problems::PROBLEMS_JSON)?;
        // Get the maximum sidebar position
        let max_pos: i32 = match difficulty {
            config::Difficulty::Easy => utils::problems::find_max_problem_idx(&problems.easy),
            config::Difficulty::Medium => utils::problems::find_max_problem_idx(&problems.medium),
            config::Difficulty::Hard => utils::problems::find_max_problem_idx(&problems.hard),
        };

        // Reading the template file
        let mut template_str = std::fs::read_to_string(template_path)?;

        // Path to the output file
        let output = format!("./{}.md", problem_name);
        let output_path = docs_path.join(&output);

        // Find and modify sidebar position to be 1 more than the max
        if let Some(pos) = template_str.find("sidebar_position:") {
            let start = pos + "sidebar_position:".len();
            if let Some(end) = template_str[start..].find("\n") {
                let new_value = format!(" {}", max_pos + 1);
                template_str.replace_range(start..start + end, &new_value);
            }
        }

        // Write the new file
        std::fs::write(output_path, template_str)?;
        log::info!(
            "created template docs page for {} problem '{}'",
            format!("{}", difficulty).magenta(),
            problem_name.magenta()
        );
        log::info!("Run with '{}'", "cd docs && npm start".cyan())
    }
    Ok(())
}

pub fn remove_problem(problem_name: &str, difficulty: config::Difficulty) -> anyhow::Result<()> {
    let target = format!("./{}/{}", difficulty, problem_name);
    let target_path = std::path::Path::new(&target);
    // Delete problem package directory
    if target_path.exists() {
        std::fs::remove_dir_all(target_path)?;
        log::info!("Removed '{}' directory", problem_name.magenta());
    }

    // Remove from JSON map
    let mut problems = utils::problems::Problems::extract(utils::problems::PROBLEMS_JSON)?;
    match difficulty {
        config::Difficulty::Easy => {
            problems.easy.remove(problem_name);
        }
        config::Difficulty::Medium => {
            problems.medium.remove(problem_name);
        }
        config::Difficulty::Hard => {
            problems.hard.remove(problem_name);
        }
    };
    problems.write_to_file(utils::problems::PROBLEMS_JSON)?;
    log::info!("Removed '{}' JSON entry", problem_name.magenta());

    // Remove docs file
    let docs = format!("./docs/docs/{}/{}.md", difficulty, problem_name);
    let docs_path = std::path::Path::new(&docs);
    if docs_path.exists() {
        std::fs::remove_file(docs_path)?;
        log::info!("Removed '{}' docs page", problem_name.magenta());
    }

    Ok(())
}
