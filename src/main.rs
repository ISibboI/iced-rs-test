use crate::game_state::GameState;
use crate::ui::ApplicationState;
use clap::Parser;
use iced::{Application, Settings};
use log::{error, info, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};

mod game_state;
mod ui;

#[derive(Parser, Debug)]
pub struct Configuration {
    #[clap(long, default_value = "savegame.json")]
    savegame_file: String,

    #[clap(long, default_value = "Info")]
    log_level: LevelFilter,
}

fn initialize_logging(log_level: LevelFilter) {
    CombinedLogger::init(vec![TermLogger::new(
        log_level,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    info!("Logging initialised successfully");
}

fn main() {
    let configuration = Configuration::parse();
    initialize_logging(configuration.log_level);

    let mut settings = Settings::with_flags(configuration);
    settings.exit_on_close_request = false;
    ApplicationState::run(settings).unwrap_or_else(|err| error!("Error: {err}"));
}
