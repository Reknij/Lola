use std::path::PathBuf;

use directories::ProjectDirs;
use tokio::{
    fs::{self, File},
    io::AsyncReadExt,
};
use tracing::error;

use crate::{
    source_provider::{GameMode, LolRuneItem, RuneItem},
    util::write_to_file,
};

#[derive(Debug)]
pub struct CustomProvider {
    custom_builds: PathBuf,
}

impl CustomProvider {
    pub fn initialize() -> Self {
        if let Some(proj) = ProjectDirs::from("com", "Jinte", "Lola") {
            let custom_builds = proj.data_dir().join("custom_builds");
            std::fs::create_dir_all(&custom_builds).unwrap();
            return CustomProvider { custom_builds };
        }
        panic!("Can't initialize!")
    }

    pub async fn get_champion_lol_runes(
        &self,
        champion_name: &str,
        mode: GameMode,
    ) -> Vec<LolRuneItem> {
        let build_file = self
            .custom_builds
            .join(&CustomProvider::get_file_name(champion_name, mode));
        if build_file.is_file() {
            let file = File::open(build_file).await;
            match file {
                Ok(mut file) => {
                    let mut data = String::new();
                    file.read_to_string(&mut data)
                        .await
                        .map_err(|err| err.to_string())
                        .unwrap();
                    return serde_json::from_str(&data).unwrap();
                }
                Err(err) => {
                    error!("{:?}", err);
                    return Vec::new();
                }
            }
        } else {
            return Vec::new();
        }
    }

    pub async fn get_champion_runes(&self, champion_name: &str, mode: GameMode) -> Vec<RuneItem> {
        let lol = self.get_champion_lol_runes(champion_name, mode).await;
        let mut runes = Vec::with_capacity(lol.len());

        for l in lol {
            runes.push(RuneItem::from(l))
        }

        return runes;
    }

    pub async fn add_champion_rune(
        &self,
        champion_name: &str,
        mode: GameMode,
        rune_item: LolRuneItem,
    ) -> bool {
        let mut runes = self.get_champion_lol_runes(champion_name, mode).await;
        for r in &runes {
            if r.name == rune_item.name {
                return false;
            }
        }
        runes.push(rune_item);
        let build_file = self
            .custom_builds
            .join(&CustomProvider::get_file_name(champion_name, mode));
        write_to_file(&build_file, &serde_json::to_string(&runes).unwrap())
            .await
            .unwrap();
        return true;
    }

    pub async fn remove_champion_rune(&self, champion_name: &str, mode: GameMode, rune_name: &str) -> bool {
        let mut runes = self.get_champion_lol_runes(champion_name, mode).await;
        for (i, rune) in runes.iter().enumerate() {
            if rune.name == rune_name {
                runes.remove(i);
                let build_file = self
                    .custom_builds
                    .join(&CustomProvider::get_file_name(champion_name, mode));
                write_to_file(&build_file, &serde_json::to_string(&runes).unwrap())
                    .await
                    .unwrap();
                return true;
            }
        }
        return false;
    }

    pub async fn remove_champion_runes(&self, champion_name: &str, mode: GameMode) -> bool {
        let build_file = self
            .custom_builds
            .join(&CustomProvider::get_file_name(champion_name, mode));
        if build_file.is_file() {
            fs::remove_file(build_file).await.unwrap();
        }
        return true;
    }

    pub fn get_file_name(champion_name: &str, mode: GameMode) -> String {
        format!("{}-{:?}-runes.json", champion_name, mode)
    }
}
