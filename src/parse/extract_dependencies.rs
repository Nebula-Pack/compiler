use std::fs;
use std::path::Path;
use serde_json::Value;

pub fn find_and_replace_requires(project_folder: &str, output_folder: &str) {
    let config_path = Path::new(project_folder).join("nebula-config.json");
    println!("Looking for config file at: {:?}", config_path);

    if config_path.exists() {
        println!("nebula-config.json found");
        match fs::read_to_string(&config_path) {
            Ok(json_str) => {
                match serde_json::from_str::<Value>(&json_str) {
                    Ok(json) => {
                        if let Some(dependencies) = json.get("dependencies") {
                            if let Some(deps) = dependencies.as_object() {
                                process_directory(project_folder, output_folder, deps);
                            } else {
                                eprintln!("Dependencies is not an object");
                            }
                        } else {
                            eprintln!("No dependencies found in nebula-config.json");
                        }
                    },
                    Err(e) => eprintln!("Failed to parse JSON: {}", e),
                }
            },
            Err(e) => eprintln!("Failed to read nebula-config.json: {}", e),
        }
    } else {
        eprintln!("nebula-config.json not found at {:?}", config_path);
    }
}

fn process_directory(input_dir: &str, output_dir: &str, deps: &serde_json::Map<String, Value>) {
    for entry in fs::read_dir(input_dir).expect("Failed to read input directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "lua" {
                    process_lua_file(&path, input_dir, output_dir, deps);
                }
            }
        } else if path.is_dir() {
            let new_input_dir = path.to_str().expect("Invalid path");
            let relative_path = path.strip_prefix(input_dir).expect("Failed to strip prefix");
            let new_output_dir = Path::new(output_dir).join(relative_path);
            let new_output_dir_str = new_output_dir.to_str().expect("Invalid path");
            
            fs::create_dir_all(&new_output_dir).expect("Failed to create output directory");
            process_directory(new_input_dir, new_output_dir_str, deps);
        }
    }
}

fn process_lua_file(file_path: &Path, input_dir: &str, output_dir: &str, deps: &serde_json::Map<String, Value>) {
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let mut new_content = content.clone();

    for key in deps.keys() {
        let require_statement = format!("require(\"{}\")", key);
        let new_require_statement = format!("require(\"neb-pack/{}\")", key);
        new_content = new_content.replace(&require_statement, &new_require_statement);
    }

    let relative_path = file_path.strip_prefix(input_dir).expect("Failed to get relative path");
    let new_path = Path::new(output_dir).join(relative_path);

    if let Some(parent) = new_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create directories");
    }

    fs::write(new_path, new_content).expect("Failed to write file");
}