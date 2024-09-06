#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{
  image::Image, include_image, menu::{MenuBuilder, MenuItemBuilder}, tray::{
    MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent
  }, Manager, WindowEvent
};
use stargazer::libinstance::instance::{ClientInstance, generate_id};
use chrono::Utc;
mod pipe_client;

const TRAY_ICON: Image<'_> = include_image!("./icons/icon.ico");
fn main() {
  //client instance representing this client
  let client = ClientInstance {
    time: Utc::now(),
    id: generate_id(),
    version: env!("VERSION").to_string(),
  };

  tauri::Builder::default()
    .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
            let webview_window = app.get_webview_window("main").unwrap();
            let _ = webview_window.show();
            if webview_window.is_minimized().unwrap() {
              let _ = webview_window.unminimize();
            }
            let _ = webview_window.set_focus();
    }))
    .manage(client)
    .setup(|app| {

      let hide_item = MenuItemBuilder::with_id("hide", "Hide").build(app)?;
      let show_item = MenuItemBuilder::with_id("show", "Show").build(app)?;
      //system tray and menu
      let menu = MenuBuilder::new(app)
        .items(&[&hide_item, &show_item])
        .separator()
        .quit()
        .build()?;
      let tray = TrayIconBuilder::new()
        .icon(TRAY_ICON)
        .tooltip("Stargazer")
        .menu(&menu)
        .on_menu_event(move |app, event| match event.id().as_ref() {
          "hide" => {
            let webview_window = app.get_webview_window("main").unwrap();
            let _ = webview_window.hide();
          },
          "show" => {
            let webview_window = app.get_webview_window("main").unwrap();
            let _ = webview_window.show();
            if webview_window.is_minimized().unwrap() {
              let _ = webview_window.unminimize();
            }
            let _ = webview_window.set_focus();
          },
          _ => {},
        })
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
              if webview_window.is_minimized().unwrap() {
                let _ = webview_window.unminimize();
              }
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
      pipe_client::get_query_log,
      pipe_client::get_databases,
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
