use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use std::fs;

use std::path::PathBuf;

use todo_list_parser::{DescriptionPart, Priority, Status, Task, parse_file};

#[derive(Parser)]
#[command(
    name = "ToDo List Parser",
    version = "1.0",
    about = "Parser for ToDo list sorting"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Parse {
        #[arg(required = true)]
        file: PathBuf,

        #[arg(long)]
        sort_by_priority: bool,

        #[arg(long)]
        sort_by_status: bool,

        #[arg(long)]
        sort_by_start: bool,

        #[arg(long)]
        sort_by_due: bool,
    },
    Credits,
}

fn print_credits() {
    println!("Credits");
    println!("Created by Dyshlyuk Nikita");
    println!("MIT license");
}

fn print_task(task: &Task) {
    let priority_str = match &task.priority {
        Some(Priority::P1) => "(***)",
        Some(Priority::P2) => "(**)",
        Some(Priority::P3) => "(*)",
        None => "",
    };

    let status_str = match &task.status {
        Status::Todo => "[TODO]",
        Status::Doing => "[DOING]",
        Status::Done => "[DONE]",
    };

    print!("{} {}", priority_str, status_str);

    for part in &task.description {
        match part {
            DescriptionPart::Text(t) => print!(" {}", t.trim()),
            DescriptionPart::Tag(t) => print!(" {}", t),
            DescriptionPart::DueDate(d) => print!(" {}", d),
            DescriptionPart::StartDate(s) => print!(" {}", s),
        }
    }
    println!();
}

fn extract_due_date(task: &Task) -> Option<String> {
    for part in &task.description {
        if let DescriptionPart::DueDate(date) = part {
            return Some(date.clone());
        }
    }
    None
}

fn extract_start_date(task: &Task) -> Option<String> {
    for part in &task.description {
        if let DescriptionPart::StartDate(date) = part {
            return Some(date.clone());
        }
    }
    None
}

fn priority_order(priority: &Option<Priority>) -> u8 {
    match priority {
        Some(Priority::P1) => 0,
        Some(Priority::P2) => 1,
        Some(Priority::P3) => 2,
        None => 3,
    }
}

fn status_order(status: &Status) -> u8 {
    match status {
        Status::Todo => 0,
        Status::Doing => 1,
        Status::Done => 2,
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Parse {
            file,
            sort_by_priority,
            sort_by_status,
            sort_by_start,
            sort_by_due,
        } => {
            let file_content = fs::read_to_string(file)
                .map_err(|e| anyhow!("cant read '{}':\n  Error {}\n", file.display(), e))?;

            let mut tasks = parse_file(&file_content)?;

            let sort_type = if *sort_by_priority {
                "Priority"
            } else if *sort_by_status {
                "Status"
            } else if *sort_by_start {
                "Start Date"
            } else if *sort_by_due {
                "Final date"
            } else {
                "just list"
            };

            if *sort_by_priority {
                tasks.sort_by(|a, b| priority_order(&a.priority).cmp(&priority_order(&b.priority)));
            } else if *sort_by_status {
                tasks.sort_by(|a, b| status_order(&a.status).cmp(&status_order(&b.status)));
            } else if *sort_by_start {
                tasks.sort_by(|a, b| {
                    let date_a = extract_start_date(a).unwrap_or_else(|| "9999-99-99".to_string());
                    let date_b = extract_start_date(b).unwrap_or_else(|| "9999-99-99".to_string());
                    date_a.cmp(&date_b)
                });
            } else if *sort_by_due {
                tasks.sort_by(|a, b| {
                    let date_a = extract_due_date(a).unwrap_or_else(|| "9999-99-99".to_string());
                    let date_b = extract_due_date(b).unwrap_or_else(|| "9999-99-99".to_string());
                    date_a.cmp(&date_b)
                });
            }

            println!("There is {}  tasks | sorting by {}", tasks.len(), sort_type);
            for task in tasks {
                print_task(&task);
            }
        }
        Commands::Credits => {
            print_credits();
        }
    }

    Ok(())
}
