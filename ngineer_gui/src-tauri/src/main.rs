// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Imports 
use tauri::{CustomMenuItem, Menu, Submenu};

#[derive(Clone, serde::Serialize)]
struct Payload
{
    message: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() 
{
    // File Submenu
    let open_project = CustomMenuItem::new("open project".to_string(),  "Open Project..."   );
    let new_project  = CustomMenuItem::new("new project".to_string(),   "New Project..."    );
    let new_file     = CustomMenuItem::new("new file".to_string(),      "New File..."       );
    let file = Submenu::new("File", Menu::new()
        .add_item(open_project)
        .add_item(new_project)
        .add_item(new_file));

    // Make "file menu" bar
    let menu = Menu::new()
        .add_submenu(file);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| 
        {
            match event.menu_item_id()
            {
                "open project" => {},
                "new project" => {},
                "new file" => 
                { 
                    event.window()
                        .emit("NewFileButtonClicked", Payload { message: "NewFile.txt".to_string() })
                        .unwrap(); 
                },
                _ => {},
            }
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
