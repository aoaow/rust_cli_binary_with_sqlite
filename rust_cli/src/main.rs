use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: i32,
    title: String,
    completed: bool,
}

// Implement the functions for CRUD operations here (create_task, read_tasks, etc.)

#[derive(Parser)]
#[command(name = "Task CLI", version, about = "A simple task manager CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add(AddArgs),
    List,
    Update(UpdateArgs),
    Delete(DeleteArgs),
}

#[derive(Args)]
struct AddArgs {
    title: String,
}

#[derive(Args)]
struct UpdateArgs {
    id: i32,
    title: String,
    #[arg(short, long)]
    completed: bool,
}

#[derive(Args)]
struct DeleteArgs {
    id: i32,
}

// The main function
fn main() -> Result<()> {
    let cli = Cli::parse();

    let conn = Connection::open("tasks.db")?;
    initialize_database(&conn)?;

    match cli.command {
        Commands::Add(args) => {
            create_task(&conn, &args.title)?;
            println!("Task '{}' added.", args.title);
        }
        Commands::List => {
            let tasks = read_tasks(&conn)?;
            for task in tasks {
                println!(
                    "[{}] {} - {}",
                    task.id,
                    task.title,
                    if task.completed { "Completed" } else { "Pending" }
                );
            }
        }
        Commands::Update(args) => {
            update_task(&conn, args.id, &args.title, args.completed)?;
            println!("Task '{}' updated.", args.id);
        }
        Commands::Delete(args) => {
            delete_task(&conn, args.id)?;
            println!("Task '{}' deleted.", args.id);
        }
    }

    Ok(())
}

// Functions for database initialization and CRUD operations
fn initialize_database(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            title       TEXT NOT NULL,
            completed   BOOLEAN NOT NULL
        )",
        [],
    )?;
    Ok(())
}

fn create_task(conn: &Connection, title: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks (title, completed) VALUES (?1, ?2)",
        params![title, false],
    )?;
    Ok(())
}

fn read_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare("SELECT id, title, completed FROM tasks")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;

    let mut tasks = Vec::new();
    for task in task_iter {
        tasks.push(task?);
    }
    Ok(tasks)
}

fn update_task(conn: &Connection, id: i32, title: &str, completed: bool) -> Result<()> {
    conn.execute(
        "UPDATE tasks SET title = ?1, completed = ?2 WHERE id = ?3",
        params![title, completed, id],
    )?;
    Ok(())
}

fn delete_task(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
    Ok(())
}
