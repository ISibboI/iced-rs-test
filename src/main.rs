#![cfg_attr(target_arch = "wasm32", allow(dead_code))]

extern crate core;

use crate::game_state::GameState;
use crate::ui::ApplicationState;
use async_std::path::PathBuf;
use clap::{Args, Parser, Subcommand};
use iced::{Application, Settings};
use log::{info, LevelFilter};
#[cfg(not(target_arch = "wasm32"))]
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode};

mod game_state;
mod game_template;
mod io;
mod ui;
mod utils;

pub const TITLE: &str = "Hero Quest";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    command: Command,

    #[clap(long, default_value = "Info")]
    log_level: LevelFilter,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Run(RunConfiguration),

    #[cfg(not(target_arch = "wasm32"))]
    Compile(crate::game_template::compiler::CompileConfiguration),
}

#[derive(Debug, Clone, Args)]
pub struct RunConfiguration {
    #[clap(long, default_value = "savegame.json")]
    savegame_file: PathBuf,

    #[clap(long, default_value = "data.bin.gz")]
    compiled_game_data_file: PathBuf,

    #[clap(long, default_value = "data.bin.gz")]
    compiled_game_data_url: String,

    #[clap(long, default_value = "static")]
    static_prefix_directory: PathBuf,

    #[clap(long, default_value = "static")]
    static_prefix_url: String,

    #[clap(long, default_value = "60.0")]
    target_fps: f32,

    #[clap(long)]
    profile: bool,
}

fn initialize_logging(log_level: LevelFilter) {
    #[cfg(not(target_arch = "wasm32"))]
    CombinedLogger::init(vec![TermLogger::new(
        log_level,
        ConfigBuilder::default()
            .add_filter_allow_str("iced_rs_test")
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    #[cfg(all(target_arch = "wasm32", debug_assertions))]
    let log_level = log_level.max(LevelFilter::Debug);
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(
        wasm_logger::Config::new(log_level.to_level().unwrap()).module_prefix("iced_rs_test"),
    );

    info!("Logging initialised successfully");
}

fn main() -> Result<(), Error> {
    #[cfg(not(target_arch = "wasm32"))]
    let cli = Cli::parse();
    #[cfg(target_arch = "wasm32")]
    let cli = Cli {
        #[cfg(debug_assertions)]
        log_level: LevelFilter::Debug,
        #[cfg(not(debug_assertions))]
        log_level: LevelFilter::Info,

        command: Command::Run(RunConfiguration::wasm_default()),
    };
    initialize_logging(cli.log_level);

    match cli.command {
        Command::Run(configuration) => {
            let mut settings = Settings::with_flags(configuration);
            settings.exit_on_close_request = false;
            settings.window.resizable = false;
            settings.window.size = (1500, 800);
            ApplicationState::run(settings)?;
        }
        #[cfg(not(target_arch = "wasm32"))]
        Command::Compile(configuration) => {
            async_std::task::Builder::new()
                .name("Game data compiler".to_string())
                .blocking(crate::game_template::compiler::compile(&configuration))?;
        }
    }

    Ok(())
}

impl RunConfiguration {
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn wasm_default() -> Self {
        Self {
            savegame_file: "savegame.json".into(),
            compiled_game_data_file: "".into(),
            compiled_game_data_url: "data.bin.gz".into(),
            static_prefix_directory: "".into(),
            static_prefix_url: "static".into(),
            target_fps: 60.0,
            profile: false,
        }
    }
}

#[derive(Debug)]
enum Error {
    IcedError(iced::Error),
    #[cfg(not(target_arch = "wasm32"))]
    CompilerError(crate::game_template::compiler::CompilerError),
}

impl From<iced::Error> for Error {
    fn from(error: iced::Error) -> Self {
        Self::IcedError(error)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<crate::game_template::compiler::CompilerError> for Error {
    fn from(error: crate::game_template::compiler::CompilerError) -> Self {
        Self::CompilerError(error)
    }
}
