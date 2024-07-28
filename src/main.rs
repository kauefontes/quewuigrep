use std::{env, process};

use quewuigrep::{run, Config};

/// Entry point of the application.
///
/// The `main` function sets up the application, parses command-line arguments,
/// and executes the main logic. If there is an error in configuration or execution,
/// error messages are displayed and the process exits with a status code of 1.
fn main() {
    // Create a new configuration from the command-line arguments.
    // If there is an error parsing the arguments, print an error message and exit.
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Run the main logic of the application with the provided configuration.
    // If there is an error during execution, print an error message and exit.
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}