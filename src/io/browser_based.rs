use crate::game_template::CompiledGameTemplate;
use crate::io::{LoadError, SaveError};
use crate::{GameState, RunConfiguration};
use async_std::path::Path;
use async_std::sync::Arc;
use flate2::bufread::GzDecoder;
use log::info;
use reqwest::Url;
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
    configuration: Arc<RunConfiguration>,
) -> Result<CompiledGameTemplate, LoadError> {
    let base_url = Url::parse(
        &window()
            .ok_or(LoadError::JsWindowNotFound)?
            .location()
            .href()
            .map_err(|error| LoadError::JsError(format!("{error:?}")))?,
    )?;
    let url = base_url.join(&configuration.compiled_game_data_url)?;
    info!("Loading {:?}", url);
    let body = reqwest::get(url).await?.bytes().await?;
    let decoder = GzDecoder::new(&body[..]);
    Ok(pot::from_reader(decoder)?)
}

pub async fn load_bytes(
    configuration: Arc<RunConfiguration>,
    static_file: String,
) -> Result<Vec<u8>, LoadError> {
    let base_url = Url::parse(
        &window()
            .ok_or(LoadError::JsWindowNotFound)?
            .location()
            .href()
            .map_err(|error| LoadError::JsError(format!("{error:?}")))?,
    )?;
    let url = base_url.join(&configuration.static_prefix_url)?;
    let url = url.join(&static_file)?;
    info!("Loading {:?}", url);
    let body = reqwest::get(url).await?.bytes().await?;
    Ok(body.into())
}
