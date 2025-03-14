# Quewuigrep

Quewuigrep is a command-line utility for searching a word or phrase in a file. It mimics the functionality of the classic `grep` command but is implemented in Rust. This tool allows you to perform both case-sensitive and case-insensitive searches.

## Features

- **Case-sensitive search**: Search for a word or phrase while considering the case.
- **Case-insensitive search**: Search for a word or phrase ignoring the case.
- **Simple and fast**: Built with Rust for performance and safety.

## Installation

To install Quewuigrep, you need to have Rust installed on your machine. If you don't have Rust installed, you can get it from [rust-lang.org](https://www.rust-lang.org/).

Clone the repository and build the project:

``sh
git clone https://github.com/yourusername/quewuigrep.git
cd quewuigrep
cargo build --release
``

The executable will be located in the `target/release` directory.

## Usage

To use Quewuigrep, run the following command:

``sh
./quewuigrep <query> <filename>
``

- `<query>`: The word or phrase you want to search for.
- `<filename>`: The file in which to search.

### Example

``sh
./quewuigrep "search_term" example.txt
``

### Case-insensitive Search

To perform a case-insensitive search, set the `CASE_INSENSITIVE` environment variable:

``sh
CASE_INSENSITIVE=1 ./quewuigrep "search_term" example.txt
``

## Project Structure

- **main.rs**: The entry point of the application. It handles argument parsing and calls the `run` function.
- **lib.rs**: Contains the core functionality, including the `Config` struct, `run` function, and search functions.

## Running Tests

To run the tests, use the following command:

``sh
cargo test
``

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## License

This project is licensed under the MIT License. See the LICENSE file for details.