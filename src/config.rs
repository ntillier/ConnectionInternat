use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufRead};
use std::path::{Path, PathBuf};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Config {
    pub update: bool,
    pub version: String,
    pub username: String,
    pub password: String,
}

fn create_config(path: &Path) {
    let mut file = File::create(path).expect("[ERREUR]: Impossible de créer la configuration.\n");
    file.write_all(format!("{}\n", VERSION).as_bytes()).unwrap();
}

fn get_config_path() -> PathBuf {
    let home_dir = PathBuf::from(match env::var("HOME") {
        Ok(path) => path,
        Err(_) => match env::var("USERPROFILE") {
            Ok(path) => path,
            Err(_) => panic!("Home directory not found"),
        },
    });
    return home_dir.join(".internat-connection.txt");
}

impl Config {
    pub fn init() -> Self {
        let mut create = false;
        let mut update = false;
        let mut username = String::new();
        let mut password = String::new();

        let config_path = get_config_path();

        if config_path.exists() {
            let file =
                File::open(&config_path).expect("[ERREUR]: Impossible de lire la configuration.\n");
            let mut index = 0;

            for line in io::BufReader::new(file).lines().flatten() {
                match index {
                    0 => {
                        if line != VERSION {
                            update = true;
                            create_config(&config_path);
                            break;
                        }
                    }
                    1 => {
                        username = line;
                    }
                    2 => {
                        password = line;
                    }
                    _ => {
                        break;
                    }
                }
                index += 1;
            }
        } else {
            create_config(&config_path);
        }

        Self {
            update,
            username,
            password,
            version: VERSION.to_string(),
        }
    }

    pub fn save(&self) {
        let config_path = get_config_path();

        if config_path.exists() {
            let mut file = File::create(&config_path)
                .expect("[ERREUR]: Impossible de sauvegarder la configuration - 0.\n");
            file.write_all(
                format!("{}\n{}\n{}\n", VERSION, self.username, self.password).as_bytes(),
            )
            .expect("[ERREUR]: Impossible de sauvegarder la configuration - 1.\n");
        } else {
            create_config(&config_path);
        }
    }
}
