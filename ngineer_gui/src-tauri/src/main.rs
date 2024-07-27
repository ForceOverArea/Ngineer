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
    // Global state
    let debugger_open = false;

    // File Submenu
    let open_project = CustomMenuItem::new("open_project".to_string(),  "Open Project..."   );
    let new_project  = CustomMenuItem::new("new_project".to_string(),   "New Project..."    );
    let new_file     = CustomMenuItem::new("new_file".to_string(),      "New File..."       );
    let file = Submenu::new("File", Menu::new()
        .add_item(open_project)
        .add_item(new_project)
        .add_item(new_file));

    let debug_mode   = CustomMenuItem::new("debug_mode".to_string(),    "Open Debug Console");
    let help = Submenu::new("Help", Menu::new()
        .add_item(debug_mode));

    // Make "file menu" bar
    let menu = Menu::new()
        .add_submenu(file)
        .add_submenu(help);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(move |event| 
        {
            match event.menu_item_id()
            {
                "open_project" => 
                {
                    println!("clicked open_project!");
                },
                "new_project" => 
                {
                    println!("clicked new_project!");
                },
                "new_file" => 
                { 
                    println!("clicked new_file!");
                    event.window()
                        .emit("new-file-button-clicked", "")
                        .unwrap(); 
                },
                "debug_mode" => 
                {
                    if debugger_open
                    {
                        event.window().close_devtools();
                    }
                    else
                    {
                        event.window().open_devtools();
                    }
                }
                _ => {},
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
