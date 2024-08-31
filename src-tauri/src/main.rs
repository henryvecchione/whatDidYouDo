// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{self, OpenOptions};
use std::io::{self, Write};

const FILE_NAME: &str = "../todos.txt";

fn write_to_file(contents: &str) -> Result<(), io::Error> {
    let contents = contents.as_bytes();
    let mut file = OpenOptions::new().write(true).truncate(true).create(true).open(FILE_NAME)?;
    file.write_all(contents)?;
    Ok(())
}

fn read_file_as_string() -> Result<String, io::Error> {
    fs::read_to_string(FILE_NAME)
}

fn read_csv_into_vec() -> Result<Vec<String>, io::Error> {
    let f_str = read_file_as_string()?;
    if f_str.is_empty() {
        return Ok(Vec::new());
    }
    let f_vec = f_str.split(",").map(|s| s.to_string()).collect();
    Ok(f_vec)
}

fn write_vec_to_file(items: Vec<String>) -> Result<(), io::Error> {
    let contents = items.join(",");
    write_to_file(&contents)
}

#[tauri::command]
fn get_items() -> Result<Vec<String>, String> {
    read_csv_into_vec()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn add_item(new_item: &str) -> Result<Vec<String>, String> {
    match (|| -> Result<Vec<String>, io::Error> {
        // Read the contents of the file
        let mut contents = read_csv_into_vec()?;
        
        // Append the new item to the list
        let _ = new_item.trim();
        if new_item.is_empty() {
            return Ok(contents);
        }
        contents.push(new_item.into());
        
        // Write the new contents back to the file
        write_vec_to_file(contents.clone())?;
        
        Ok(contents)
    })() {
        Ok(contents) => Ok(contents),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn clear_items() -> Result<(), String> {
    write_to_file("")
        .map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![add_item, clear_items, get_items])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}