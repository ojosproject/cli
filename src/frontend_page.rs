use std::{env, fs, io::stdin, path::PathBuf};

fn input(message: &str) -> String {
    let mut input = String::new();

    println!("{}", message);
    stdin().read_line(&mut input).expect("Failed to read input.");

    String::from(input.trim())
}

fn create_files(path: PathBuf, name: &String) {
    // components/
    fs::create_dir_all(path.join(format!("{}/components/", name))).expect("Creating the components directory failed.");
    // .tsx
    fs::write(path.join(format!("{}/page.tsx", name)), "// page.tsx\n// Ojos Project\n//\n// Enter a description of this page here!\n").expect("Failed to write .tsx file.");
    // .module.css
    fs::write(path.join(format!("{}/page.module.css", name)), "/*\npage.module.css\nOjos Project\n\nEnter a description of this page here!\n*/\n").expect("Failed to write .module.css file.");
}

pub fn create_page(page_name: Option<String>, dir: String, y: bool) {
    let mut path = env::current_dir().expect("Current directory could not be found.");
    let name: String;
    
    if dir == "src/app/" {
        path = path.join(dir);

        match page_name {
            Some(f) => {name = f},
            None => {name = input("Set a name for the page:")}
        }
        
    } else {
        // dir was specified
        path = path.join(dir);

        match page_name {
            Some(f) => {name = f},
            None => {name = ".".to_string()}
        }
    }

    if y {
        create_files(path, &name)
    } else {
        println!("\nWARNING: Will create these files.");
        let extensions = [".tsx", ".module.css"];
        for extension in extensions {
            println!("{:?}", path.join(format!("{name}/page{extension}")));
        }
        println!("{:?}", path.join(format!("{name}/components/")));
    
        let yes_no = input("\n\nDoes this look right? (y/N)");
    
        if yes_no.eq_ignore_ascii_case("y") {
            create_files(path, &name);
            println!("Created!")
        } else {
            println!("Abandoning...")
        }
    }
}