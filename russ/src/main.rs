use serde_json::Value;
use std::fs::File;
use std::io::Read;

fn main() {
    // Open the JSON file
    let mut file = File::open("./../gpx_attribute_map.json").expect("Failed to open file");

    // Read the content of the file into a String
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    // Parse the JSON content into a serde_json::Value object
    let json_value: Value = serde_json::from_str(&contents).expect("Failed to parse JSON");

    // Print the entire dictionary
    println!("{:#}", json_value);
    println!("{}", json_value["power"]);
}


// use std::env;
// use serde_json::{Value};
// use std::fs::File;
// 
// use serde::{Deserialize, Serialize};
// use std::fs::File;
// use std::io::Read;
// 
// 
//  
//  
// fn main() {
// 	let fp = "./../gpx_attribute_map.json";
// 	let file = File::open(fp);
// 	let mut contents = String::new();
// 	file.read_to_string(&mut contents);
// 
//  	let v: Value = serde_json::from_str(contents)?;
// 
//     // Access parts of the data by indexing with square brackets.
//     println!("Please be {}", v["power"]);
// 
//     // Ok(())
// }
