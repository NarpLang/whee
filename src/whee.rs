#!/usr/bin/env rustc
use std::env;
use std::fs::{self, File};
use std::io::{Write, BufWriter};
use std::process::Command;
use std::path::Path;

const TEMP_RUST_FILE: &str = "/tmp/whee_temp.rs";
const TEMP_EXEC_FILE: &str = "/tmp/whee_exec";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <whee_script>", args[0]);
        std::process::exit(1);
    }

    let whee_file = &args[1];

    // Run wheec and capture the output
    let output = Command::new("wheec")
        .arg(whee_file)
        .output()
        .expect("Failed to execute wheec");

    if !output.status.success() {
        eprintln!("Error: wheec failed");
        std::process::exit(1);
    }

    // Write output to TEMP_RUST_FILE, skipping the first line
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut lines = stdout.lines();
    lines.next(); // Skip the header line "=== Converted Rust Code ==="

    let mut file = BufWriter::new(File::create(TEMP_RUST_FILE).expect("Failed to create temp Rust file"));
    for line in lines {
        writeln!(file, "{}", line).unwrap();
    }
    file.flush().unwrap();

    // Compile the Rust code
    let status = Command::new("rustc")
        .arg(TEMP_RUST_FILE)
        .arg("-o")
        .arg(TEMP_EXEC_FILE)
        .status()
        .expect("Failed to run rustc");

    if !status.success() {
        eprintln!("Error: rustc compilation failed");
        cleanup();
        std::process::exit(1);
    }

    // Execute the compiled binary
    let run_status = Command::new(TEMP_EXEC_FILE)
        .status()
        .expect("Failed to run compiled Whee program");

    cleanup();

    std::process::exit(run_status.code().unwrap_or(1));
}

fn cleanup() {
    if Path::new(TEMP_RUST_FILE).exists() {
        fs::remove_file(TEMP_RUST_FILE).ok();
    }
    if Path::new(TEMP_EXEC_FILE).exists() {
        fs::remove_file(TEMP_EXEC_FILE).ok();
    }
}
