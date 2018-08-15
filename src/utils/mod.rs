use chrono::prelude::*;
use chrono::Local;
use std;
use std::fs::File;
use std::io::prelude::*;

pub fn format_date(date: &String) -> (String) {
    let formatted_date = Local
        .datetime_from_str(&date, "%Y-%m-%dT%H:%M:%SZ")
        .unwrap()
        .format("%H:%M, %a %b %e");
    formatted_date.to_string()
}

pub fn read_file(file_name: String) -> String {
    let mut file = File::open(file_name)
        .expect("Config file does not exist. Please run \'configure\' command");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");;
    contents
}

pub fn write_file(name: String, content: String) -> std::io::Result<()> {
    let mut file = File::create(name)?;
    file.write_all(&content.into_bytes())?;
    Ok(())
}
