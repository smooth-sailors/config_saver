
use std::fs;
use rfd::FileDialog;
use std::path::PathBuf;
use std::collections::HashMap;

fn main()
{
    let file : Option<PathBuf> = load_file();
    let file_data : String = get_data_from_file(file);
    let keyed_data = key_and_value_data(&file_data);
    write_map_to_json(&keyed_data, "output.json");
}

// Choosing the file to load via a dialog menu.
fn load_file() -> Option<PathBuf>
{
    FileDialog::new()
        .set_directory("/")
        .set_title("Choose a File")
        .pick_file()
}

// Getting the data from the passed file buffer.
// Will return an empty string if it couldn't read the data.
// Will return a string with the contents "Empty file." if there was no data.
fn get_data_from_file(file: Option<PathBuf>) -> String
{
    match file {
        Some(path)  => { fs::read_to_string(path).unwrap_or_default() },
        None        => { String::from("Empty file.") }
    }
}

// Will return a hashmap of the data that's passed in after it keys and values
// everything within the lines of data.  Empty values and empty keys will be
// returned as empty strings "".
fn key_and_value_data(data: &String) -> HashMap<String, String>
{
    let mut map = HashMap::new();

    for line in data.lines() {

        // Skipping empty lines.
        if line.trim().is_empty() { continue; }

        // IMPORTANT NOTE: You could modify this line to add in multiple splits to receive
        // multiple parts, but if you do so then you'll need to modify the HashMap structure
        // you're using to have its value be a tuple or vector (this is so you can store multiple
        // values for a singular key).
        let parts: Vec<&str> = line.split('=').collect();
        map.insert(String::from(parts[0]), String::from(parts[1]));
    }

    map
}

//
fn write_map_to_json(map: &HashMap<String, String>, path: &str) {
    let json = serde_json::to_string_pretty(map).unwrap_or_default();
    fs::write(path, json);
}
