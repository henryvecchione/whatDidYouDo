// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{self, OpenOptions};
use std::io::{self, Write};

#[tauri::command]
fn add_item(new_item: &str) -> Result<String, String> {
    match (|| -> Result<String, io::Error> {
        // Read the contents of the file
        let mut contents = fs::read_to_string("todos.txt").unwrap_or_else(|_| String::new());
        
        // Split the contents by commas to get the list of items
        let mut items: Vec<&str> = contents.split(',').collect();
        
        // Append the new item to the list
        items.push(new_item);
        
        // Join the list back into a comma-separated string
        contents = items.join(",");
        
        // Write the updated string back to the file
        let mut file = OpenOptions::new().write(true).truncate(true).open("todos.txt")?;
        file.write_all(contents.as_bytes())?;
        
        Ok(contents)
    })() {
        Ok(contents) => Ok(contents.into()),
        Err(_) => Err("couldn't process file".to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![add_item])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}