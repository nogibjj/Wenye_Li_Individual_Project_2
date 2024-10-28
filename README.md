[![CI/CD](https://github.com/nogibjj/Wenye_Li_Individual_Project_2/actions/workflows/cicd.yml/badge.svg)](https://github.com/nogibjj/Wenye_Li_Individual_Project_2/actions/workflows/cicd.yml)

## Wenye Li Individual Project 2

A Rust command-line application that manages and analyzes drug use data using SQLite. This project demonstrates CRUD operations, database management, and data analysis capabilities using Rust and SQLite.

## Requirements

- Rust source code: The code should comprehensively understand Rust's syntax and unique features.
- Use of LLM: In your README, explain how you utilized an LLM in your coding process.
- SQLite Database: Include a SQLite database and demonstrate CRUD (Create, Read, Update, Delete) operations.
- Optimized Rust Binary: Include a process that generates an optimized Rust binary as a Gitlab Actions artifact that can be downloaded.
- README.md: A file that clearly explains what the project does, its dependencies, how to run the program, and how Gitlab Copilot was used.
- Github/Gitlab Actions: A workflow file that tests, builds, and lints your Rust code.
- Video Demo: A YouTube link in README.md showing a clear, concise walkthrough and demonstration of your CLI binary.

## Project Overview

This CLI application processes and analyzes drug use data from FiveThirtyEight's dataset. It performs the following operations:

- Downloads and extracts data from a CSV file
- Transforms and loads data into a SQLite database
- Performs CRUD (Create, Read, Update, Delete) operations
- Logs all database operations
- Provides test coverage for all main functionalities

## Usage Instructions

The application supports the following commands:

### 1. Initial Setup

```bash
# Extract data from source
cargo run -- extract

# Load data into database
cargo run -- load
```

### 2. CRUD Operations

#### Create (Insert)

```bash
# Insert predefined test data
cargo run -- insert
```

#### Read (Select)

```bash
# View all records
cargo run -- select
```

#### Update

```bash
# Update specific age group data
cargo run -- update -a "30-34" -f alcohol_use -v 70.0
```

#### Delete

```bash
# Delete specific age group
cargo run -- delete -a "26-29"
```

### 3. Custom Queries

```bash
# Execute custom SQL query
cargo run -- query -s "SELECT age, alcohol_use FROM DrugUse WHERE alcohol_use > 50"

# Or use interactive mode
cargo run -- query
```

### Operation Logging

All operations are logged in `db_operations.md` with timestamps and details.

[Rest of your original README content...]

### Running the Application in Codespaces

Once your codespace is ready:

```bash
# Build the project
cargo build

# Run the application
cargo run

# Run tests
cargo test
```

### VS Code Extensions

The devcontainer automatically installs:

- rust-analyzer
- SQLite viewer
- CodeLLDB (Debugger)
- GitHub Copilot (optional)

## Project Structure

```
.
├── .devcontainer/           # Development container configuration
├── .github/workflows/       # GitHub Actions workflow
├── data/                    # Data directory for CSV files
├── src/                     # Source code
│   ├── lib.rs              # Library code
│   └── main.rs             # Main application
├── test/                    # Test files
├── Cargo.toml              # Rust dependencies and project metadata
├── Dockerfile              # Docker configuration
└── Makefile                # Build automation
```

## Testing

Run the test suite:

```bash
make test
```

## CI/CD Pipeline

This project uses GitHub Actions for continuous integration and delivery. The pipeline:

- Runs code formatting checks
- Performs linting using clippy
- Executes the test suite
- Builds an optimized binary
- Creates downloadable artifacts

## Use of Large Language Models

During development, GitHub Copilot was utilized to:

- Provide code suggestions for complex SQLite operations
- Assist with documentation
- Suggest test cases

## Video Demo

[Watch the demo on YouTube](your-youtube-link)
