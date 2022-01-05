use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub enum DefaultNotes {
    CLNotes(String),
    Logseq(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub default_notes: DefaultNotes,
    pub notes_path: Option<String>,
    pub logseq_path: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            default_notes: DefaultNotes::Logseq(String::from("Logseq")),
            notes_path: None,
            logseq_path: None,
        }
    }

    pub fn load_config(&mut self) -> Config {
        let config_path = self.get_config_path();

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(config_path)
            .expect("Unable to open file");
        let mut reader = BufReader::new(file);
        let contents = &reader.fill_buf().unwrap();
        if contents.is_empty() {
            self.save_config();
        }
        let config: Config = serde_json::from_reader(reader).unwrap();
        config
    }

    pub fn set_logseq_path(&mut self, path: String) {
        self.logseq_path = Some(path);
        self.save_config();
    }

    pub fn set_notes_path(&mut self, path: String) {
        self.notes_path = Some(path);
        self.save_config();
    }

    fn save_config(&mut self) {
        let config_data = serde_json::to_string(self).unwrap();
        let config_path = PathBuf::from(self.get_config_path());
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(config_path)
            .unwrap();

        file.write_all(config_data.as_bytes()).unwrap();
    }

    fn get_config_path(&self) -> String {
        let mut config_path = "".to_string();
        if let Some(proj_dirs) = ProjectDirs::from("com", "tylerdotdev", "CLNotes") {
            let dir = proj_dirs.config_dir();
            let base_path = dir.to_str().unwrap();
            std::fs::create_dir_all(base_path).unwrap();
            config_path = format!("{}\\lgconfig.json", base_path);
        }

        config_path
    }
}
