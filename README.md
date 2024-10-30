# rust_cli_binary_with_sqlite
[![CI](https://github.com/aoaow/rust_cli_binary_with_sqlite/actions/workflows/ci.yml/badge.svg)](https://github.com/aoaow/rust_cli_binary_with_sqlite/actions/workflows/ci.yml)

## Introduction

`rust_cli` is a Rust command-line application that manages a task list stored in a SQLite database. It supports creating, reading, updating, and deleting tasks via command-line arguments, demonstrating CRUD operations and leveraging Rust's unique features. You can access the detailed introductory video [here](https://youtu.be/RmSjr4ubk3U)

## Features

- **Add Tasks**: Create new tasks with a title.
- **List Tasks**: Display all tasks with their completion status.
- **Update Tasks**: Modify existing tasks, including marking them as completed.
- **Delete Tasks**: Remove tasks by their ID.

## Dependencies

Ensure you have the latest stable version of Rust installed. This project uses the following Rust crates:

- **[rusqlite](https://crates.io/crates/rusqlite)**: SQLite database interaction.
- **[serde](https://crates.io/crates/serde)**: Serialization and deserialization.
- **[clap](https://crates.io/crates/clap)**: Command-line argument parsing.

You can install Rust using [rustup](https://www.rust-lang.org/tools/install):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Installation

1. **Clone the Repository**

   ```bash
   git clone https://gitlab.com/your-username/task_cli.git
   cd task_cli
   ```

2. **Build the Application**

   Build the application in release mode for optimization:

   ```bash
   cargo build --release
   ```

   The optimized binary will be located at `./target/release/rust_cli`.

## Usage

Run the `rust_cli` binary with the desired command and options.

### **Add a Task**

Create a new task with a title.

```bash
./target/release/rust_cli add "Your task title"
```

**Example:**

```bash
./target/release/task_cli add "Finish the project report"
```

### **List Tasks**

Display all tasks with their status.

```bash
./target/release/task_cli list
```

**Example Output:**

```
[1] Finish the project report - Pending
[2] Update the website - Completed
```

### **Update a Task**

Update a task's title and/or mark it as completed.

```bash
./target/release/task_cli update <id> "New title" [--completed]
```

- `<id>`: The ID of the task to update.
- `"New title"`: The new title for the task.
- `--completed`: (Optional) Mark the task as completed.

**Example:**

```bash
./target/release/task_cli update 1 "Finalize the project report" --completed
```

### **Delete a Task**

Delete a task by its ID.

```bash
./target/release/task_cli delete <id>
```

**Example:**

```bash
./target/release/task_cli delete 1
```

## How I Used an LLM in this project

During the development of this project, I leveraged a Large Language Model (LLM) to enhance productivity and code quality:

- **Understanding Rust Concepts**: The LLM explained complex Rust concepts like ownership, borrowing, lifetimes, and error handling with the `Result` type and the `?` operator.
- **Best Practices**: It suggested best practices for code organization, including modularization and error handling strategies.
- **Documentation**: The LLM helped articulate explanations in the README and code comments, ensuring clarity and comprehensiveness.

By integrating the LLM into the development workflow, I was able to accelerate the coding process, learn new Rust features, and maintain high code quality.

## GitLab CI/CD Pipeline

The project includes a GitLab CI/CD pipeline defined in `.gitlab-ci.yml` that automates the following tasks:

- **Linting**: Uses `cargo clippy` to check code for common mistakes and adherence to best practices.
- **Testing**: Runs `cargo test` to execute any unit tests and validate functionality.
- **Building**: Compiles the code in release mode to generate an optimized binary.

## Project Structure

```
task_cli/
├── Cargo.lock
├── Cargo.toml
├── .gitlab-ci.yml
├── README.md
├── src
│   └── main.rs
└── tasks.db
```

- **`Cargo.toml`**: Contains the project's dependencies and metadata.
- **`.gitlab-ci.yml`**: Defines the CI/CD pipeline configuration.
- **`src/main.rs`**: The main Rust source code file.
- **`tasks.db`**: The SQLite database file (created at runtime).
- **`README.md`**: Project documentation.

## Contributing

Contributions are welcome! If you'd like to contribute:

- Fork the repository.
- Create a new branch for your feature or bugfix.
- Submit a pull request with a detailed description of your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- **Rust Community**: For providing excellent documentation and support.
- **Crate Authors**: Gratitude to the developers of `rusqlite`, `serde`, and `clap` for their invaluable libraries.
- **LLM Assistance**: Appreciation for the guidance provided by the LLM in coding and problem-solving.
