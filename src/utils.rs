use chrono::prelude::*;
use chrono::Local;
use std;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::env;

pub fn format_date(date: &String) -> (String) {
    let formatted_date = Local
        .datetime_from_str(&date, "%Y-%m-%dT%H:%M:%SZ")
        .unwrap()
        .format("%H:%M, %a %b %e");
    formatted_date.to_string()
}

pub fn read_file(file_name: String) -> String {
    let mut path = env::home_dir().unwrap();
    path.push("footy-cli");
    path.push(&file_name);
    let file = File::open(path);
    let mut result = match file {
        Ok(file) => file,
        Err(_err) => {
            eprintln!("No configuration file found, run \'footy configure\'");
            process::exit(2);
        }
    };
    let mut contents = String::new();
    result
        .read_to_string(&mut contents)
        .expect("Problem reading file");
    contents
}

pub fn write_file(name: String, content: String) -> std::io::Result<()> {
    let mut path = env::home_dir().unwrap();
    path.push("footy-cli");
    fs::create_dir_all(&path)?;
    path.push(&name);
    let mut file = File::create(path)?;
    file.write_all(&content.into_bytes())?;
    Ok(())
}
