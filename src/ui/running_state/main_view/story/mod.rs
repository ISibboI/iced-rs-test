use crate::game_state::story::quests::quest_stages::{CompiledQuestStage, QuestStageState};
use crate::game_state::story::quests::{CompiledQuest, CurrentQuestStage, QuestId};
use crate::ui::elements::ERROR_COLOR;
use crate::ui::running_state::main_view::MainViewMessage;
use crate::ui::style::{ButtonStyleSheet, FramedContainer, SelectedButtonStyleSheet};
use crate::ui::Message;
use crate::GameState;
use iced::{
    button, scrollable, Button, Column, Command, Container, Element, Length, ProgressBar, Row,
    Scrollable, Text,
};
use std::ptr;

#[derive(Debug, Clone)]
pub struct StoryState {
    quest_picker_state: scrollable::State,
    quest_buttons: Vec<(bool, button::State)>,
    selected_quest: Option<QuestId>,
}

#[derive(Debug, Clone)]
pub enum StoryMessage {
    Init,
    SelectQuest(QuestId),
}

impl StoryState {
    pub fn new(game_state: &GameState) -> Self {
        Self {
            quest_picker_state: Default::default(),
            quest_buttons: game_state
                .story
                .iter_all_quests()
                .enumerate()
                .map(|(index, quest)| {
                    debug_assert_eq!(index, quest.id.0);
                    (false, Default::default())
                })
                .collect(),
            selected_quest: Default::default(),
        }
    }

    pub fn update(&mut self, message: StoryMessage, game_state: &GameState) -> Command<Message> {
        match message {
            StoryMessage::Init => {
                if self.selected_quest.is_none() {
                    if let Some(quest) = game_state
                        .story
                        .iter_active_quests_by_activation_time()
                        .rev()
                        .chain(
                            game_state
                                .story
                                .iter_completed_quests_by_completion_time()
                                .rev(),
                        )
                        .chain(game_state.story.iter_failed_quests_by_failure_time().rev())
                        .next()
                    {
                        self.selected_quest = Some(quest.id);
                    }
                }
            }
            StoryMessage::SelectQuest(quest_id) => {
                self.selected_quest = Some(quest_id);
            }
        }

        Command::none()
    }

    pub fn view(&mut self, game_state: &GameState) -> Element<Message> {
        let mut columns = Row::new().spacing(5).padding(5);
        columns = columns
            .push(view_quest_picker(
                &mut self.quest_buttons,
                self.selected_quest,
                &mut self.quest_picker_state,
                game_state,
            ))
            .push(view_quest(self.selected_quest, game_state));

        Container::new(columns)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(FramedContainer)
            .into()
    }
}

fn view_quest<'result>(
    selected_quest: Option<QuestId>,
    game_state: &GameState,
) -> Element<'result, Message> {
    Container::new(if let Some(quest_id) = selected_quest {
        let quest = game_state.story.quest(quest_id);

        let mut stages = Column::new().spacing(5);
        for stage in quest.completed_stages() {
            stages = stages.push(view_quest_stage(stage, game_state));
        }

        match quest.current_stage() {
            CurrentQuestStage::Inactive => unreachable!(),
            CurrentQuestStage::Active(stage) => {
                stages = stages.push(view_quest_stage(stage, game_state));
            }
            CurrentQuestStage::Completed => {}
            CurrentQuestStage::FailedWhileInactive => {}
            CurrentQuestStage::FailedWhileActive(stage) => {
                stages = stages.push(view_quest_stage(stage, game_state));
            }
        }

        let mut rows = Column::new().spacing(5).width(Length::Fill);
        rows = rows.push(Text::new(quest.title.clone()).size(24));
        if let Some(description) = quest.description.clone() {
            rows = rows.push(Text::new(description));
        }
        rows = rows.push(stages);

        Element::from(rows)
    } else {
        Element::from(Text::new("No quest selected"))
    })
    .padding(5)
    .width(Length::Fill)
    .style(FramedContainer)
    .into()
}

fn view_quest_stage<'result>(
    stage: &CompiledQuestStage,
    game_state: &GameState,
) -> Element<'result, Message> {
    let mut stage_rows = Column::new().spacing(5);
    if let Some(description) = stage.description.clone() {
        stage_rows = stage_rows.push(Text::new(description));
    }
    stage_rows = stage_rows.push(Text::new(stage.task.clone()));

    match stage.state {
        QuestStageState::Active { .. } => {
            let (progress, goal) = game_state
                .triggers
                .progress(stage.completion_condition)
                .unwrap();
            stage_rows = stage_rows.push(
                ProgressBar::new(1.0..=goal as f32, progress as f32).height(Length::Units(10)),
            );
        }
        QuestStageState::FailedWhileInactive { .. } | QuestStageState::FailedWhileActive { .. } => {
            stage_rows = stage_rows.push(Text::new("failed").color(*ERROR_COLOR));
        }
        _ => {}
    }

    Container::new(stage_rows)
        .padding(5)
        .width(Length::Fill)
        .style(FramedContainer)
        .into()
}

fn view_quest_picker<'result, 'quest_buttons: 'result, 'quest_picker_state: 'result>(
    quest_buttons: &'quest_buttons mut [(bool, button::State)],
    selected_quest: Option<QuestId>,
    quest_picker_state: &'quest_picker_state mut scrollable::State,
    game_state: &GameState,
) -> Element<'result, Message> {
    quest_buttons
        .iter_mut()
        .for_each(|quest_button| quest_button.0 = false);

    let mut quest_picker = Column::new().spacing(5).padding(5);

    quest_picker = quest_picker.push(Text::new("Active quests").size(24));
    for quest in game_state
        .story
        .iter_active_quests_by_activation_time()
        .rev()
    {
        quest_picker = view_quest_button(quest_picker, quest_buttons, selected_quest, quest);
    }

    quest_picker = quest_picker.push(Text::new("Completed quests").size(24));
    for quest in game_state
        .story
        .iter_completed_quests_by_completion_time()
        .rev()
    {
        quest_picker = view_quest_button(quest_picker, quest_buttons, selected_quest, quest);
    }

    quest_picker = quest_picker.push(Text::new("Failed quests").size(24));
    for quest in game_state.story.iter_failed_quests_by_failure_time().rev() {
        quest_picker = view_quest_button(quest_picker, quest_buttons, selected_quest, quest);
    }

    let quest_picker = Scrollable::new(quest_picker_state).push(quest_picker);
    let quest_picker = Container::new(quest_picker).style(FramedContainer);
    quest_picker.into()
}

fn view_quest_button<'a>(
    quest_picker: Column<'a, Message>,
    quest_buttons: &mut [(bool, button::State)],
    selected_quest: Option<QuestId>,
    quest: &CompiledQuest,
) -> Column<'a, Message> {
    let quest_button = &mut quest_buttons[quest.id.0];
    assert!(!quest_button.0);
    quest_button.0 = true;
    let quest_button_state = unsafe { ptr::addr_of_mut!(quest_button.1).as_mut().unwrap() };

    quest_picker.push(
        Button::new(quest_button_state, Text::new(quest.title.clone()))
            .on_press(StoryMessage::SelectQuest(quest.id).into())
            .style(if selected_quest == Some(quest.id) {
                SelectedButtonStyleSheet::style_sheet()
            } else {
                ButtonStyleSheet::style_sheet()
            }),
    )
}

impl From<StoryMessage> for Message {
    fn from(story_message: StoryMessage) -> Self {
        MainViewMessage::Story(story_message).into()
    }
}
