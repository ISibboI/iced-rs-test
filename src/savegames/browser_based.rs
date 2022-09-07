use crate::savegames::{LoadError, SaveError};
use crate::GameState;
use async_std::path::Path;
use web_sys::window;

pub async fn load_game(path: impl AsRef<Path>) -> Result<GameState, LoadError> {
    let storage = window()
        .ok_or(LoadError::JsWindowNotFound)?
        .local_storage()?
        .ok_or(LoadError::LocalStorageNotFound)?;
    let savegame = storage
        .get_item(&path.as_ref().to_string_lossy())?
        .ok_or(LoadError::SavegameNotFound)?;
    Ok(pot::from_slice(&base64::decode(&savegame)?)?)
}

pub async fn save_game(game_state: &GameState) -> Result<(), SaveError> {
    let storage = window()
        .ok_or(SaveError::JsWindowNotFound)?
        .local_storage()?
        .ok_or(SaveError::LocalStorageNotFound)?;
    let savegame = base64::encode(pot::to_vec(game_state)?);
    storage.set_item(
        &game_state.savegame_file.as_ref().to_string_lossy(),
        &savegame,
    )?;
    Ok(())
}
