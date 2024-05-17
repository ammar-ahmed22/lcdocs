use colored::*;
use std::io::Write;

pub mod config;
pub mod utils;

pub fn create_problem_file(
    problem_name: &str,
    difficulty: config::Difficulty,
) -> anyhow::Result<()> {
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

    // Writing the template file to the new problem file
    let template_path = std::path::Path::new("./examples/_template.rs");

    let dest = format!("./examples/{}/{}.rs", difficulty, problem_name);
    let dest_path = std::path::Path::new(&dest);

    std::fs::copy(&template_path, &dest_path)?;

    // Updating the Cargo.toml to include the example
    let cargo_path = std::path::Path::new("./Cargo.toml");
    let mut cargo_file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&cargo_path)?;

    writeln!(cargo_file, "\n\n[[example]]")?;
    writeln!(cargo_file, "name = \"{}\"", problem_name)?;
    writeln!(cargo_file, "path = {:?}", dest_path)?;

    // Logging
    log::info!(
        "Created '{}' problem '{}'",
        format!("{}", difficulty).magenta(),
        problem_name.magenta()
    );
    log::info!(
        "Implement your solution in {}",
        format!("{:?}", dest_path).cyan()
    );
    log::info!(
        "Test with `{}`",
        format!("lcdocs test {}", problem_name).cyan()
    );
    log::info!(
        "Run with `{}`",
        format!("lcdocs run {}", problem_name).cyan()
    );
    Ok(())
}

/// Creates the template docs page for the problem <br />
/// **IMPORTANT:** This must be run AFTER `create_problem_file`
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
        // Get the sidebar position
        let sidebar_pos = match difficulty {
            config::Difficulty::Easy => match problems.easy.get(problem_name) {
                Some(val) => *val,
                None => return Err(anyhow::anyhow!("Cannot find problem '{}'", problem_name)),
            },
            config::Difficulty::Medium => match problems.medium.get(problem_name) {
                Some(val) => *val,
                None => return Err(anyhow::anyhow!("Cannot find problem '{}'", problem_name)),
            },
            config::Difficulty::Hard => match problems.hard.get(problem_name) {
                Some(val) => *val,
                None => return Err(anyhow::anyhow!("Cannot find problem '{}'", problem_name)),
            },
        };

        // Reading the template file
        let mut template_str = std::fs::read_to_string(template_path)?;

        // Path to the output file
        let output = format!("./{}.md", problem_name);
        let output_path = docs_path.join(&output);

        // Find and modify sidebar position
        if let Some(pos) = template_str.find("sidebar_position:") {
            let start = pos + "sidebar_position:".len();
            if let Some(end) = template_str[start..].find("\n") {
                let new_value = format!(" {}", sidebar_pos);
                template_str.replace_range(start..start + end, &new_value);
            }
        }

        // Write the new file
        std::fs::write(output_path, template_str)?;
        log::info!(
            "Created template docs page for '{}' problem '{}'",
            format!("{}", difficulty).magenta(),
            problem_name.magenta()
        );
        log::info!("Run with '{}'", "cd docs && npm start".cyan())
    }
    Ok(())
}

pub fn remove_problem(problem_name: &str, difficulty: config::Difficulty) -> anyhow::Result<()> {
    let target = format!("./examples/{}/{}.rs", difficulty, problem_name);
    let target_path = std::path::Path::new(&target);
    // Delete problem package file
    if target_path.exists() {
        std::fs::remove_file(&target_path)?;
        log::info!("Removed '{}' file", problem_name.magenta());
    }

    // Remove the example entry in Cargo.toml
    let cargo_path = std::path::Path::new("./Cargo.toml");
    let delete_target = format!("name = \"{}\"", problem_name);
    utils::fs::delete_lines_around(cargo_path, &delete_target, Some(1), Some(1))?;

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
