use std::sync::Arc;

use crate::{
    app_config::AppConfig,
    app_status::AppStatus,
    custom_provider::CustomProvider,
    ddragon::{self, DDragon},
    lcu_driver::LcuDriver,
    source_provider::{Build, DynSource, GameMode, Lane, LolRuneItem},
};
use futures_util::lock::Mutex;
use serde_json::Value;
use tauri::{AppHandle, Manager};
use tokio::{join, try_join};
use tracing::{error, info, instrument};

#[instrument]
#[tauri::command]
pub async fn lcu_get(lcu: tauri::State<'_, LcuDriver>, url: String) -> Result<Value, String> {
    lcu.get(&url).await.map_err(|err| {
        error!(error= %err);
        err.to_string()
    })
}

#[instrument]
#[tauri::command]
pub async fn lcu_post(
    lcu: tauri::State<'_, LcuDriver>,
    url: String,
    body: Value,
) -> Result<Value, String> {
    lcu.post(&url, &body).await.map_err(|err| {
        error!(error= %err);
        err.to_string()
    })
}

#[instrument]
#[tauri::command]
pub async fn lcu_put(
    lcu: tauri::State<'_, LcuDriver>,
    url: String,
    body: Value,
) -> Result<Value, String> {
    lcu.put(&url, &body).await.map_err(|err| {
        error!(error= %err);
        err.to_string()
    })
}

#[instrument]
#[tauri::command]
pub async fn lcu_patch(
    lcu: tauri::State<'_, LcuDriver>,
    url: String,
    body: Value,
) -> Result<Value, String> {
    lcu.patch(&url, &body).await.map_err(|err| {
        error!(error= %err);
        err.to_string()
    })
}

#[instrument]
#[tauri::command]
pub async fn lcu_delete(lcu: tauri::State<'_, LcuDriver>, url: String) -> Result<Value, String> {
    lcu.delete(&url).await.map_err(|err| {
        error!(error= %err);
        err.to_string()
    })
}

#[tauri::command]
#[instrument]
pub async fn initialize(
    app: tauri::AppHandle,
    lcu: tauri::State<'_, LcuDriver>,
) -> Result<(), String> {
    lcu.hook_on_window(app).await;
    Ok(())
}

#[tauri::command]
#[instrument]
pub async fn get_app_status(
    app_status: tauri::State<'_, Arc<Mutex<AppStatus>>>,
) -> Result<AppStatus, String> {
    Ok(app_status.lock().await.clone())
}

#[tauri::command]
#[instrument]
pub async fn get_champion_build(
    champion_name: String,
    lane: String,
    mode: String,
    provider: tauri::State<'_, Mutex<DynSource>>,
    custom: tauri::State<'_, CustomProvider>,
) -> Result<Build, String> {
    if lane == "custom" {
        info!("Get custom from local..");
        let runes = custom
            .get_champion_runes(&champion_name, GameMode::from_str(&mode))
            .await;
        return Ok(Build {
            runes,
            spells: Vec::new(),
            is_cache: true,
            is_custom: true,
            lane,
        });
    }

    let info = provider
        .lock()
        .await
        .get_champion_info(
            &champion_name,
            Lane::from_str(&lane).unwrap(),
            GameMode::from_str(&mode),
        )
        .await
        .unwrap();

    let (runes, spells) = join!(info.get_runes(), info.get_spells());
    let runes = match runes {
        Ok(runes) => runes,
        Err(err) => {
            error!(error=%err);
            return Err(err);
        }
    };

    let spells = match spells {
        Ok(spells) => spells,
        Err(err) => {
            error!(error=%err);
            return Err(err);
        }
    };

    Ok(Build {
        runes,
        spells,
        is_cache: info.is_cache(),
        is_custom: false,
        lane,
    })
}

