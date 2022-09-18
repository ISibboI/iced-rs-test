use crate::game_template::parser::error::ParserError;
use crate::game_template::parser::parse_game_template_file;
use crate::game_template::GameTemplate;
use async_recursion::async_recursion;
use async_std::fs::File;
use async_std::io::{BufReader, WriteExt};
use async_std::path::{Path, PathBuf};
use async_std::stream::StreamExt;
use clap::Args;
use flate2::write::GzEncoder;
use flate2::Compression;
use log::{debug, info, warn};
use std::ffi::OsStr;
use std::io::Write;

#[derive(Debug)]
pub enum CompilerError {
    Parser(ParserError),
    Pot(pot::Error),
    Io(std::io::Error),
}

#[derive(Debug, Args)]
pub struct CompileConfiguration {
    #[clap(long, default_value = "data")]
    source_game_data: PathBuf,

    #[clap(long, default_value = "data.bin")]
    compiled_game_data: PathBuf,
}

pub async fn compile(configuration: &CompileConfiguration) -> Result<(), CompilerError> {
    let mut game_template = GameTemplate::default();
    compile_directory(&mut game_template, &configuration.source_game_data).await?;
    info!("Compiling...");
    let game_template = game_template.compile()?;
    info!("Serialising...");
    let game_template_vec = pot::to_vec(&game_template)?;
    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(&game_template_vec)?;
    let game_template_vec = encoder.finish()?;

    if configuration.compiled_game_data.exists().await {
        info!(
            "Overwriting {}",
            configuration.compiled_game_data.to_string_lossy()
        );
    }

    info!("Writing...");
    let mut compiled_game_data = File::create(&configuration.compiled_game_data).await?;
    compiled_game_data.write_all(&game_template_vec).await?;
    Ok(())
}

#[async_recursion]
async fn compile_directory(
    game_template: &mut GameTemplate,
    directory: &Path,
) -> Result<(), CompilerError> {
    let mut read_dir = directory.read_dir().await?;
    while let Some(entry) = read_dir.next().await {
        let entry = entry?;
        let path = entry.path();
        if path.is_file().await {
            if path.extension().and_then(OsStr::to_str) == Some("tpl") {
                info!("Parsing {}", path.to_string_lossy());
                parse_game_template_file(game_template, BufReader::new(File::open(path).await?))
                    .await?;
            } else {
                debug!("Skipping {}", path.to_string_lossy());
            }
        } else if path.is_dir().await {
            compile_directory(game_template, &path).await?;
        } else {
            warn!(
                "Found directory entry that is neither a file nor a directory: {:?}",
                path
            );
        }
    }

    Ok(())
}

impl From<ParserError> for CompilerError {
    fn from(error: ParserError) -> Self {
        Self::Parser(error)
    }
}

impl From<pot::Error> for CompilerError {
    fn from(error: pot::Error) -> Self {
        Self::Pot(error)
    }
}

impl From<std::io::Error> for CompilerError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
