use crate::game_state::actions::Action;
use crate::game_state::combat::CombatStyle;
use crate::text_utils::a_or_an;
use crate::ui::elements::{attribute, labelled_element, title};
use crate::ui::Message;
use crate::{Configuration, GameState};
use async_std::fs::File;
use async_std::io::{BufWriter, WriteExt};
use async_std::task::sleep;
use enum_iterator::all;
use iced::alignment::Horizontal;
use iced::{pick_list, Alignment, Column, Command, Element, Length, PickList, Row, Space, Text};
use iced_native::widget::ProgressBar;
use log::{info, warn};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub const UI_SLEEP_BETWEEN_UPDATES: Duration = Duration::from_millis(0);

#[derive(Debug, Clone)]
pub struct RunningState {
    game_state: GameState,
    last_update: Instant,
    frame_times: VecDeque<Instant>,
    fps: Option<f32>,

    action_picker_state: pick_list::State<Action>,
    combat_style_picker_state: pick_list::State<CombatStyle>,
}

#[derive(Clone, Debug)]
pub enum RunningMessage {
    Init,
    Update,
    SaveAndQuit,

    ActionChanged(Action),
    CombatStyleChanged(CombatStyle),
}

impl RunningState {
    pub fn new(game_state: GameState) -> Self {
        Self {
            game_state,
            last_update: Instant::now(),
            frame_times: Default::default(),
            fps: Default::default(),
            action_picker_state: Default::default(),
            combat_style_picker_state: Default::default(),
        }
    }

    pub fn update(
        &mut self,
        _configuration: &Configuration,
        message: RunningMessage,
    ) -> Command<Message> {
        match message {
            RunningMessage::Init => {
                return Command::perform(sleep(UI_SLEEP_BETWEEN_UPDATES), |()| {
                    RunningMessage::Update.into()
                })
            }
            RunningMessage::Update => {
                let current_time = Instant::now();
                let passed_real_seconds = (current_time - self.last_update).as_secs_f64();
                self.game_state.update(passed_real_seconds);
                self.last_update = current_time;

                // measure frame time
                {
                    let size = self.frame_times.len();
                    self.frame_times.push_back(current_time);
                    let front = *self.frame_times.front().unwrap();
                    let one_second_ago = current_time - Duration::from_secs(1);
                    if front < one_second_ago {
                        assert!(size > 0);
                        self.fps = Some((size as f32) / (current_time - front).as_secs_f32());
                        while *self.frame_times.front().unwrap() < one_second_ago {
                            self.frame_times.pop_front();
                        }
                    }
                }

                return Command::perform(sleep(UI_SLEEP_BETWEEN_UPDATES), |()| {
                    RunningMessage::Update.into()
                });
            }
            RunningMessage::SaveAndQuit => {
                return Command::perform(save_game(self.game_state.clone()), |result| {
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
        }

        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
        let label_column_width = 130;

        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(title("Progress Quest"))
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
                                Text::new(&format!(
                                    "{}y {}m {}w {}d {}h {}m",
                                    self.game_state.current_time.years(),
                                    self.game_state.current_time.month_of_year(),
                                    self.game_state.current_time.week_of_month(),
                                    self.game_state.current_time.day_of_week(),
                                    self.game_state.current_time.hour_of_day(),
                                    self.game_state.current_time.minute_of_hour(),
                                ))
                                .horizontal_alignment(Horizontal::Center),
                            )
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
                                        self.game_state.character.strength,
                                        self.game_state.character.strength_progress,
                                    ))
                                    .push(attribute(
                                        "DEX",
                                        self.game_state.character.dexterity,
                                        self.game_state.character.dexterity_progress,
                                    ))
                                    .push(attribute(
                                        "INT",
                                        self.game_state.character.intelligence,
                                        self.game_state.character.intelligence_progress,
                                    ))
                                    .push(attribute(
                                        "CHR",
                                        self.game_state.character.charisma,
                                        self.game_state.character.charisma_progress,
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
                            .push(labelled_element(
                                "Selected action:",
                                label_column_width,
                                PickList::new(
                                    &mut self.action_picker_state,
                                    self.game_state.list_feasible_actions(),
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
                                    |combat_style| {
                                        RunningMessage::CombatStyleChanged(combat_style).into()
                                    },
                                ),
                            ))
                            .push(Space::new(Length::Shrink, Length::Fill))
                            .push(Text::new(&format!(
                                "{} is currently {}",
                                self.game_state.character.name,
                                if self.game_state.current_action.action == Action::FightMonsters {
                                    let monster_name = self
                                        .game_state
                                        .current_action
                                        .monster
                                        .as_ref()
                                        .unwrap()
                                        .to_lowercase_string();
                                    let a = a_or_an(&monster_name);
                                    format!("fighting {a} {monster_name}")
                                } else {
                                    self.game_state
                                        .current_action
                                        .action
                                        .verb_progressive()
                                        .to_string()
                                }
                            )))
                            .push(ProgressBar::new(
                                0.0..=1.0,
                                self.game_state.current_action_progress(),
                            )),
                    ),
            )
            .into()
    }
}

async fn save_game(game_state: GameState) -> Result<(), SaveError> {
    let path = &game_state.savegame_file;
    let savegame_file = File::create(path).await?;
    let savegame = serde_json::to_vec(&game_state)?;
    let mut writer = BufWriter::new(savegame_file);
    writer.write_all(&savegame).await?;
    writer.flush().await?;
    Ok(())
}

#[derive(Debug, Clone)]
pub enum SaveError {
    IoError(Arc<std::io::Error>),
    JsonError(Arc<serde_json::Error>),
}

impl From<std::io::Error> for SaveError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(Arc::new(error))
    }
}
impl From<serde_json::Error> for SaveError {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonError(Arc::new(error))
    }
}

impl ToString for SaveError {
    fn to_string(&self) -> String {
        match self {
            SaveError::IoError(error) => format!("IO error: {}", error),
            SaveError::JsonError(error) => format!("Serialization error: {}", error),
        }
    }
}
