use std::{env, fs, path::{Path, PathBuf}};
use crate::tools::{input, get_home};
use reqwest;

/// Ensure that the directory is a proper `newsletter` directory.
pub fn is_valid_dir(path: &PathBuf) -> bool {
    let mut content_exists = false;
    let mut env_exists = false;

    if path.exists() {
        for item in path.read_dir().unwrap() {
            let unwrapped_item = item.unwrap().file_name().into_string().unwrap();
    
            if unwrapped_item.contains(".env") {
                env_exists = true;
            }
            else if unwrapped_item.contains("content.txt") {
                content_exists = true;
            }
        }
    }

    content_exists && env_exists
}

pub fn setup(path: String, email: Option<String>, domain: Option<String>, api_key: Option<String>, show: bool) {
    if show {
        return show_config();
    }

    let newsletter_path = Path::new(&path).join("Newsletter");
    let env_path = Path::new(&path).join("Newsletter/.env");

    if !is_valid_dir(&newsletter_path) {
        println!("Valid `Newsletter` folder was not found in home directory. Creating...");
        fs::create_dir_all(&newsletter_path).unwrap();

        fs::write(&newsletter_path.join(".env"), "").unwrap();
        fs::write(&newsletter_path.join("content.txt"), "# Use this file to write the content of your email.\n# \n# Note: Lines that start with `#` are skipped.\n# \n# If you need any help, contact Carlos Valdez.\n# Have fun!\nSUBJECT=!!! INSERT TITLE HERE !!!\n").unwrap();
    } else {
        println!("Valid `Newsletter` folder found!");
    }

    let mut env_content = String::new();

    match email {
        Some(e) => {
            println!("Editing email...");
            env_content.push_str(format!("EMAIL={e}\n").as_str());
        },
        None => {}
    }

    match domain {
        Some(d) => {
            println!("Editing domain...");
            env_content.push_str(format!("DOMAIN={d}\n").as_str());
        },
        None => {}
    }

    match api_key {
        Some(a) => {
            println!("Editing API key...");
            env_content.push_str(format!("API_KEY={a}\n").as_str());
        }
        None => {}
    }

    for line in fs::read_to_string(&env_path).unwrap().split("\n") {

        if !env_content.contains("API_KEY") && line.contains("API_KEY") {
            env_content.push_str(format!("{line}\n").as_str());
        }
        else if !env_content.contains("DOMAIN") && line.contains("DOMAIN") {
            env_content.push_str(format!("{line}\n").as_str());
        }
        else if !env_content.contains("EMAIL") && line.contains("EMAIL") {
            env_content.push_str(format!("{line}\n").as_str());
        }
    }

    fs::write(&env_path, env_content).unwrap();
}

fn load_envs() {
    let env_path = Path::new(get_home().as_str()).join("Newsletter/.env");
    let c = fs::read_to_string(env_path).unwrap();

    for line in c.split("\n") {
        if line.len() > 0 {
            // https://www.reddit.com/r/rust/comments/thz3g4/comment/i1arbn7/
            let items = line.split("=").collect::<Vec<_>>();
            env::set_var(items[0], items[1]);
        }

        
    }
}

fn show_config() {
    let env_path = Path::new(get_home().as_str()).join("Newsletter/.env");

    if !env_path.exists() {
        println!("Could not find a `.env` file in {:?}. Did you run the setup?\n\nSuggestion: Run `ojos newsletter config`", env_path);
        return;
    }

    let c = fs::read_to_string(&env_path).unwrap();
    println!();

    for line in c.split("\n") {
        if line.len() > 0 {
            // https://www.reddit.com/r/rust/comments/thz3g4/comment/i1arbn7/
            let items = line.split("=").collect::<Vec<_>>();
            env::set_var(items[0], items[1]);
            
            if items[0] == "API_KEY" {
                println!("API_KEY = REDACTED.");
            } else {
                println!("{}", format!("{key} = {value}", key = items[0], value = items[1]))
            }
        }
    }

    println!("\nCheck the full configuration in {:?}.", env_path)

}

fn cleanup(content: &String) -> Vec<String> {
    let mut subject: String = String::new();
    let mut content_to_send = String::new();

    for line in content.split("\n") {
        if !line.starts_with("#") && !line.starts_with("SUBJECT") {
            content_to_send.push_str(format!("{line}\n").as_str())
        }
        else if line.starts_with("SUBJECT") {
            subject = line.split("=").collect::<Vec<_>>()[1].to_string();
        }
    }

    vec![subject, content_to_send.trim().to_string()]
}

pub fn batch_send(for_newsletter: String) {
    load_envs();
    let email: String;
    let domain: String;
    let api_key: String;
    let newsletter = Path::new(get_home().as_str()).join("Newsletter/");


    match env::var("EMAIL") {
        Ok(e) => {email = e;}
        Err(_) => {
            println!("Error: EMAIL environment variable was not found. Set it with `ojos newsletter config --email <Name <email@example.com>>`");
            std::process::exit(1);
        },
    }

    match env::var("DOMAIN") {
        Ok(d) => {domain = d}
        Err(_) => {
            println!("Error: DOMAIN environment variable was not found. Set it with `ojos newsletter config --domain <DOMAIN>`");
            std::process::exit(1);
        }
    }

    match env::var("API_KEY") {
        Ok(a) => {api_key = a},
        Err(_) => {
            println!("Error: API_KEY environment variable was not found. Set it with `ojos newsletter config --api_key <API_KEY>");
            std::process::exit(1);
        }
    }

    println!("\nRunning with these settings...\n");
    println!("Email: {}", email);
    println!("Domain: {}", domain);
    println!("Newsletter: {}", for_newsletter);
    println!("API key: REDACTED");

    let yes_no = input("\n\nDoes this look right? (N/y)");

    if yes_no.eq_ignore_ascii_case("y") {
        // actual sending happens here
        let subject_content = cleanup(&fs::read_to_string(newsletter.join("content.txt")).unwrap());

        println!("CONFIRMATION");
        println!("You're about to batch send an email. Here is the content you're about to send:\n");
        println!("--------------------------------------------------------------------------------");
        println!("SUBJECT: {}", subject_content[0]);
        println!("{}", subject_content[1]);
        println!("--------------------------------------------------------------------------------");

        if for_newsletter == "newsletter-testing" {
            println!("\nYou are sending TESTING email.");
        } else {
            println!("\nYOU ARE SENDING OFFICIAL EMAIL. this is not a test.");
        }

        let confirmed = input("Confirm by typing 'SEND EMAIL':");

        let form = reqwest::blocking::multipart::Form::new()
        .text("to", format!("{for_newsletter}@{domain}"))
        .text("from", email)
        .text("subject", subject_content[0].clone())
        .text("text", subject_content[1].clone());

        if confirmed == "SEND EMAIL" {
            let client = reqwest::blocking::Client::new();
            let res = client.post(format!("https://api.mailgun.net/v3/{domain}/messages"))
            .basic_auth("api", Some(api_key))
            .multipart(form)
            .send().unwrap();

        println!("Server responded: {}", res.status());
        println!("Server body: {}", res.text().unwrap());

        } else {
            println!("Operation cancelled.");
        }

    } else {
        println!("Abandoning...");
    }

}
