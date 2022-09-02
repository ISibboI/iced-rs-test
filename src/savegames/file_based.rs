use crate::savegames::{LoadError, SaveError};
use crate::GameState;
use async_std::fs::File;
use async_std::io::{BufReader, BufWriter, ReadExt, WriteExt};
use async_std::path::Path;

pub async fn load_game(path: impl AsRef<Path>) -> Result<GameState, LoadError> {
    let path = path.as_ref();
    let savegame_file = File::open(path).await?;
    let mut savegame = String::new();
    BufReader::new(savegame_file)
        .read_to_string(&mut savegame)
        .await?;
    Ok(serde_json::from_str(&savegame)?)
}

pub async fn save_game(game_state: &GameState) -> Result<(), SaveError> {
    let path = &game_state.savegame_file.as_ref();
    let savegame_file = File::create(path).await?;
    let savegame = serde_json::to_vec(game_state)?;
    let mut writer = BufWriter::new(savegame_file);
    writer.write_all(&savegame).await?;
    writer.flush().await?;
    Ok(())
}
