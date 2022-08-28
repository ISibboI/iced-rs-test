use crate::game_state::combat::CombatStyle;
use crate::savegames::{save_game_owned, SaveError};
use crate::ui::elements::{
    active_action_description, attribute, clock_time, currency, date, event_log, labelled_element,
    labelled_label, scrollable_quest_column, title,
};
use crate::ui::Message;
use crate::{Configuration, GameState};
use chrono::{DateTime, Duration, Utc};
use enum_iterator::all;
use iced::alignment::Horizontal;
use iced::{
    pick_list, scrollable, Alignment, Column, Command, Element, Length, PickList, Row, Space, Text,
};
use iced_native::widget::ProgressBar;
use lazy_static::lazy_static;
use log::{error, info, trace, warn};
use std::collections::VecDeque;

lazy_static! {
    pub static ref AUTOSAVE_INTERVAL: Duration = Duration::seconds(10);
}

#[derive(Debug, Clone)]
pub struct RunningState {
    game_state: GameState,
    frame_times: VecDeque<DateTime<Utc>>,
    fps: Option<f32>,
    last_save: DateTime<Utc>,

    action_picker_state: pick_list::State<String>,
    combat_style_picker_state: pick_list::State<CombatStyle>,
    combat_location_picker_state: pick_list::State<String>,
    quest_column_scrollable_state: scrollable::State,
    event_log_scrollable_state: scrollable::State,
}

#[derive(Clone, Debug)]
pub enum RunningMessage {
    Init,
    Update,
    GameSaved(Result<(), SaveError>),
    SaveAndQuit,

    ActionChanged(String),
    CombatStyleChanged(CombatStyle),
    CombatLocationChanged(String),
}

impl RunningState {
    pub fn new(game_state: GameState) -> Self {
        Self {
            game_state,
            frame_times: Default::default(),
            fps: Default::default(),
            last_save: Utc::now(),
            action_picker_state: Default::default(),
            combat_style_picker_state: Default::default(),
            combat_location_picker_state: Default::default(),
            quest_column_scrollable_state: Default::default(),
            event_log_scrollable_state: Default::default(),
        }
    }

    pub fn update(
        &mut self,
        _configuration: &Configuration,
        message: RunningMessage,
    ) -> Command<Message> {
        match message {
            RunningMessage::Init => {}
            RunningMessage::Update => {
                // measure time delta
                let current_time = Utc::now();
                let passed_real_milliseconds =
                    (current_time - self.game_state.last_update).num_milliseconds();
                if passed_real_milliseconds > 60_000 {
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
                self.game_state.update(passed_real_milliseconds);

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
            RunningMessage::ActionChanged(action) => {
                self.game_state.selected_action = action;
            }
            RunningMessage::CombatStyleChanged(combat_style) => {
                self.game_state.selected_combat_style = combat_style;
            }
            RunningMessage::CombatLocationChanged(combat_location) => {
                self.game_state.selected_combat_location = combat_location;
            }
        }

        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
        let label_column_width = 160;

        let action_column = Column::new()
            .width(Length::Shrink)
            .height(Length::Fill)
            .spacing(5)
            .padding(5)
            .push(labelled_element(
                "Selected action:",
                label_column_width,
                PickList::new(
                    &mut self.action_picker_state,
                    self.game_state
                        .list_feasible_actions()
                        .map(|action| action.name.clone())
                        .collect::<Vec<_>>(),
                    Some(self.game_state.selected_action.clone()),
                    |action| RunningMessage::ActionChanged(action).into(),
                ),
            ))
            .push(labelled_element(
                "Combat style:",
                label_column_width,
                PickList::new(
                    &mut self.combat_style_picker_state,
                    all::<CombatStyle>().collect::<Vec<_>>(),
                    Some(self.game_state.selected_combat_style.clone()),
                    |combat_style| RunningMessage::CombatStyleChanged(combat_style).into(),
                ),
            ))
            .push(labelled_label(
                "Damage per minute:",
                label_column_width,
                format!("{:.0}", self.game_state.damage_output()),
            ))
            .push(labelled_element(
                "Combat location:",
                label_column_width,
                PickList::new(
                    &mut self.combat_location_picker_state,
                    self.game_state
                        .list_feasible_locations()
                        .map(|location| location.name.clone())
                        .collect::<Vec<_>>(),
                    Some(self.game_state.selected_combat_location.clone()),
                    |combat_location| RunningMessage::CombatLocationChanged(combat_location).into(),
                ),
            ));

        Column::new()
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
                            .push(currency(self.game_state.character.currency, true))
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
                                    self.game_state.savegame_file,
                                    self.fps
                                        .map(|fps| format!("{:.0}", fps))
                                        .unwrap_or_else(|| "-".to_string())
                                ))
                                .size(12),
                            ),
                    )
                    .push(
                        Column::new()
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .spacing(5)
                            .padding(5)
                            .push(
                                Row::new()
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                                    .spacing(5)
                                    .padding(5)
                                    .push(action_column)
                                    .push(
                                        scrollable_quest_column(
                                            &self.game_state.story,
                                            &mut self.quest_column_scrollable_state,
                                        )
                                        .width(Length::Units(300))
                                        .height(Length::Fill),
                                    )
                                    .push(
                                        event_log(
                                            &self.game_state,
                                            &mut self.event_log_scrollable_state,
                                        )
                                        .width(Length::Units(300))
                                        .height(Length::Fill),
                                    ),
                            )
                            .push(active_action_description(&self.game_state))
                            .push(ProgressBar::new(
                                0.0..=1.0,
                                self.game_state.current_action_progress(),
                            )),
                    ),
            )
            .into()
    }
}
