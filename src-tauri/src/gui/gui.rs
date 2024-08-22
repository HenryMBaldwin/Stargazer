#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{
  menu::MenuBuilder,
  tray::{
    TrayIconBuilder,
    TrayIconEvent,
    MouseButton,
    MouseButtonState,
  },
  Manager,
  WindowEvent
};
//CustomMenuItem, SystemTrayMenu, SystemTrayEvent, 
use stargazer::libinstance::instance::{ClientInstance, generate_id};
use chrono::Utc;
mod pipe_client;


fn main() {
  //client instance representing this client
  let client = ClientInstance {
    time: Utc::now(),
    id: generate_id(),
    version: env!("VERSION").to_string(),
  };

  tauri::Builder::default()
    .manage(client)
    .setup(|app| {
      //system tray and menu
      let menu = MenuBuilder::new(app)
        .hide()
        .show_all()
        .quit()
        .build()?;
      let tray = TrayIconBuilder::new()
        .menu(&menu)
        .on_tray_icon_event(|tray, event| {
          if let TrayIconEvent::Click {
                  button: MouseButton::Left,
                  button_state: MouseButtonState::Up,
                  ..
          } = event
          {
              let app = tray.app_handle();
              if let Some(webview_window) = app.get_webview_window("main") {
              let _ = webview_window.show();
              let _ = webview_window.set_focus();
              }
          }
        })
        .build(app)?;
        
        //prevent close
        let webview_window = app.get_webview_window("main").unwrap();

        webview_window.clone().on_window_event(move |event| {
          if let WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            webview_window.hide().unwrap();
          }
        });
      Ok(())
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
