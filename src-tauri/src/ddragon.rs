use std::collections::HashMap;

use jsonpath_rust::JsonPathFinder;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct DDragon {
    champions: HashMap<String, Champion>,
    version: String,

    #[allow(dead_code)]
    language: Language, //prepare for future
}

impl DDragon {
    pub async fn get_latest_version() -> Result<String, String> {
        let versions = Self::get_versions().await;
        match versions {
            Ok(versions) => {
                if let Some(first) = versions.first() {
                    Ok(first.clone())
                } else {
                    Err(String::from("get versions array but it length is zero."))
                }
            }
            Err(err) => Err(err),
        }
    }

    pub async fn get_versions() -> Result<Vec<String>, String> {
        let resp = reqwest::get("https://ddragon.leagueoflegends.com/api/versions.json").await;
        match resp {
            Ok(resp) => {
                let versions: Vec<String> = resp.json().await.expect("Convert json error!");
                Ok(versions)
            }

            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn get_champions(
        version: &str,
        language: Language,
    ) -> Result<HashMap<String, Champion>, String> {
        let resp = reqwest::get(format!(
            "http://ddragon.leagueoflegends.com/cdn/{}/data/{}/champion.json",
            version,
            language.to_string()
        ))
        .await
        .expect("Get champion.json error!");
        let json = resp.text().await.expect("Get json text error!");
        let name_finder = JsonPathFinder::from_str(&json, "$.data..name")?;
        let id_finder = JsonPathFinder::from_str(&json, "$.data..id")?;
        let key_finder = JsonPathFinder::from_str(&json, "$.data..key")?;

        let names = name_finder.find_slice();
        let ids = id_finder.find_slice();
        let keys = key_finder.find_slice();
        if !(names.len() == ids.len() && ids.len() == keys.len()) {
            return Err(String::from(
                "Champions name, id and key length is no same!",
            )); //error because they no same.
        }

        let mut champions = HashMap::with_capacity(names.len());
        for i in 0..names.len() {
            champions.insert(
                keys[i].as_str().unwrap().to_owned(),
                Champion {
                    name: names[i].as_str().unwrap().to_owned(),
                    id: ids[i].as_str().unwrap().to_owned(),
                    key: keys[i].as_str().unwrap().to_owned(),
                },
            );
        }

        Ok(champions)
    }

    pub async fn new(language: Language) -> Self {
        let version = DDragon::get_latest_version()
            .await
            .expect("Get latest lol client version failed!");
        DDragon {
            champions: DDragon::get_champions(&version, language)
                .await
                .expect("Get lol champions data failed!"),
            version,
            language,
        }
    }

    pub fn get_champion_information(&self, key: &str) -> Result<Champion, String> {
        if let Some(c) = self.champions.get(key) {
            return Ok(c.clone());
        }

        Err(String::from(format!("Key value {} no match with ddragon!", key)))
    }

    pub fn get_version(&self) -> String {
        self.version.to_owned()
    }
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Self::en_US => "en_US".to_owned(),
            Self::cs_CZ => "cs_CZ".to_owned(),
            Self::de_DE => "de_DE".to_owned(),
            Self::el_GR => "el_GR".to_owned(),
            Self::en_AU => "en_AU".to_owned(),
            Self::en_GB => "en_GB".to_owned(),
            Self::en_PH => "en_PH".to_owned(),
            Self::en_SG => "en_SG".to_owned(),
            Self::es_AR => "es_AR".to_owned(),
            Self::es_ES => "es_ES".to_owned(),
            Self::es_MX => "es_MX".to_owned(),
            Self::fr_FR => "fr_FR".to_owned(),
            Self::hu_HU => "hu_HU".to_owned(),
            Self::id_ID => "id_ID".to_owned(),
            Self::it_IT => "it_IT".to_owned(),
            Self::ja_JP => "ja_JP".to_owned(),
            Self::ko_KR => "ko_KR".to_owned(),
            Self::pl_PL => "pl_PL".to_owned(),
            Self::pt_BR => "pt_BR".to_owned(),
            Self::ro_RO => "ro_RO".to_owned(),
            Self::ru_RU => "ru_RU".to_owned(),
            Self::th_TH => "th_TH".to_owned(),
            Self::tr_TR => "tr_TR".to_owned(),
            Self::vn_VN => "vn_VN".to_owned(),
            Self::zh_CN => "zh_CN".to_owned(),
            Self::zh_MY => "zh_MY".to_owned(),
            Self::zh_TW => "zh_TW".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Champion {
    pub id: String,
    pub key: String,
    pub name: String,
}

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Language {
    en_US,
    cs_CZ,
    de_DE,
    el_GR,
    en_AU,
    en_GB,
    en_PH,
    en_SG,
    es_AR,
    es_ES,
    es_MX,
    fr_FR,
    hu_HU,
    id_ID,
    it_IT,
    ja_JP,
    ko_KR,
    pl_PL,
    pt_BR,
    ro_RO,
    ru_RU,
    th_TH,
    tr_TR,
    vn_VN,
    zh_CN,
    zh_MY,
    zh_TW,
}
