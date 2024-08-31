// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{self, OpenOptions};
use std::io::{self, Write};

const FILE_NAME: &str = "../todos.txt";

fn file_writer(contents: &str) -> Result<(), io::Error> {
    let mut file = OpenOptions::new().write(true).truncate(true).create(true).open(FILE_NAME)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

fn get_file_contents() -> Result<String, io::Error> {
    fs::read_to_string(FILE_NAME)
}

#[tauri::command]
fn get_items() -> Result<Vec<String>, String> {
    get_file_contents()
        .map(|contents| contents.split(',').map(String::from).collect())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn add_item(new_item: &str) -> Result<String, String> {
    match (|| -> Result<String, io::Error> {
        // Read the contents of the file
        let mut contents = get_file_contents()?;

        // Split the contents by commas to get the list of items
        let mut items: Vec<&str> = contents.split(',').collect();
        
        // Append the new item to the list
        let _ = new_item.trim();
        if new_item.is_empty() {
            return Ok(contents);
        }
        items.push(new_item);
        
        // Join the list back into a comma-separated string
        if !items.is_empty() {
            contents = items.join(",");
        }
        
        // Write the updated string back to the file
        file_writer(&contents)?;
        
        Ok(contents)
    })() {
        Ok(contents) => Ok(contents.into()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn clear_items() -> Result<(), String> {
    file_writer("")
        .map_err(|e| e.to_string())
        .map(|_| ())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![add_item, clear_items, get_items])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}