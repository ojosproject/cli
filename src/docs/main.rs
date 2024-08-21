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

        fs::write(new_path, content).unwrap();
        println!("Copied {:?}", path);
    }
}

/// A recursive function to copy files to a path.
pub fn run(path: String, include_readme: bool) {
    let p = Path::new(&path);

    if include_readme {
        let parent = p.parent().unwrap();
        if parent.join("README.md").exists() {
            println!("Given {:?}", p.parent().unwrap().join("README.md"));
            let cwd = env::current_dir().unwrap();
            let content = fs::read_to_string(p.parent().unwrap().join("README.md")).unwrap();

            println!("Copied {:?}", p.parent().unwrap().join("README.md"));
            fs::write(cwd.join("README.md"), content).unwrap();
        } else {
            println!("Could not find file: {:?}", parent.join("README.md"));
        }
    }

    copy(p.to_path_buf());
}
