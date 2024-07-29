use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::Read;
use regex::Regex;

pub fn compile_love2d_project(project_path: &Path, state: &str, output_path: &Path) -> std::io::Result<()> {
    // Create the output directory
    fs::create_dir_all(output_path)?;

    // Read the main.lua file
    let mut main_content = String::new();
    File::open(project_path.join("main.lua"))?.read_to_string(&mut main_content)?;

    // Find all required files for the given state
    let required_files = find_required_files(&main_content, state, project_path)?;

    // Copy required files to the output directory
    for file in required_files {
        let relative_path = file.strip_prefix(project_path).unwrap();
        let target_path = output_path.join(relative_path);
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(&file, target_path)?;
    }

    println!("Project compiled successfully with only required dependencies for state: {}", state);
    Ok(())
}

fn find_required_files(main_content: &str, state: &str, project_path: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut required_files = vec![project_path.join("main.lua")];

    // Find the GameStates table
    let game_states_regex = Regex::new(r"GameStates\s*=\s*\{[^}]+\}").unwrap();
    if let Some(game_states_match) = game_states_regex.find(main_content) {
        let game_states_content = &main_content[game_states_match.start()..game_states_match.end()];

        // Find the specific state's require statement
        let state_regex = Regex::new(&format!(r"{}\s*=\s*require\s+'([^']+)'", state)).unwrap();
        if let Some(cap) = state_regex.captures(game_states_content) {
            let module_path = &cap[1];
            let file_path = project_path.join(module_path.replace(".", "/") + ".lua");
            if file_path.exists() {
                required_files.push(file_path.clone());

                // Recursively find dependencies in the state file
                let mut state_content = String::new();
                File::open(&file_path)?.read_to_string(&mut state_content)?;
                let nested_files = find_nested_dependencies(&state_content, project_path)?;
                required_files.extend(nested_files);
            }
        }
    }

    // Add global dependencies
    add_global_dependencies(&main_content, project_path, &mut required_files)?;

    Ok(required_files)
}

fn find_nested_dependencies(content: &str, project_path: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut nested_files = Vec::new();
    let file_regex = Regex::new(r#"(?:require|love\.graphics\.newImage|love\.audio\.newSource)\s*\(\s*["']([^"']+)["']\s*\)"#).unwrap();

    for cap in file_regex.captures_iter(content) {
        let file_name = &cap[1];
        let file_path = project_path.join(file_name);
        if file_path.exists() {
            nested_files.push(file_path.clone());

            // Recursively find dependencies in the nested file
            if file_path.extension().and_then(|s| s.to_str()) == Some("lua") {
                let mut nested_content = String::new();
                File::open(&file_path)?.read_to_string(&mut nested_content)?;
                let sub_nested_files = find_nested_dependencies(&nested_content, project_path)?;
                nested_files.extend(sub_nested_files);
            }
        }
    }

    Ok(nested_files)
}

fn add_global_dependencies(main_content: &str, project_path: &Path, required_files: &mut Vec<PathBuf>) -> std::io::Result<()> {
    let global_regex = Regex::new(r#"(?:require|love\.graphics\.newImage|love\.audio\.newSource)\s*\(\s*["']([^"']+)["']\s*\)"#).unwrap();

    for cap in global_regex.captures_iter(main_content) {
        let file_name = &cap[1];
        let file_path = project_path.join(file_name);
        if file_path.exists() && !required_files.contains(&file_path) {
            required_files.push(file_path);
        }
    }

    Ok(())
}
