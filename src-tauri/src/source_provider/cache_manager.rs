use std::{path::{PathBuf}, fs};

use chrono::{DateTime, Utc, Duration};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};
use tracing::{info, error};

use crate::util::write_to_file;

use super::{GameMode, Lane};

#[derive(Debug, Clone)]
pub struct CacheManager {
    cache_dir: PathBuf,
    json: CacheLogJson,
    expired: i64,
}

impl CacheManager {
    pub fn new(cache_dir: PathBuf) -> Self {
        CacheManager {
            cache_dir,
            json: CacheLogJson::new(),
            expired: 3
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CacheLogJson {
    pub champions: Vec<ChampionCacheInfo>,
}

impl CacheLogJson {
    pub fn new() -> Self {
        CacheLogJson {
            champions: Vec::new(),
        }
    }
}

impl CacheManager {
    pub fn set_expired(&mut self, days: i64) {
        self.expired = days;
    }

    pub fn get_expired(&self)-> i64 {
        self.expired
    }

    pub async fn initialize() -> Result<Self, String> {
        if let Some(proj) = ProjectDirs::from("com", "Jinte", "Lola") {
            let cache_dir = proj.cache_dir().join("opgg");
            fs::create_dir_all(&cache_dir).unwrap();

            let cachelog = cache_dir.join("cache.log");
            if cachelog.is_file() {
                let file = File::open(cachelog).await;
                match file {
                    Ok(mut file) => {
                        let mut buff = String::new();
                        file.read_to_string(&mut buff)
                            .await
                            .map_err(|err| err.to_string())?;
                        let json: CacheLogJson =
                            serde_json::from_str(&buff).map_err(|err| err.to_string())?;

                        return Ok(CacheManager { cache_dir, json, expired: 3 });
                    }
                    Err(err) => {
                        error!("{:?}", err);
                        return Err(err.to_string());
                    }
                }
            } else {
                let file = File::create(cachelog).await;
                match file {
                    Ok(mut file) => {
                        let cl = CacheManager::new(cache_dir);
                        file.write(serde_json::to_string(&cl.json).unwrap().as_bytes())
                            .await
                            .map_err(|err| err.to_string())?;
                        return Ok(cl);
                    }
                    Err(err) => return Err(err.to_string()),
                }
            }
        } else {
            Err(String::from("Can't get cache directory path!"))
        }
    }
    pub fn is_champion_data_old(&self, champion_name: &str, lane: Lane, mode: GameMode) -> bool {
        for champ in &self.json.champions {
            if champ.is_same(champion_name, lane, mode) {
                return champ.is_expired(Duration::days(self.expired));
            }
        }
        info!("No found log, it is old data");
        true
    }

    pub async fn update_champion(
        &mut self,
        champion_name: &str,
        lane: Lane,
        mode: GameMode,
        data: &str,
    ) -> Result<(), String> {
        for champ in &mut self.json.champions {
            if champ.is_same(champion_name, lane, mode) {
                champ.update_last_modified(Utc::now());
                write_to_file(&self.cache_dir.join(champ.get_file_name()), data).await?;
                self.save_cache_log().await?;
                return Ok(());
            }
        }

        // No found it.
        let new = ChampionCacheInfo::new(&champion_name, lane, mode);
        write_to_file(&self.cache_dir.join(&new.get_file_name()), data).await?;
        self.json.champions.push(new);
        self.save_cache_log().await?;

        Ok(())
    }

    pub async fn clear_cache(&mut self) {
        self.json.champions.clear();
        fs::remove_dir_all(&self.cache_dir).unwrap();
        fs::create_dir_all(&self.cache_dir).unwrap();
        self.json.champions.clear();
    }

    pub async fn save_cache_log(&self) -> Result<(), String> {
        let tar = self.cache_dir.join("cache.log");

        write_to_file(&tar, &serde_json::to_string(&self.json).unwrap()).await?;
        Ok(())
    }

    pub async fn get_champion_data(
        &self,
        champion_name: &str,
        lane: Lane,
        mode: GameMode,
    ) -> Result<String, String> {
        let champ_cache_file = self.cache_dir.join(ChampionCacheInfo::get_file_name_manual(
            champion_name,
            lane,
            mode,
        ));
        if champ_cache_file.is_file() {
            let file = File::open(champ_cache_file).await;
            match file {
                Ok(mut file) => {
                    let mut next_data = String::new();
                    file.read_to_string(&mut next_data)
                        .await
                        .map_err(|err| err.to_string())?;
                    return Ok(next_data);
                }
                Err(err) => return Err(err.to_string()),
            }
        }
        else {
            Err(String::from("cache file no exists!"))
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChampionCacheInfo {
    name_with_lane: String,
    last_modified: DateTime<Utc>,
}

impl ChampionCacheInfo {
    pub fn new(name: &str, lane: Lane, mode: GameMode) -> Self {
        ChampionCacheInfo {
            name_with_lane: format!("{}-{:?}-{:?}", name, lane, mode),
            last_modified: Utc::now(),
        }
    }

    pub fn is_same(&self, name: &str, lane: Lane, mode: GameMode) -> bool {
        self.name_with_lane == format!("{}-{:?}-{:?}", name, lane, mode)
    }

    pub fn get_file_name(&self) -> String {
        format!("{}.txt", self.name_with_lane)
    }

    pub fn get_file_name_manual(name: &str, lane: Lane, mode: GameMode) -> String {
        format!("{}-{:?}-{:?}.txt", name, lane, mode)
    }

    pub fn is_expired(&self, duration: Duration) -> bool {
        Utc::now().signed_duration_since(self.last_modified) >= duration
    }

    pub fn update_last_modified(&mut self, new: DateTime<Utc>) {
        self.last_modified = new;
    }
}