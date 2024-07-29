use std::path::Path;
use your_project_name::parse::dependencies_only;

fn main() {
    let project_path = Path::new("input/gameBuilder");
    let state = "SLIDER";
    let output_path = Path::new("output");

    match dependencies_only::compile_love2d_project(project_path, state, output_path) {
        Ok(_) => println!("Project compiled successfully!"),
        Err(e) => eprintln!("Error compiling project: {}", e),
    }
}