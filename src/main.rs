mod lib;
use std::{fs, path};

fn main() {
    let args = lib::get_args();
    let path_to_file = path::PathBuf::from(args.value_of("file").expect("File not provided"));
    let key = args.value_of("key").expect("No key found");
    let meta = fs::metadata(&path_to_file).expect("Error reading file");

    if meta.is_dir() {
        let func = Box::new(|path_to_file| lib::run_operation(&path_to_file, &key));
        let _ = lib::walk_directory(&path_to_file, &func);
    } else {
        let _ = lib::run_operation(&path_to_file, &key);
    }
}
