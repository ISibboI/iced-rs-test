use crate::ui::running_state::RunningState;
use crate::ui::{do_nothing, ApplicationUiState, Message};
use crate::{Configuration, GameState};
use iced::alignment::{Horizontal, Vertical};
use iced::{Command, Element, Length, Text};
use log::info;
use std::time::{Duration, Instant};

pub static BULK_UPDATE_STEP_SIZE: Duration = Duration::from_secs(3600 * 24);

#[derive(Debug, Clone)]
pub struct BulkUpdateState {
    game_state: GameState,
    initial_time: Instant,
    update_count: u64,
}

impl BulkUpdateState {
    pub fn new(game_state: GameState) -> Self {
        Self {
            initial_time: game_state.last_update.into(),
            game_state,
            update_count: 0,
        }
    }

    pub fn update(
        &mut self,
        _configuration: &Configuration,
        message: BulkUpdateMessage,
    ) -> Command<Message> {
        match message {
            BulkUpdateMessage::Init => {
                Command::perform(do_nothing(()), |()| BulkUpdateMessage::Step.into())
            }
            BulkUpdateMessage::Step => {
                let current_time = Instant::now();
                let next_delta =
                    (current_time - self.game_state.last_update.time).min(BULK_UPDATE_STEP_SIZE);
                self.game_state.update(next_delta.as_secs_f64());

                if next_delta == BULK_UPDATE_STEP_SIZE {
                    Command::perform(do_nothing(()), |()| BulkUpdateMessage::Step.into())
                } else {
                    info!("Finished bulk update");
                    Command::perform(
                        do_nothing(Box::new(RunningState::new(self.game_state.clone()))),
                        |running_state| {
                            Message::ChangeState(Box::new(ApplicationUiState::Running(
                                running_state,
                            )))
                        },
                    )
                }
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let current_time = Instant::now();
        let total_steps: u64 = ((current_time - self.initial_time).as_secs_f64()
            / BULK_UPDATE_STEP_SIZE.as_secs_f64())
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

#[derive(Clone, Debug)]
pub enum BulkUpdateMessage {
    Init,
    Step,
}
