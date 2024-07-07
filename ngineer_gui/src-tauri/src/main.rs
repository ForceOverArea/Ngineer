// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Imports 
use tauri::{CustomMenuItem, Menu, Submenu};

#[derive(Clone, serde::Serialize)]
struct Payload
{
    message: String,
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
                        .emit("new-file-button-clicked", Payload { message: "NewFile.txt".to_string() })
                        .unwrap(); 
                },
                _ => {},
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
