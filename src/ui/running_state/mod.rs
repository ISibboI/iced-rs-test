use crate::game_state::character::CombatStyle;
use crate::game_state::player_actions::{PlayerActionId, ACTION_EXPLORE};
use crate::game_state::time::GameTime;
use crate::game_state::world::locations::LocationId;
use crate::io::{save_game_owned, SaveError};
use crate::ui::elements::{attribute, clock_time, currency, date, title};
use crate::ui::running_state::main_view::{MainViewMessage, MainViewState};
use crate::ui::{do_nothing, Message};
use crate::{GameState, RunConfiguration};
use async_std::sync::Arc;
use chrono::{DateTime, Duration, Utc};
use iced::alignment::Horizontal;
use iced::{Alignment, Command, Element, Length,};
use iced::widget::{Column, Row, Space, Text, ProgressBar};
use lazy_static::lazy_static;
use log::{error, info, trace, warn};
use std::collections::VecDeque;

mod main_view;

lazy_static! {
    pub static ref AUTOSAVE_INTERVAL: Duration = Duration::seconds(10);
}

#[derive(Debug, Clone)]
pub struct RunningState {
    game_state: GameState,
    frame_times: VecDeque<DateTime<Utc>>,
    fps: Option<f32>,
    last_save: DateTime<Utc>,
    main_view_state: MainViewState,
    last_view_duration: Duration,
}

#[derive(Clone, Debug)]
pub enum RunningMessage {
    Init,
    Update,
    GameSaved(Result<(), SaveError>),
    SaveAndQuit,

    GameState(GameStateMessage),
    MainView(MainViewMessage),
}

#[derive(Clone, Debug)]
pub enum GameStateMessage {
    ActionChanged(PlayerActionId),
    ActionChangedExplore(LocationId),
    ExplorationLocationChanged(LocationId),
    CombatStyleChanged(CombatStyle),
}

impl RunningState {
    pub fn new(game_state: GameState) -> Self {
        Self {
            frame_times: Default::default(),
            fps: Default::default(),
            last_save: Utc::now(),
            main_view_state: MainViewState::new(&game_state),
            last_view_duration: Duration::zero(),
            game_state,
        }
    }

