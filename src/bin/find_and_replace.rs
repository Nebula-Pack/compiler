use std::env;
use your_project_name::parse::extract_dependencies;

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let project_folder = current_dir.join("input").join("example-nebula-pack-project");
    let output_folder = current_dir.join("output").join("example-nebula-pack-project");

    let project_folder_str = project_folder.to_str().expect("Invalid project folder path");
    let output_folder_str = output_folder.to_str().expect("Invalid output folder path");

    println!("Project folder: {}", project_folder_str);
    println!("Output folder: {}", output_folder_str);

    extract_dependencies::find_and_replace_requires(project_folder_str, output_folder_str);
}