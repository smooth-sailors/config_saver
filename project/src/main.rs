

use std::fs;
use rfd::FileDialog;
use std::path::PathBuf;
use std::collections::HashMap;

fn main()
{
    let file : Option<PathBuf> = select_file();
    let file_data : String = get_data_from_file(file);
    let keyed_data = key_and_value_data(&file_data);
    println!("{:?}", keyed_data);
}

// Choosing the file to load via a dialog menu.
fn select_file() -> Option<PathBuf>
{
    FileDialog::new()
        .set_directory("/")
        .set_title("Choose a File")
        .pick_file()
}

// Getting the data from the chosen file path.
// Will return an empty string if it couldn't read the data.
// Will return a string with the contents "Empty file." if there was no data.
fn get_data_from_file(file: Option<PathBuf>) -> String
{
    match file {
        Some(path) => {
            fs::read_to_string(path).unwrap_or_default()
        },
        None => {
            String::from("Empty file.")
        }
    }
}

fn key_and_value_data(data: &String) -> HashMap<String, String>
{
    let mut map = HashMap::new();

    for line in data.lines() {
        let parts: Vec<&str> = line.split('=').collect();
        map.insert(String::from(parts[0]), String::from(parts[1]));
    }

    map
}
