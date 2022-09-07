#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{sync::Arc, time::Duration};

use futures_util::lock::Mutex;
use lcu_driver::{LcuDriver, LcuDriverConfig};
use tauri::{async_runtime::block_on, Manager};
use tracing::{error, Level};
use tracing_subscriber::FmtSubscriber;
use window_shadows::set_shadow;

use crate::{
    app_config::AppConfig,
    app_status::AppStatus,
    ddragon::{DDragon, Language}, source_provider::{opgg::OPGG, DynSource}, custom_provider::CustomProvider,
};

mod app_config;
mod app_status;
mod ddragon;
mod for_render;
mod lcu_driver;
mod source_provider;
mod ws;
mod tray;
mod custom_provider;
mod util;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    tracing::info!("Starting tracing!");

    let mut builder = tauri::Builder::default().setup(|app| {
        #[cfg(debug_assertions)] // only include this code on debug builds
        {
            let window = app.get_window("main").unwrap();
            window.open_devtools();
            window.close_devtools();
        }
        let window = app.get_window("main").unwrap();
        set_shadow(&window, true).expect("Unsupported platform!");
        Ok(())
    });

    let config = LcuDriverConfig::from_client();
    let app_status = Arc::new(Mutex::new(AppStatus::default()));
    let mut watch_config = false;
    match config {
        Ok(config) => {
            block_on(app_status.lock()).lcu_loaded = true;
            builder = builder.manage(block_on(LcuDriver::connect(
                &config,
                Duration::from_secs(5),
            )))
        }
        Err(err) => {
            error!(err);
            watch_config = true;
        }
    };
    let custom = CustomProvider::initialize();
    let ddragon = block_on(DDragon::new(Language::en_US));
    let mut opgg: DynSource = Box::new(block_on(OPGG::new()));
    let config = block_on(AppConfig::from_local());
    block_on(config.invoke(&mut opgg));
    let config = Mutex::new(config);
    let app = builder
        .manage(ddragon)
        .manage(Mutex::new(opgg))
        .manage(app_status.clone())
        .manage(config)
        .manage(custom)
        .system_tray(tray::create_tray())
        .on_system_tray_event(tray::handle_event)
        .invoke_handler(tauri::generate_handler![
            for_render::initialize,
            for_render::get_app_status,
            for_render::lcu_get,
            for_render::lcu_post,
            for_render::lcu_put,
            for_render::lcu_patch,
            for_render::lcu_delete,
            for_render::add_champion_custom_rune,
            for_render::remove_champion_custom_rune,
            for_render::remove_champion_custom_runes,
            for_render::get_champion_build,
            for_render::get_champion_all_build,
            for_render::get_champion_raw_info,
            for_render::get_ddragon_version,
            for_render::get_champion_icon,
            for_render::get_app_config,
            for_render::set_app_config,
            for_render::save_app_config,
            for_render::clear_cache,
            for_render::log,
        ])
        .build(tauri::generate_context!())
        .expect("Can't run tauri application.");

    if watch_config {
        let a = app_status.clone();
        let handle = app.handle();
        std::thread::spawn(move || {
            let mut config = LcuDriverConfig::from_client();
            while let Err(_) = config {
                std::thread::sleep(Duration::from_secs(1));
                config = LcuDriverConfig::from_client();
            }
            let config = config.unwrap();

            handle.manage(block_on(LcuDriver::connect(
                &config,
                Duration::from_secs(5),
            )));

            let mut am = block_on(a.lock());
            am.lcu_loaded = true;
            handle.emit_all("app_status_changed", am.clone()).unwrap();
        });
    }
    app.run(|_app_handle, _event| {});
}
