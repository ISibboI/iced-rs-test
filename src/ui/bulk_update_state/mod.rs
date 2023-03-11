use crate::ui::running_state::RunningState;
use crate::ui::{do_nothing, ApplicationUiState, Message};
use crate::{GameState, RunConfiguration};
use async_std::sync::Arc;
use chrono::{DateTime, Duration, Utc};
use iced::alignment::{Horizontal, Vertical};
use iced::{Command, Element, Length};
use iced::widget::Text;
use lazy_static::lazy_static;
use log::{debug, info};

lazy_static! {
    pub static ref BULK_UPDATE_STEP_SIZE: Duration = Duration::hours(1);
}

#[derive(Debug, Clone)]
pub struct BulkUpdateState {
    game_state: Option<GameState>,
    initial_time: DateTime<Utc>,
    update_count: u64,
}

impl BulkUpdateState {
    pub fn new(game_state: GameState) -> Self {
        Self {
            initial_time: game_state.last_update,
            game_state: game_state.into(),
            update_count: 0,
        }
    }

    pub fn update(
        &mut self,
        _configuration: Arc<RunConfiguration>,
        message: BulkUpdateMessage,
    ) -> Command<Message> {
        match message {
            BulkUpdateMessage::Init => Command::perform(
                do_nothing(Box::new(self.game_state.take().unwrap())),
                |game_state| BulkUpdateMessage::Step(game_state).into(),
            ),
            BulkUpdateMessage::Step(game_state) => {
                let current_time = Utc::now();
                let next_delta =
                    (current_time - game_state.last_update).min(*BULK_UPDATE_STEP_SIZE);
                self.update_count += 1;

                let total_steps: u64 = ((current_time - self.initial_time).num_milliseconds()
                    as f64
                    / BULK_UPDATE_STEP_SIZE.num_milliseconds() as f64)
                    .ceil() as u64;

                debug!("Bulk updating {}/{total_steps}", self.update_count);

                if next_delta == *BULK_UPDATE_STEP_SIZE {
                    Command::perform(
                        update(game_state, next_delta.num_milliseconds()),
                        |game_state| BulkUpdateMessage::Step(game_state).into(),
                    )
                } else {
                    Command::perform(
                        update(game_state, next_delta.num_milliseconds()),
                        |game_state| BulkUpdateMessage::Finished(game_state).into(),
                    )
                }
            }
            BulkUpdateMessage::Finished(game_state) => {
                info!("Finished bulk update");
                Command::perform(do_nothing(game_state), |game_state| {
                    Message::ChangeState(Box::new(ApplicationUiState::Running(Box::new(
                        RunningState::new(*game_state),
                    ))))
                })
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let current_time = Utc::now();
        let total_steps: u64 = ((current_time - self.initial_time).num_milliseconds() as f64
            / BULK_UPDATE_STEP_SIZE.num_milliseconds() as f64)
            .ceil() as u64;

        Text::new(&format!(
            "Evaluating offline progress... ({}/{total_steps})",
            self.update_count
        ))
        .size(100)
        .horizontal_alignment(Horizontal::Center)
        .vertical_alignment(Vertical::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

async fn update(mut game_state: Box<GameState>, delta_milliseconds: i64) -> Box<GameState> {
    game_state.update(delta_milliseconds);
    game_state
}

#[derive(Clone, Debug)]
pub enum BulkUpdateMessage {
    Init,
    Step(Box<GameState>),
    Finished(Box<GameState>),
}
