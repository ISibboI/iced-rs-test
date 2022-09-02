use std::sync::Arc;
use wasm_bindgen::JsValue;

#[cfg(not(target_arch = "wasm32"))]
mod file_based;
#[cfg(not(target_arch = "wasm32"))]
pub use file_based::{load_game, save_game};

#[cfg(target_arch = "wasm32")]
mod browser_based;
use crate::GameState;
#[cfg(target_arch = "wasm32")]
pub use browser_based::{load_game, save_game};

pub mod pathbuf_serde;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum LoadError {
    IoError(Arc<std::io::Error>),
    JsonError(Arc<serde_json::Error>),
    JsError(String),
    JsWindowNotFound,
    LocalStorageNotFound,
    SavegameNotFound,
}

impl From<std::io::Error> for LoadError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(Arc::new(error))
    }
}

impl From<serde_json::Error> for LoadError {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonError(Arc::new(error))
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
            LoadError::JsonError(error) => format!("Parsing error: {error}"),
            LoadError::JsError(error) => format!("Javascript error: {error:?}"),
            LoadError::JsWindowNotFound => {
                "The browser does not provide a window object".to_string()
            }
            LoadError::LocalStorageNotFound => {
                "The browser does not provide local storage".to_string()
            }
            LoadError::SavegameNotFound => "Could not find savegame".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum SaveError {
    IoError(Arc<std::io::Error>),
    JsonError(Arc<serde_json::Error>),
    JsError(String),
    JsWindowNotFound,
    LocalStorageNotFound,
}

impl From<std::io::Error> for SaveError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(Arc::new(error))
    }
}

impl From<serde_json::Error> for SaveError {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonError(Arc::new(error))
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
            SaveError::JsonError(error) => format!("Serialization error: {}", error),
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
