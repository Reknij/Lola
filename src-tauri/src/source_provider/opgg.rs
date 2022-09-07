use async_trait::async_trait;
use futures_util::lock::Mutex;
use jsonpath_rust::JsonPathFinder;
use reqwest::Client;
use scraper::{Html, Selector};
use tracing::info;

use super::{
    cache_manager::CacheManager, ChampionInfo, DynChampionInfo, FetchMode, GameMode, Lane,
    RuneItem, Source, SpellItem,
};

pub struct OPGG {
    fetch_mode: FetchMode,
    http_client: Client,
    cache_manager: Mutex<CacheManager>,
}

#[derive(Debug, Clone)]
pub struct OPGGChampionInfo {
    next_data: String,
    is_cache: bool,
    lane: Lane,
    champion_name: String,
}

#[async_trait]
impl ChampionInfo for OPGGChampionInfo {
    //remember add new function go modify format_data()
    fn is_cache(&self) -> bool {
        self.is_cache
    }

    fn get_lane(&self) -> Lane {
        self.lane
    }

    async fn get_runes(&self) -> Result<Vec<RuneItem>, String> {
        let runes_finder =
            JsonPathFinder::from_str(&self.next_data, "$.props.pageProps.data.runes[0:]")
                .expect("Create runes finder failed!");
        let rune_items = runes_finder.find();
        let mut runes: Vec<RuneItem> = serde_json::from_value(rune_items).unwrap();
        for mut r in &mut runes {
            r.name = format!(
                "[Lola] {}-{:?} ({:.2}%/{} games)",
                self.champion_name,
                self.lane,
                r.win * 100 / r.play,
                r.play
            )
        }
        info!("runes len is {}", runes.len());
        Ok(runes)
    }

    async fn get_spells(&self) -> Result<Vec<SpellItem>, String> {
        let spells_finder = JsonPathFinder::from_str(
            &self.next_data,
            "$.props.pageProps.data.summoner_spells[0:]",
        )
        .expect("Create spells finder failed!");
        let spell_items = spells_finder.find();
        let spells: Vec<SpellItem> = serde_json::from_value(spell_items).unwrap();
        info!("spells len is {}", spells.len());
        Ok(spells)
    }
}

#[async_trait]
impl Source for OPGG {
    fn set_fetch_mode(&mut self, mode: FetchMode) {
        self.fetch_mode = mode;
    }

    fn fetch_mode(&self) -> FetchMode {
        self.fetch_mode
    }

    async fn set_expired(&self, days: i64) {
        self.cache_manager.lock().await.set_expired(days)
    }

    async fn get_expired(&self) -> i64 {
        self.cache_manager.lock().await.get_expired()
    }

    async fn clear_cache(&mut self) {
        self.cache_manager.lock().await.clear_cache().await;
    }

    async fn get_champion_info(
        &self,
        champion_name: &str,
        lane: Lane,
        mode: GameMode,
    ) -> Result<DynChampionInfo, String> {
        match self.fetch_mode {
            FetchMode::Online => Ok(Box::new(OPGGChampionInfo {
                is_cache: false,
                next_data: self
                    .get_champion_data_online(champion_name, lane, mode)
                    .await?,
                lane,
                champion_name: champion_name.to_owned(),
            })),
            FetchMode::Auto => {
                if self
                    .cache_manager
                    .lock()
                    .await
                    .is_champion_data_old(champion_name, lane, mode)
                {
                    let mut next_data = self
                        .get_champion_data_online(champion_name, lane, mode)
                        .await?;
                    let next_data = self.format_data(&mut next_data);
                    self.cache_manager
                        .lock()
                        .await
                        .update_champion(champion_name, lane, mode, &next_data)
                        .await?;
                    Ok(Box::new(OPGGChampionInfo {
                        is_cache: false,
                        next_data,
                        lane,
                        champion_name: champion_name.to_owned(),
                    }))
                } else {
                    let next_data = self
                        .cache_manager
                        .lock()
                        .await
                        .get_champion_data(champion_name, lane, mode)
                        .await?;
                    Ok(Box::new(OPGGChampionInfo {
                        is_cache: true,
                        next_data,
                        lane,
                        champion_name: champion_name.to_owned(),
                    }))
                }
            }
        }
    }
}

impl OPGG {
    pub async fn new() -> Self {
        OPGG {
            fetch_mode: FetchMode::Auto,
            http_client: reqwest::Client::builder().build().unwrap(),
            cache_manager: Mutex::new(CacheManager::initialize().await.unwrap()),
        }
    }

    async fn get_html(&self, url: &str) -> Result<String, String> {
        self.http_client
            .get(url)
            .send()
            .await
            .map_err(|e| format!("Get request error: {}", e))?
            .text()
            .await
            .map_err(|e| format!("Get html error: {}", e))
    }

    fn format_data(&self, data: &str) -> String {
        let rune_items = JsonPathFinder::from_str(data, "$.props.pageProps.data.runes[0:]")
            .expect("Create runes finder failed!")
            .find();
        let spell_items =
            JsonPathFinder::from_str(data, "$.props.pageProps.data.summoner_spells[0:]")
                .expect("Create spells finder failed!")
                .find();
        let cache = format!(
            r#"{{
                "props":{{
                    "pageProps":{{
                        "data":{{
                            "runes": {},
                            "summoner_spells": {}
                        }}
                    }}
                }}
            }}"#,
            serde_json::to_string(&rune_items).unwrap(),
            serde_json::to_string(&spell_items).unwrap()
        );
        cache
    }

    async fn get_champion_data_online(
        &self,
        champion_name: &str,
        lane: Lane,
        mode: GameMode,
    ) -> Result<String, String> {
        let champion_name: String = champion_name
            .chars()
            .filter(|c| !is_illegal_for_opgg(c))
            .collect(); //remove illegal characters.
        let url = match mode {
            GameMode::Aram | GameMode::Urf => format!(
                "https://www.op.gg/{}/{}/build",
                mode_as_opgg(mode),
                champion_name
            ),
            GameMode::Classic => format!(
                "https://www.op.gg/{}/{}/{}/build",
                mode_as_opgg(mode),
                champion_name,
                lane.to_string()
            ),
        };
        info!("url is {}", &url);
        let opgg_html = self.get_html(&url).await.expect("Get html failed!");
        // get runes
        let document = Html::parse_document(&opgg_html);
        let next_data = document
            .select(&Selector::parse("script#__NEXT_DATA__").unwrap())
            .next()
            .expect("No found 'script#__NEXT_DATA__' element!")
            .inner_html();
        Ok(next_data)
    }
}

fn is_illegal_for_opgg(c: &char) -> bool {
    c == &'\'' || c.is_whitespace()
}

fn mode_as_opgg(mode: GameMode) -> &'static str {
    match mode {
        GameMode::Classic => "champions",
        GameMode::Aram => "modes/aram",
        GameMode::Urf => "modes/urf",
    }
}
