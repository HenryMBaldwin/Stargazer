#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{
  image::Image, include_image, menu::{MenuBuilder, MenuItemBuilder}, tray::{
    MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent
  }, Manager, WindowEvent
};
use stargazer::libinstance::instance::{ClientInstance, generate_id};
use chrono::Utc;
use tauri_plugin_updater::UpdaterExt;
use tauri_plugin_shell::ShellExt;
mod pipe_client;

const TRAY_ICON: Image<'_> = include_image!("./icons/icon.ico");
fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
            let webview_window = app.get_webview_window("main").unwrap();
            let _ = webview_window.show();
            if webview_window.is_minimized().unwrap() {
              let _ = webview_window.unminimize();
            }
            let _ = webview_window.set_focus();
    }))
    .plugin(tauri_plugin_updater::Builder::new().build())
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_shell::init())
    //.manage(client)
    .setup(|app| {
      //check for updates on startup and then every 10 minutes
      let handle = app.handle().clone();
      tauri::async_runtime::spawn(async move {
          let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600));
          loop {
              interval.tick().await;
              println!("Client: checking for updates");
              update(handle.clone()).await.unwrap();
          }
      });
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
      pipe_client::switch_database,
      pipe_client::logout,
      pipe_client::get_server_version,
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

async fn update(app: tauri::AppHandle) -> anyhow::Result<()> {
  let app_clone = app.clone();
  if let Some(update) = app.updater_builder().on_before_exit(move || {
    // Run the async block synchronously 
    let app_clone_clone = app_clone.clone();
    tauri::async_runtime::spawn(async move {
        app_clone_clone.shell().command("taskkill")
            .args(&["/IM", "stargazer_server.exe", "/F"])
            .output()
            .await
            .unwrap();
    });
    })
    .build()?
    .check()
    .await? {
    
    let mut downloaded = 0;
    update.download_and_install(|chunk_length, content_length| {
      downloaded += chunk_length;
      println!("downloaded {downloaded} from {content_length:?}");
    }, || {
      println!("download finished");
    }).await?;

    println!("update installed");
    app.restart();
  }

  Ok(())
}