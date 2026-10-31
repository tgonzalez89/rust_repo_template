//! Reusable functionality for project-name.

/// Build a greeting for the supplied name.
///
/// # Examples
///
/// ```
/// assert_eq!(project_name::greet("Rust"), "Hello, Rust!");
/// ```
#[must_use]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}
