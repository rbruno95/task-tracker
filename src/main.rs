mod parser;
mod structs;

use {
    parser::{Cli, Command},
    structs::Task,
};

use {clap::Parser, std::collections::HashMap};

fn main() {
    let cli = Cli::parse();

    let mut tasks = HashMap::new();

    match cli.command {
        Some(Command::Add { description }) => {
            let new_task = Task::new(description);
            tasks.insert(new_task.id, new_task);
        }
        Some(Command::Update { index, new_value }) => {}
        Some(Command::Delete { index }) => {}
        Some(Command::MarkInProgress { index }) => {}
        Some(Command::MarkDone { index }) => {}
        Some(Command::List { status }) => {}
        None => {}
    }
}
