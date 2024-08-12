use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Option<Command>,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    Add { description: String },
    Update { index: usize, new_value: String },
    Delete { index: usize },
    MarkInProgress { index: usize },
    MarkDone { index: usize },
    List { status: Option<String> },
}
