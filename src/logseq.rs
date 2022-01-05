use chrono::prelude::*;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

fn open_file(path: &PathBuf) -> Result<File, io::Error> {
    OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(path)
}

fn get_date() -> String {
    let dt = Local::now();
    let date = dt.format("%Y_%m_%d").to_string();

    date
}

pub fn write_daily(logseq_path: String, note: String) -> std::io::Result<()> {
    let file_path = PathBuf::from(logseq_path + "\\journals\\" + &get_date() + ".md");
    let mut file = open_file(&file_path)?;
    let buf_reader = BufReader::new(&file);
    let notes_header = "- ## CLNotes";
    let mut has_notes_header = false;

    for line in buf_reader.lines() {
        let line = line?;

        if line == notes_header {
            has_notes_header = true;
        }
    }

    if !has_notes_header {
        write!(&file, "\n{}", notes_header)?;
    }

    write!(file, "\n\t- {}", note)?;

    Ok(())
}
