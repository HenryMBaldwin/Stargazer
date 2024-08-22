#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, Manager, WindowEvent};
use stargazer::libinstance::instance::{ClientInstance, generate_id};
use chrono::Utc;
use std::process::exit;
mod pipe_client;

#[tokio::main]
async fn main() {
  //client instance representing this client
  let client = ClientInstance {
    time: Utc::now(),
    id: generate_id(),
    version: env!("VERSION").to_string(),
  };

  //check if startup should continue
  match pipe_client::phone_home().await {
    Ok(response) => {
      if response {
        //no worries theres clearly already another client
        exit(0)
      }
    }
    Err(e) => {
      println!("Error contemplating!");
      exit(1)
    }
  }



  // Set up system tray menu
  let quit = CustomMenuItem::new("quit".to_string(), "Quit".to_string());
  let show = CustomMenuItem::new("show".to_string(), "Show".to_string());

  let tray_menu = SystemTrayMenu::new()
    .add_item(quit)
    .add_item(show);

  let system_tray = SystemTray::new()
    .with_menu(tray_menu);

  tauri::Builder::default()
    .manage(client)
    .setup(|app| {
      let main_window = app.get_window("main").unwrap();
      let main_window_clone = main_window.clone(); // Clone the window handle
      
      // Listen for the close request event and hide the window instead
      main_window.on_window_event(move |event| {
        if let WindowEvent::CloseRequested { api, .. } = event {
          // Prevent the window from closing
          api.prevent_close();
          // Hide the window instead
          main_window_clone.hide().unwrap();
        }
      });
      Ok(())
    })
    .system_tray(system_tray)
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => {
        match id.as_str() {
          "quit" => {
            app.exit(0);
          }
          "show" => {
            if let Some(window) = app.get_window("main") {
              window.show().expect("Failed to show window");
            }
          }
          _ => {}
        }
      },
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![
      pipe_client::login,
      pipe_client::check_auth,
      pipe_client::check_alive,
      pipe_client::get_query_log
    ])
    .build(tauri::generate_context!())
    .expect("error while running tauri application")
    .run(|_app_handle, event| match event {
      tauri::RunEvent::ExitRequested { api, .. } => {
        api.prevent_exit();
      }
      _ => {}
    });
}