    pub fn update(
        &mut self,
        configuration: Arc<RunConfiguration>,
        message: RunningMessage,
    ) -> Command<Message> {
        match message {
            RunningMessage::Init => {
                return Command::batch([self.main_view_state.update(
                    configuration,
                    MainViewMessage::Init,
                    &self.game_state,
                )])
            }
            RunningMessage::Update => {
                // measure time delta
                let current_time = Utc::now();
                let passed_real_milliseconds =
                    (current_time - self.game_state.last_update).num_milliseconds();
                if i128::from(passed_real_milliseconds) > GameTime::from_hours(1).milliseconds() {
                    return Command::perform(do_nothing(()), |()| {
                        Message::ChangeFromRunningToBulkUpdate
                    });
                } else if passed_real_milliseconds > 5_000 {
                    warn!(
                        "Making {:.0} seconds worth of updates",
                        passed_real_milliseconds as f64 / 1000.0
                    );
                } else {
                    trace!(
                        "Making {:.3} seconds worth of updates",
                        passed_real_milliseconds as f64 / 1000.0
                    );
                }

                // update game state
                let pre_update = Utc::now();
                self.game_state.update(passed_real_milliseconds);
                let post_update = Utc::now();
                let update_duration = post_update - pre_update;
                if configuration.profile {
                    info!(
                        "Update/View times: {}ms/{}ms",
                        update_duration.num_milliseconds(),
                        self.last_view_duration.num_milliseconds()
                    );
                }

                // measure fps
                {
                    let size = self.frame_times.len();
                    self.frame_times.push_back(current_time);
                    let front = *self.frame_times.front().unwrap();
                    let one_second_ago = current_time - Duration::seconds(1);
                    if front < one_second_ago {
                        assert!(size > 0);
                        self.fps = Some(
                            (size as f32)
                                / ((current_time - front).num_nanoseconds().unwrap() as f32 / 1e9),
                        );
                        while *self.frame_times.front().unwrap() < one_second_ago {
                            self.frame_times.pop_front();
                        }
                    }
                }

                if current_time - self.last_save >= *AUTOSAVE_INTERVAL {
                    // save game periodically
                    self.last_save = current_time;

                    return Command::perform(save_game_owned(self.game_state.clone()), |result| {
                        RunningMessage::GameSaved(result).into()
                    });
                }
            }
            RunningMessage::GameSaved(result) => match result {
                Ok(()) => info!("Game saved successfully"),
                Err(error) => error!("Error saving game: {error:?}"),
            },
            RunningMessage::SaveAndQuit => {
                return Command::perform(save_game_owned(self.game_state.clone()), |result| {
                    match result {
                        Ok(()) => {
                            info!("Game saved successfully!");
                        }
                        Err(error) => {
                            warn!("Game could not be saved: {}", error.to_string());
                        }
                    }
                    Message::Quit
                });
            }
            RunningMessage::GameState(game_state_message) => {
                match &game_state_message {
                    GameStateMessage::ActionChanged(action) => {
                        self.game_state.actions.selected_action = *action;
                    }
                    GameStateMessage::ActionChangedExplore(location) => {
                        self.game_state.actions.selected_action = ACTION_EXPLORE;
                        self.game_state.world.selected_location = *location;
                    }
                    GameStateMessage::ExplorationLocationChanged(location) => {
                        self.game_state.world.selected_location = *location;
                    }
                    GameStateMessage::CombatStyleChanged(combat_style) => {
                        self.game_state.character.selected_combat_style = *combat_style;
                    }
                }
                return self.main_view_state.update_game_state(
                    configuration,
                    &self.game_state,
                    &game_state_message,
                );
            }
            RunningMessage::MainView(main_view_message) => {
                return self.main_view_state.update(
                    configuration,
                    main_view_message,
                    &self.game_state,
                )
            }
        }

        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
        let pre_view = Utc::now();
        let result = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(title())
            .push(
                Row::new()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .push(
                        Column::new()
                            .width(Length::Units(220))
                            .align_items(Alignment::Fill)
                            .spacing(5)
                            .padding(5)
                            .push(
                                Text::new(&self.game_state.character.name)
                                    .size(40)
                                    .horizontal_alignment(Horizontal::Center),
                            )
                            .push(
                                Text::new(&format!("Level {}", self.game_state.character.level))
                                    .horizontal_alignment(Horizontal::Center),
                            )
                            .push(
                                Column::new().padding([0, 20]).push(
                                    ProgressBar::new(
                                        0.0..=(self.game_state.character.required_level_progress()
                                            as f32),
                                        self.game_state.character.level_progress as f32,
                                    )
                                    .height(Length::Units(10)),
                                ),
                            )
                            .push(
                                Text::new(&self.game_state.character.race.to_string())
                                    .horizontal_alignment(Horizontal::Center),
                            )
                            .push(
                                date(self.game_state.current_time)
                                    .horizontal_alignment(Horizontal::Center),
                            )
                            .push(
                                clock_time(self.game_state.current_time)
                                    .horizontal_alignment(Horizontal::Center),
                            )
                            .push(currency(self.game_state.inventory.currency, true))
                            .push(Space::new(Length::Shrink, Length::Units(20)))
                            .push(
                                Text::new("Attributes")
                                    .size(25)
                                    .horizontal_alignment(Horizontal::Center),
                            )
                            .push(
                                Column::new()
                                    .align_items(Alignment::Start)
                                    .padding([0, 20])
                                    .spacing(5)
                                    .push(attribute(
                                        "STR",
                                        self.game_state.character.attributes().strength,
                                        self.game_state.character.attribute_progress().strength,
                                    ))
                                    .push(attribute(
                                        "STA",
                                        self.game_state.character.attributes().stamina,
                                        self.game_state.character.attribute_progress().stamina,
                                    ))
                                    .push(attribute(
                                        "DEX",
                                        self.game_state.character.attributes().dexterity,
                                        self.game_state.character.attribute_progress().dexterity,
                                    ))
                                    .push(attribute(
                                        "INT",
                                        self.game_state.character.attributes().intelligence,
                                        self.game_state.character.attribute_progress().intelligence,
                                    ))
                                    .push(attribute(
                                        "WIS",
                                        self.game_state.character.attributes().wisdom,
                                        self.game_state.character.attribute_progress().wisdom,
                                    ))
                                    .push(attribute(
                                        "CHR",
                                        self.game_state.character.attributes().charisma,
                                        self.game_state.character.attribute_progress().charisma,
                                    )),
                            )
                            .push(Space::new(Length::Shrink, Length::Fill))
                            .push(
                                Text::new(&format!(
                                    "{}; FPS: {}",
                                    self.game_state.savegame_file.as_ref().to_string_lossy(),
                                    self.fps
                                        .map(|fps| format!("{:.0}", fps))
                                        .unwrap_or_else(|| "-".to_string())
                                ))
                                .size(12),
                            ),
                    )
                    .push(self.main_view_state.view(&self.game_state)),
            )
            .into();
        let post_view = Utc::now();
        self.last_view_duration = post_view - pre_view;
        result
    }

    pub fn into_game_state(self) -> GameState {
        self.game_state
    }
}

impl From<GameStateMessage> for Message {
    fn from(message: GameStateMessage) -> Self {
        Message::Running(RunningMessage::GameState(message))
    }
}
