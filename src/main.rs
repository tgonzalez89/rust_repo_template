//! Command-line entry point for project-name.

use std::env;

fn main() {
    let name = env::args().nth(1).unwrap_or_else(|| String::from("World"));
    println!("{}", project_name::greet(&name));
}
