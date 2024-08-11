use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Add { value: String },
    Update { index: usize, new_value: String },
    Delete { index: usize },
    MarkInProgress { index: usize },
    MarkDone { index: usize },
    List { status: Option<String> },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Add { value }) => {}
        Some(Command::Update { index, new_value }) => {}
        Some(Command::Delete { index }) => {}
        Some(Command::MarkInProgress { index }) => {}
        Some(Command::MarkDone { index }) => {}
        Some(Command::List { status }) => {}
        None => {}
    }
}
