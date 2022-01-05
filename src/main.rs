mod config;
mod logseq;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
    #[clap(short, long)]
    logseq_path: Option<String>,
    note: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Logseq { note: String },
}

fn main() {
    let cli = Cli::parse();
    let mut config = config::Config::new().load_config();

    if let Some(path) = cli.logseq_path {
        config.set_logseq_path(path);
    }

    match &cli.command {
        Some(Commands::Logseq { note }) => {
            logseq::write_daily(config.logseq_path.unwrap(), note.to_string()).unwrap()
        }
        None => {
            if let Some(note) = cli.note {
                if config.default_notes == "logseq" {
                    logseq::write_daily(config.logseq_path.unwrap(), note.to_string()).unwrap()
                } else {
                    ()
                }
            }
        }
    }
}
