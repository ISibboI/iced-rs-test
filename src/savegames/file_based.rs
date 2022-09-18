use crate::game_template::CompiledGameTemplate;
use crate::savegames::{LoadError, SaveError};
use crate::{GameState, RunConfiguration};
use async_std::fs::File;
use async_std::io::{BufReader, BufWriter, ReadExt, WriteExt};
use async_std::path::Path;
use log::info;

pub async fn load_game(path: impl AsRef<Path>) -> Result<GameState, LoadError> {
    let path = path.as_ref();
    let savegame_file = File::open(path).await?;
    let mut savegame = Vec::new();
    BufReader::new(savegame_file)
        .read_to_end(&mut savegame)
        .await?;
    Ok(pot::from_slice(&savegame)?)
}

pub async fn save_game(game_state: &GameState) -> Result<(), SaveError> {
    let path = &game_state.savegame_file.as_ref();
    let savegame_file = File::create(path).await?;
    let savegame = pot::to_vec(game_state)?;
    let mut writer = BufWriter::new(savegame_file);
    writer.write_all(&savegame).await?;
    writer.flush().await?;
    Ok(())
}

pub async fn load_game_template(
    configuration: RunConfiguration,
) -> Result<CompiledGameTemplate, LoadError> {
    info!("Loading {:?}", &configuration.compiled_game_data_file);
    let savegame_file = File::open(&configuration.compiled_game_data_file).await?;
    let mut savegame = Vec::new();
    BufReader::new(savegame_file)
        .read_to_end(&mut savegame)
        .await?;
    Ok(pot::from_slice(&savegame)?)
}
