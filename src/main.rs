use std::{env::args, fs, path::PathBuf, process::exit, str::FromStr};

struct Config {
    show_all: bool,
    path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Config { show_all: false, path: PathBuf::from_str(".").unwrap() }
    }
}

fn main() {
    let args: Vec<String> = args().skip(1).collect();

    let config = match build_config(args) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            exit(1);
        }
    };

    if !config.path.is_dir() {
        eprintln!("Provided path is not a directory");
        exit(1);
    }

    let entries = match config.path.read_dir() {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Cannot read directory {}: {}", config.path.display(), e);
            exit(1);
        }
    };

    for entry in entries {
        match entry {
            Ok(e) => {
                let file_name = e.file_name();
                let name = file_name.to_string_lossy();

                if config.show_all || !name.starts_with(".") {
                    println!("{}", e.path().display());
                }
            },
            Err(e) => {
                eprintln!("Cannot read entry: {}", e);
            }
        }
    }
}


fn build_config(args: Vec<String>) -> Result<Config, String> {
    let mut config = Config::default();

    for arg in args {
        match arg.as_str() {
            "-a" | "--show-all" => config.show_all = true,
            string if !string.starts_with('-') => {
                config.path = PathBuf::from(string);
                if !config.path.exists() {
                    return Err(format!("Provided path does not exist: {}", config.path.display()));
                }
            }
            _ => return Err(format!("Unknown argument: {}", arg)),
        }
    }

    Ok(config)
}