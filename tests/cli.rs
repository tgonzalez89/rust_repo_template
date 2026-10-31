//! End-to-end tests for the command-line entry point.

use std::error::Error;
use std::process::Command;

#[test]
fn greets_world_by_default() -> Result<(), Box<dyn Error>> {
    let output = Command::new(env!("CARGO_BIN_EXE_project-name")).output()?;

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout)?.trim(), "Hello, World!");
    assert_eq!(output.stderr.as_slice(), b"");
    Ok(())
}

#[test]
fn greets_the_supplied_name() -> Result<(), Box<dyn Error>> {
    let output = Command::new(env!("CARGO_BIN_EXE_project-name"))
        .arg("Rust")
        .output()?;

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout)?.trim(), "Hello, Rust!");
    assert_eq!(output.stderr.as_slice(), b"");
    Ok(())
}
