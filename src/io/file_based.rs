use crate::game_template::CompiledGameTemplate;
use crate::io::{LoadError, SaveError};
use crate::{GameState, RunConfiguration};
use async_std::fs::File;
use async_std::io::{BufReader, BufWriter, ReadExt, WriteExt};
use async_std::path::Path;
use async_std::sync::Arc;
use flate2::bufread::GzDecoder;
use log::{debug, info};

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
    configuration: Arc<RunConfiguration>,
) -> Result<CompiledGameTemplate, LoadError> {
    info!("Loading {:?}", &configuration.compiled_game_data_file);
    let savegame_file = File::open(&configuration.compiled_game_data_file).await?;
    let mut compressed_savegame = Vec::new();
    BufReader::new(savegame_file)
        .read_to_end(&mut compressed_savegame)
        .await?;
    let decoder = GzDecoder::new(compressed_savegame.as_slice());
    Ok(pot::from_reader(decoder)?)
}

pub async fn load_bytes(
    configuration: Arc<RunConfiguration>,
    static_file: String,
) -> Result<Vec<u8>, LoadError> {
    let mut file = configuration.static_prefix_directory.clone();
    file.push(static_file);
    debug!("Loading {:?}", file);
    let mut static_file = File::open(&file).await?;
    let mut bytes = Vec::new();
    static_file.read_to_end(&mut bytes).await?;
    Ok(bytes)
}
