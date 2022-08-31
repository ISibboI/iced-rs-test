use crate::game_state::GameState;
use crate::ui::ApplicationState;
use clap::Parser;
use iced::{Application, Settings};
use log::{error, info, LevelFilter};
#[cfg(not(target_arch = "wasm32"))]
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode};

mod game_state;
mod savegames;
mod ui;
mod utils;

pub const TITLE: &str = "Hero Quest";

#[derive(Parser, Debug)]
pub struct Configuration {
    #[clap(long, default_value = "savegame.json")]
    savegame_file: String,

    #[clap(long, default_value = "Info")]
    log_level: LevelFilter,

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

fn main() {
    let configuration = Configuration::parse();
    initialize_logging(configuration.log_level);

    let mut settings = Settings::with_flags(configuration);
    settings.exit_on_close_request = false;
    settings.window.resizable = false;
    settings.window.size = (1500, 800);
    ApplicationState::run(settings).unwrap_or_else(|err| error!("Error: {err}"));
}
