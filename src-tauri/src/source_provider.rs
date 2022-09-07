use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod cache_manager;
pub mod opgg;

pub type DynChampionInfo = Box<dyn ChampionInfo>;
pub type DynSource = Box<dyn Source>;

#[async_trait]
pub trait Source: Send + Sync {
    fn set_fetch_mode(&mut self, mode: FetchMode);
    async fn set_expired(&self, days: i64);
    async fn get_expired(&self) -> i64;
    fn fetch_mode(&self) -> FetchMode;
    async fn get_champion_info(
        &self,
        champion: &str,
        lane: Lane,
        mode: GameMode,
    ) -> Result<DynChampionInfo, String>;
    async fn clear_cache(&mut self);
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum FetchMode {
    Online,
    Auto,
}
#[async_trait]
pub trait ChampionInfo: Send + Sync {
    fn is_cache(&self) -> bool;
    async fn get_runes(&self) -> Result<Vec<RuneItem>, String>;
    async fn get_spells(&self) -> Result<Vec<SpellItem>, String>;
    fn get_lane(&self) -> Lane;
}

#[derive(Debug, Clone, Copy)]
pub enum Lane {
    Top,
    Mid,
    Bot,
    Support,
    Jungle,
}

#[derive(Debug, Clone, Copy)]
pub enum GameMode {
    Classic,
    Aram,
    Urf,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Build {
    pub runes: Vec<RuneItem>,
    pub spells: Vec<SpellItem>,
    pub is_cache: bool,
    pub is_custom: bool,
    pub lane: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuneItem {
    pub id: i64,
    pub primary_page_id: i32,
    pub primary_rune_ids: Vec<i32>,
    pub secondary_page_id: i32,
    pub secondary_rune_ids: Vec<i32>,
    pub stat_mod_ids: Vec<i32>,
    pub play: i32,
    pub win: i32,
    pub pick_rate: f64,

    #[serde(skip_deserializing)]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRuneItem {
    pub auto_modified_selections: Vec<i32>,
    pub current: bool,
    pub id: i64,
    pub is_active: bool,
    pub is_deletable: bool,
    pub is_editable: bool,
    pub is_valid: bool,
    pub last_modified: i64,
    pub name: String,
    pub order: i32,
    pub primary_style_id: i32,
    pub selected_perk_ids: Vec<i32>,
    pub sub_style_id: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpellItem {
    pub ids: Vec<i32>,
    pub win: i32,
    pub play: i32,
    pub pick_rate: f64,
}

impl Default for FetchMode {
    fn default() -> Self {
        FetchMode::Auto
    }
}

impl ToString for Lane {
    fn to_string(&self) -> String {
        match self {
            Self::Top => "top".to_owned(),
            Self::Mid => "mid".to_owned(),
            Self::Bot => "bot".to_owned(),
            Self::Support => "support".to_owned(),
            Self::Jungle => "jungle".to_owned(),
        }
    }
}

impl Lane {
    pub fn from_str(lane: &str) -> Result<Self, String> {
        match lane.to_lowercase().as_str() {
            "top" => Ok(Self::Top),
            "mid" => Ok(Self::Mid),
            "bot" | "adc" => Ok(Self::Bot),

            "support" | "sp" => Ok(Self::Support),

            "jungle" | "jg" => Ok(Self::Jungle),

            _ => Err(String::from("It not a true lane!")),
        }
    }
}

impl GameMode {
    pub fn from_str(mode: &str) -> Self {
        match mode.to_uppercase().as_str() {
            "ARAM" => Self::Aram,
            "URF" => Self::Urf,
            _ => Self::Classic,
        }
    }
}

impl ToString for GameMode {
    fn to_string(&self) -> String {
        match self {
            Self::Classic => "CLASSIC".to_owned(),
            Self::Aram => "ARAM".to_owned(),
            Self::Urf => "URF".to_owned(),
        }
    }
}

impl From<LolRuneItem> for RuneItem {
    fn from(l: LolRuneItem) -> Self {
        RuneItem {
            id: l.id,
            primary_page_id: l.primary_style_id,
            primary_rune_ids: l.selected_perk_ids[0..4].to_vec(),
            secondary_page_id: l.sub_style_id,
            secondary_rune_ids: l.selected_perk_ids[4..6].to_vec(),
            stat_mod_ids: l.selected_perk_ids[6..9].to_vec(),
            play: 0,
            win: 0,
            pick_rate: 0.0,
            name: l.name
        }
    }
}
