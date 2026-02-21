
use std::fs;
use std::io;
use std::path::PathBuf;
use indexmap::IndexMap;
use rfd::FileDialog;

fn main()
{
    // Choose a file and obtain its data.
    let file_to_convert : Option<PathBuf> = load_text_file();
    let file_data : String = get_data_from_file(file_to_convert);

    // Specify how the data is being split and then map the data onto a IndexMap.
    println!("Please input a string that is being used to split the keys and values in your file: ");
    let splitter: String = obtain_input();
    let keyed_data = key_and_value_data(&file_data, &splitter);

    // Take the map and write it to a JSON file for any language to easily load later.
    let location_for_json_file : Option<PathBuf> = pick_save_path_for_json();
    write_map_to_json(&keyed_data, location_for_json_file);
}

// Choosing the file to load via a dialog menu.
fn load_text_file() -> Option<PathBuf>
{
    FileDialog::new()
        .add_filter("TXT, CSV, MD", &["txt", "csv", "md"])        // Dialog will only show .txt files and folders.
        .set_directory("/")                                       // Dialog starts at root.
        .set_title("Choose a File")                               // Dialog's title.
        .pick_file()                                              // Users can only select one file.
}

// Choosing where to save a JSON file.
fn pick_save_path_for_json() -> Option<PathBuf>
{
    FileDialog::new()
        .add_filter("JSON", &["json"])
        .set_directory("/")
        .set_title("Save JSON File")
        .set_file_name("new_configuration.json")
        .save_file()
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

// Will return an indexmap (it's a hashmap without serialization in order to keep the previous
// order of whatever you're mapping) of the data that's passed in after it keys and values
// everything within the lines of data.  Empty values and empty keys will be
// returned as empty strings "".
fn key_and_value_data(data: &String, split_sequence: &String) -> IndexMap<String, String>
{
    let mut map = IndexMap::new();

    for line in data.lines() {

        // Skipping empty lines.
        if line.trim().is_empty() { continue; }

        // IMPORTANT NOTE: You could modify this line to add in multiple splits to receive
        // multiple parts, but if you do so then you'll need to modify the IndexMap structure
        // you're using to have its value be a tuple or vector (this is so you can store multiple
        // values for a singular key).
        let parts: Vec<&str> = line.split(split_sequence).collect();
        map.insert(String::from(parts[0]), String::from(parts[1]));
    }

    map
}

// Takes a IndexMap with keys being strings and values being strings and converts the map into JSON.
fn write_map_to_json(map: &IndexMap<String, String>, file_path: Option<PathBuf>) {

    // Check to see if file_path is empty or not.
    // If it is, then break the function and throw error information out.
    // If it's not, then do nothing with the path and continue through the function.
    let valid_path = match file_path {
        Some(path)  => { path },
        None        => { println!("No path specified!"); return; }
    };

    let json = serde_json::to_string_pretty(map).unwrap_or_default();
    fs::write(valid_path, json);
}

// Gets input from user via terminal/console and trims out the newline character from using Enter.
// If no value is presented, then the default is "".
fn obtain_input() -> String {

    // Setting up a variable for the input and reading the input into the variable.
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_default();

    // Removing line feed and carriage return characters on the input if they are present.
    if input.ends_with('\n') {
        input.pop();
        if input.ends_with('\r') {
            input.pop();
        }
    }

    String::from(input)
}