#[tauri::command]
#[instrument]
pub async fn get_champion_all_build(
    champion_name: String,
    mode: String,
    provider: tauri::State<'_, Mutex<DynSource>>,
) -> Result<Vec<Build>, String> {
    let builds: Arc<Mutex<Vec<Build>>> = Arc::new(Mutex::new(Vec::with_capacity(5)));
    let p = provider.lock().await;
    let all = try_join!(
        p.get_champion_info(&champion_name, Lane::Top, GameMode::from_str(&mode)),
        p.get_champion_info(&champion_name, Lane::Jungle, GameMode::from_str(&mode)),
        p.get_champion_info(&champion_name, Lane::Mid, GameMode::from_str(&mode)),
        p.get_champion_info(&champion_name, Lane::Bot, GameMode::from_str(&mode)),
        p.get_champion_info(&champion_name, Lane::Support, GameMode::from_str(&mode)),
    );
    let (top, jg, mid, bot, sp) = all.unwrap();
    let lanes = [top, jg, mid, bot, sp];
    for lane in lanes {
        let builds = builds.clone();
        tokio::spawn(async move {
            let (runes, spells) = join!(lane.get_runes(), lane.get_spells());
            let runes = match runes {
                Ok(runes) => runes,
                Err(err) => {
                    error!(error=%err);
                    return;
                }
            };

            let spells = match spells {
                Ok(spells) => spells,
                Err(err) => {
                    error!(error=%err);
                    return;
                }
            };
            builds.lock().await.push(Build {
                runes,
                spells,
                is_cache: lane.is_cache(),
                is_custom: false,
                lane: lane.get_lane().to_string(),
            });
        })
        .await
        .unwrap();
    }

    let builds = &*builds.lock().await;
    Ok(builds.to_owned())
}

#[tauri::command]
#[instrument]
pub async fn clear_cache(provider: tauri::State<'_, Mutex<DynSource>>) -> Result<(), String> {
    provider.lock().await.clear_cache().await;

    Ok(())
}

#[tauri::command]
#[instrument]
pub async fn get_champion_raw_info(
    ddragon: tauri::State<'_, DDragon>,
    champion_id: String,
) -> Result<ddragon::Champion, String> {
    ddragon.get_champion_information(&champion_id)
}

#[tauri::command]
#[instrument]
pub async fn get_ddragon_version(
    ddragon: tauri::State<'_, DDragon>,
    champion_id: String,
) -> Result<String, String> {
    Ok(ddragon.get_version())
}

#[tauri::command]
#[instrument]
pub async fn get_champion_icon(
    ddragon: tauri::State<'_, DDragon>,
    champion_id: String,
) -> Result<String, String> {
    let id = ddragon.get_champion_information(&champion_id).unwrap().id;
    Ok(format!(
        "http://ddragon.leagueoflegends.com/cdn/{}/img/champion/{}.png",
        ddragon.get_version(),
        id
    ))
}

#[tauri::command]
#[instrument]
pub async fn add_champion_custom_rune(
    custom: tauri::State<'_, CustomProvider>,
    champion_name: String,
    mode: String,
    rune_item: LolRuneItem,
) -> Result<bool, String> {
    Ok(custom
        .add_champion_rune(&champion_name, GameMode::from_str(&mode), rune_item)
        .await)
}

#[tauri::command]
#[instrument]
pub async fn remove_champion_custom_rune(
    custom: tauri::State<'_, CustomProvider>,
    champion_name: String,
    mode: String,
    id: i64,
) -> Result<bool, String> {
    Ok(custom
        .remove_champion_rune(&champion_name, GameMode::from_str(&mode), id)
        .await)
}

#[tauri::command]
#[instrument]
pub async fn remove_champion_custom_runes(
    custom: tauri::State<'_, CustomProvider>,
    champion_name: String,
    mode: String,
) -> Result<bool, String> {
    Ok(custom
        .remove_champion_runes(&champion_name, GameMode::from_str(&mode))
        .await)
}

#[tauri::command]
#[instrument]
pub async fn get_app_config(
    config: tauri::State<'_, Mutex<AppConfig>>,
) -> Result<AppConfig, String> {
    Ok(config.lock().await.clone())
}

#[tauri::command]
#[instrument]
pub async fn set_app_config(
    config: tauri::State<'_, Mutex<AppConfig>>,
    handle: AppHandle,
    new_config: AppConfig,
) -> Result<(), String> {
    config.lock().await.clone_from(&new_config);
    handle.emit_all("app_config_changed", new_config).unwrap();
    Ok(())
}

#[tauri::command]
#[instrument]
pub async fn save_app_config(
    config: tauri::State<'_, Mutex<AppConfig>>,
    provider: tauri::State<'_, Mutex<DynSource>>,
) -> Result<(), String> {
    config.lock().await.save_to_local().await;
    let mut provider = &mut *(provider.lock().await);
    config.lock().await.invoke(&mut provider).await;
    Ok(())
}

#[tauri::command]
#[instrument]
pub async fn log(msg: String) {
    info!(msg);
}
