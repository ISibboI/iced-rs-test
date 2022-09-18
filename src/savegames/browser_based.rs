use crate::game_template::CompiledGameTemplate;
use crate::savegames::{LoadError, SaveError};
use crate::{GameState, RunConfiguration};
use async_std::path::Path;
use flate2::bufread::GzDecoder;
use log::info;
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

pub async fn load_game_template(
    configuration: RunConfiguration,
) -> Result<CompiledGameTemplate, LoadError> {
    info!("Loading {:?}", &configuration.compiled_game_data_url);
    let body = reqwest::get(configuration.compiled_game_data_url)
        .await?
        .bytes()
        .await?;
    let decoder = GzDecoder::new(&body[..]);
    Ok(pot::from_reader(decoder)?)
}
