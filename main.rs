use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};
use std::env;

fn get_prefix_before_quote(string: &str) -> &str {
    let mut iter = string.split('"');
    if let Some(prefix) = iter.next() {
        return prefix;
    }
    string
}

fn line_exists_in_file(_alias_line: &str, _alias_name: &str) -> io::Result<bool> {
    let home_dir = env::var("HOME").expect("Failed to retrieve home directory.");
    let file = File::open(format!("{}/.zshrc", home_dir))?;
    let reader = BufReader::new(file);
    let alias_prefix = get_prefix_before_quote(_alias_line);
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.starts_with(alias_prefix) {
                println!("Alias \"{}\" already exists", _alias_name);
                return Ok(true);
            }
        }
    }

    Ok(false)
}

fn add_alias(_alias_line: &str) -> io::Result<()> {  
    let home_dir = env::var("HOME").expect("Failed to retrieve home directory.");
    let mut file = OpenOptions::new()
        .append(true)
        .open(format!("{}/.zshrc", home_dir))?;

    file.write_all(_alias_line.as_bytes())?;
    file.write_all(b"\n")?;

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let alias_name: &String = &args[1];

    if alias_name.to_lowercase() != "help" {
        let alias_command: &String = &args[2];
        println!(
            "Alias name: {}\nAlias command: {}",
            alias_name, alias_command
        );
        let alias_line = "alias ".to_string() + alias_name + "=\"" + alias_command + "\"";
        let line_exists = line_exists_in_file(&alias_line, &alias_name)?;
        if !line_exists {
            add_alias(&alias_line)?;
            println!("Alias added");
        }

    } else {
        println!("You stupid or something?");
    }
    Ok(())
}
