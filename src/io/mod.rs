use std::sync::Arc;
use wasm_bindgen::JsValue;

#[cfg(not(target_arch = "wasm32"))]
mod file_based;
#[cfg(not(target_arch = "wasm32"))]
pub use file_based::{load_bytes, load_game, load_game_template, save_game};

#[cfg(target_arch = "wasm32")]
mod browser_based;
use crate::GameState;
#[cfg(target_arch = "wasm32")]
pub use browser_based::{load_bytes, load_game, load_game_template, save_game};

pub mod pathbuf_serde;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum LoadError {
    IoError(Arc<std::io::Error>),
    PotError(Arc<pot::Error>),
    Base64Error(Arc<base64::DecodeError>),
    ReqwestError(Arc<reqwest::Error>),
    UrlParseError(Arc<url::ParseError>),
    JsError(String),
    JsWindowNotFound,
    LocalStorageNotFound,
    SavegameNotFound,
    LocationNotFound,
}

impl From<std::io::Error> for LoadError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(Arc::new(error))
    }
}

impl From<pot::Error> for LoadError {
    fn from(error: pot::Error) -> Self {
        Self::PotError(Arc::new(error))
    }
}

impl From<base64::DecodeError> for LoadError {
    fn from(error: base64::DecodeError) -> Self {
        Self::Base64Error(Arc::new(error))
    }
}

impl From<reqwest::Error> for LoadError {
    fn from(error: reqwest::Error) -> Self {
        Self::ReqwestError(Arc::new(error))
    }
}

impl From<url::ParseError> for LoadError {
    fn from(error: url::ParseError) -> Self {
        Self::UrlParseError(Arc::new(error))
    }
}

impl From<JsValue> for LoadError {
    fn from(error: JsValue) -> Self {
        Self::JsError(format!("{error:?}"))
    }
}

impl ToString for LoadError {
    fn to_string(&self) -> String {
        match self {
            LoadError::IoError(error) => format!("IO error: {error}"),
            LoadError::PotError(error) => format!("Parsing error: {error}"),
            LoadError::Base64Error(error) => format!("Parsing error: {error}"),
            LoadError::ReqwestError(error) => format!("HTTP request error: {error}"),
            LoadError::UrlParseError(error) => format!("URL parse error: {error}"),
            LoadError::JsError(error) => format!("Javascript error: {error:?}"),
            LoadError::JsWindowNotFound => {
                "The browser does not provide a window object".to_string()
            }
            LoadError::LocalStorageNotFound => {
                "The browser does not provide local storage".to_string()
            }
            LoadError::SavegameNotFound => "Could not find savegame".to_string(),
            LoadError::LocationNotFound => {
                "The browser does not support the window.location interface".to_string()
            }
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum SaveError {
    IoError(Arc<std::io::Error>),
    PotError(Arc<pot::Error>),
    JsError(String),
    JsWindowNotFound,
    LocalStorageNotFound,
}

impl From<std::io::Error> for SaveError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(Arc::new(error))
    }
}

impl From<pot::Error> for SaveError {
    fn from(error: pot::Error) -> Self {
        Self::PotError(Arc::new(error))
    }
}

impl From<JsValue> for SaveError {
    fn from(error: JsValue) -> Self {
        Self::JsError(format!("{error:?}"))
    }
}

impl ToString for SaveError {
    fn to_string(&self) -> String {
        match self {
            SaveError::IoError(error) => format!("IO error: {}", error),
            SaveError::PotError(error) => format!("Serialization error: {}", error),
            SaveError::JsError(error) => format!("Javascript error: {error:?}"),
            SaveError::JsWindowNotFound => {
                "The browser does not provide a window object".to_string()
            }
            SaveError::LocalStorageNotFound => {
                "The browser does not provide local storage".to_string()
            }
        }
    }
}

pub async fn save_game_owned(game_state: GameState) -> Result<(), SaveError> {
    save_game(&game_state).await
}
