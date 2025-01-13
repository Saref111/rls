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

    let config = build_config(args);
    let entries = match config.path.read_dir() {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Cannot read directory {}: {}", config.path.display(), e);
            exit(1);
        }
    };

    entries.for_each(|e| println!("{}", e.unwrap().path().display()));
}


fn build_config(args: Vec<String>) -> Config {
    args.into_iter().fold( Config::default(), |mut acc, it| {
        match it.as_str() {
            "-s" | "--show-all" => acc.show_all = true,
            string if !string.starts_with("-") => acc.path = PathBuf::from_str(string).expect("Provided incorrect path"), 
            _ => {}
        };

        acc
    })
}