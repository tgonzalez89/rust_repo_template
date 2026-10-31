//! Integration tests for the public library API.

use project_name::greet;

#[test]
fn greets_a_name() {
    assert_eq!(greet("Rust"), "Hello, Rust!");
}

#[test]
fn greets_an_empty_name() {
    assert_eq!(greet(""), "Hello, !");
}
