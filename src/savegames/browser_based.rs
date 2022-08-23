use crate::savegames::{LoadError, SaveError};
use crate::GameState;
use web_sys::window;

pub async fn load_game(path: impl AsRef<str>) -> Result<GameState, LoadError> {
    let storage = window()
        .ok_or(LoadError::JsWindowNotFound)?
        .local_storage()?
        .ok_or(LoadError::LocalStorageNotFound)?;
    let savegame = storage
        .get_item(&path.as_ref().to_string())?
        .ok_or(LoadError::SavegameNotFound)?;
    Ok(serde_json::from_str(&savegame)?)
}

pub async fn save_game(game_state: &GameState) -> Result<(), SaveError> {
    let storage = window()
        .ok_or(SaveError::JsWindowNotFound)?
        .local_storage()?
        .ok_or(SaveError::LocalStorageNotFound)?;
    let savegame = serde_json::to_string(game_state)?;
    storage.set_item(&game_state.savegame_file, &savegame)?;
    Ok(())
}
