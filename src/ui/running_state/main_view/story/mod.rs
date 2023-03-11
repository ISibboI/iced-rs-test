use crate::game_state::story::quests::quest_stages::{CompiledQuestStage, QuestStageState};
use crate::game_state::story::quests::{CompiledQuest, CurrentQuestStage, QuestId};
use crate::ui::running_state::main_view::MainViewMessage;
use crate::ui::style::{ButtonStyleSheet, FramedContainer, SelectedButtonStyleSheet};
use crate::ui::Message;
use crate::GameState;
use iced::{
     Command,  Element, Length,
};
use iced::widget::{Column, Button, Container, ProgressBar, Row, Scrollable, Text};

#[derive(Debug, Clone)]
pub struct StoryState {
    selected_quest: Option<QuestId>,
}

#[derive(Debug, Clone)]
pub enum StoryMessage {
    Init,
    SelectQuest(QuestId),
}

impl StoryState {
    pub fn new() -> Self {
        Self {
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

    pub fn view(&self, game_state: &GameState) -> Element<Message> {
        let mut columns = Row::new().spacing(5).padding(5);
        columns = columns
            .push(view_quest_picker(
                self.selected_quest,
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
    selected_quest: Option<QuestId>,
    game_state: &GameState,
) -> Element<'result, Message> {
    let mut quest_picker = Column::new().spacing(5).padding(5);

    quest_picker = quest_picker.push(Text::new("Active quests").size(24));
    for quest in game_state
        .story
        .iter_active_quests_by_activation_time()
        .rev()
    {
        quest_picker = view_quest_button(quest_picker, selected_quest, quest);
    }

    quest_picker = quest_picker.push(Text::new("Completed quests").size(24));
    for quest in game_state
        .story
        .iter_completed_quests_by_completion_time()
        .rev()
    {
        quest_picker = view_quest_button(quest_picker, selected_quest, quest);
    }

    quest_picker = quest_picker.push(Text::new("Failed quests").size(24));
    for quest in game_state.story.iter_failed_quests_by_failure_time().rev() {
        quest_picker = view_quest_button(quest_picker, selected_quest, quest);
    }

    let quest_picker = Scrollable::new(quest_picker);
    let quest_picker = Container::new(quest_picker).style(FramedContainer);
    quest_picker.into()
}

fn view_quest_button<'a>(
    quest_picker: Column<'a, Message>,
    selected_quest: Option<QuestId>,
    quest: &CompiledQuest,
) -> Column<'a, Message> {

    quest_picker.push(
        Button::new( Text::new(quest.title.clone()))
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
