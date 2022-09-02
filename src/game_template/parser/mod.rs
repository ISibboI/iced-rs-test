use crate::game_template::GameTemplate;
use async_std::io::Read;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum ParserError {
    Io(Arc<std::io::Error>),
}

pub async fn parse_game_template_file(
    game_template: GameTemplate,
    input: impl Read,
) -> Result<GameTemplate, ParserError> {
    todo!()
}

impl From<std::io::Error> for ParserError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(Arc::new(error))
    }
}
