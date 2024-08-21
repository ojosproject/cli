/*
main.rs
Ojos Project

Code for copying documentation.
*/

use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub fn copy(path: PathBuf) {
    println!("Given {:?}...", path);
    if !path.exists() {
        println!("The path does not exist: {:?}", path);
    } else if path.is_dir() {
        println!("Exploring folder {:?}...", path);

        for item in fs::read_dir(path).unwrap() {
            let unwrapped = item.unwrap();

            copy(unwrapped.path());
        }
    } else {
        let content = fs::read_to_string(&path).unwrap();

        let cwd = env::current_dir().unwrap();

        let new_path = cwd.join(path.file_name().unwrap().to_str().unwrap());

        if !new_path.parent().unwrap().exists() {
            fs::create_dir_all(&new_path).unwrap();
        }

        fs::write(new_path, content).unwrap();
        println!("Copied {:?}", path);
    }
}

/// A recursive function to copy files to a path.
pub fn run(path: String) {
    let p = Path::new(&path);

    copy(p.to_path_buf());
}
