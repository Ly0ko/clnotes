mod config;
mod logseq;

use crate::config::{Config, DefaultNotes};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
    #[clap(short, long)]
    logseq_path: Option<String>,
    #[clap(short, long)]
    notes_path: Option<String>,
    note: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Logseq { note: String },
}

fn main() {
    let cli = Cli::parse();
    let mut config = Config::new().load_config();

    if let Some(path) = cli.logseq_path {
        config.set_logseq_path(path);
    }

    if let Some(path) = cli.notes_path {
        config.set_notes_path(path);
    }

    match &cli.command {
        Some(Commands::Logseq { note }) => {
            logseq::write_daily(config.logseq_path.unwrap(), note.to_string()).unwrap()
        }
        None => match config.default_notes {
            DefaultNotes::CLNotes(_) => (),
            DefaultNotes::Logseq(_) => {
                if let Some(note) = cli.note {
                    logseq::write_daily(config.logseq_path.unwrap(), note.to_string()).unwrap()
                }
            }
        },
    }
}
