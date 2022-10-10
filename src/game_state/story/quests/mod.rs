use crate::game_state::time::GameTime;
use crate::game_state::triggers::CompiledGameEvent;
use crate::game_template::IdMaps;
use event_trigger_action_system::TriggerHandle;
use log::debug;
use quest_stages::{CompiledQuestStage, QuestStage, QuestStageId, QuestStageState};
use serde::{Deserialize, Serialize};

/*pub fn init_quests() -> Vec<Quest> {
    vec![
        Quest::new(
            "init",
            "Wake up!",
            "Wait until six o'clock, and you will wake up to a new day full of adventure!",
            none(),
            action_is("Sleep"),
        ),
        Quest::new("train_str", "Lift weights", "Lift weights a few times to gain some strength.", quest_completed("init"), action_count("Lift weights", 5)),
        Quest::new("train_sta", "Go for a run", "Jog around a bit to increase your stamina.", quest_completed("init"), action_count("Jog", 5)),
        Quest::new("train_dex", "Try out juggling", "Practice some juggling to improve your dexterity.", quest_completed("init"), action_count("Practice juggling", 5)),
        Quest::new("train_int", "Train your brain", "Read a book about logic to improve your intelligence.", quest_completed("init"), action_count("Study logic", 5)),
        Quest::new("train_wis", "Read a book", "Read a book about the world to increase your wisdom.", quest_completed("init"), action_count("Read", 5)),
        Quest::new("train_chr", "Talk to some strangers", "Visit the tavern and talk to some people to gain some charisma.", quest_completed("init"), action_count("Tavern", 5)),
        Quest::new("fight_monsters", "Fight some monsters", "You have done some basic training. Put it to work by being a hero and killing some beasts and bad guys!", any_n([quest_completed("train_str"), quest_completed("train_sta"), quest_completed("train_dex"), quest_completed("train_int"), quest_completed("train_wis"), quest_completed("train_chr")], 2), action_count("Fight monsters", 10)),
    ]
}*/

pub mod quest_stages;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub id_str: String,
    pub title: String,
    pub description: Option<String>,
    pub activation_condition: String,
    pub failure_condition: String,
    pub stages: Vec<QuestStage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledQuest {
    pub id: QuestId,
    pub id_str: String,
    pub title: String,
    pub description: Option<String>,
    pub activation_condition: TriggerHandle,
    pub failure_condition: TriggerHandle,
    stages: Vec<CompiledQuestStage>,
    state: QuestState,
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, Default, Eq, PartialEq, Hash, Ord, PartialOrd,
)]
pub struct QuestId(pub usize);

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum QuestState {
    Inactive,
    Active {
        activation_time: GameTime,
        active_stage: usize,
    },
    Completed {
        activation_time: GameTime,
        completion_time: GameTime,
    },
    FailedWhileInactive {
        failure_time: GameTime,
    },
    FailedWhileActive {
        activation_time: GameTime,
        failure_time: GameTime,
        failed_stage: usize,
    },
}

#[derive(Debug, Clone)]
pub enum CurrentQuestStage<'a> {
    Inactive,
    Active(&'a CompiledQuestStage),
    Completed,
    FailedWhileInactive,
    FailedWhileActive(&'a CompiledQuestStage),
}

impl Quest {
    pub fn compile(self, id_maps: &IdMaps) -> CompiledQuest {
        let id = *id_maps.quests.get(&self.id_str).unwrap();
        CompiledQuest {
            id,
            id_str: self.id_str,
            title: self.title,
            description: self.description,
            activation_condition: *id_maps.triggers.get(&self.activation_condition).unwrap(),
            failure_condition: *id_maps.triggers.get(&self.failure_condition).unwrap(),
            stages: self
                .stages
                .into_iter()
                .map(|stage| stage.compile(id_maps, id))
                .collect(),
            state: QuestState::Inactive,
        }
    }
}

impl CompiledQuest {
    pub fn active_stage(&self) -> Option<&CompiledQuestStage> {
        match self.state {
            QuestState::Active { active_stage, .. } => Some(self.stages.get(active_stage).unwrap()),
            _ => None,
        }
    }

    pub fn active_stage_mut(&mut self) -> Option<&mut CompiledQuestStage> {
        match self.state {
            QuestState::Active { active_stage, .. } => {
                Some(self.stages.get_mut(active_stage).unwrap())
            }
            _ => None,
        }
    }

    pub fn failed_stage(&self) -> Option<&CompiledQuestStage> {
        match self.state {
            QuestState::FailedWhileInactive { .. } => Some(self.stages.first().unwrap()),
            QuestState::FailedWhileActive { failed_stage, .. } => {
                Some(self.stages.get(failed_stage).unwrap())
            }
            _ => None,
        }
    }

    pub fn state(&self) -> &QuestState {
        &self.state
    }

    pub fn completed_stages(&self) -> impl Iterator<Item = &'_ CompiledQuestStage> {
        self.stages.iter().take(match self.state {
            QuestState::Inactive => 0,
            QuestState::Active { active_stage, .. } => active_stage,
            QuestState::Completed { .. } => self.stages.len(),
            QuestState::FailedWhileInactive { .. } => 0,
            QuestState::FailedWhileActive { failed_stage, .. } => failed_stage,
        })
    }

    pub fn current_stage(&self) -> CurrentQuestStage<'_> {
        match self.state {
            QuestState::Inactive => CurrentQuestStage::Inactive,
            QuestState::Active { .. } => CurrentQuestStage::Active(self.active_stage().unwrap()),
            QuestState::Completed { .. } => CurrentQuestStage::Completed,
            QuestState::FailedWhileInactive { .. } => CurrentQuestStage::FailedWhileInactive,
            QuestState::FailedWhileActive { .. } => {
                CurrentQuestStage::FailedWhileActive(self.failed_stage().unwrap())
            }
        }
    }

    pub fn activate(&mut self, time: GameTime) -> impl Iterator<Item = CompiledGameEvent> {
        assert!(self.state.is_inactive());
        self.state = QuestState::Active {
            activation_time: time,
            active_stage: 0,
        };
        let stage = self.stages.first_mut().unwrap();
        assert!(stage.state.is_inactive());
        stage.state = QuestStageState::Active {
            activation_time: time,
        };

        Some(CompiledGameEvent::QuestStageActivated {
            id: QuestStageId {
                quest_id: self.id,
                stage_id: 0,
            },
        })
        .into_iter()
    }

    pub fn complete_quest_stage(
        &mut self,
        quest_stage_id: QuestStageId,
        time: GameTime,
        complete_quest_callback: impl FnOnce(GameTime),
    ) -> Box<dyn Iterator<Item = CompiledGameEvent>> // this should be possible with impl Trait once #79415 is fixed
    {
        assert!(self.state.is_active());
        debug!(
            "Completing quest stage {} {}",
            self.id_str,
            self.active_stage().unwrap().id_str
        );
        let active_stage = self.active_stage_mut().unwrap();
        assert!(active_stage.state.is_active());
        assert_eq!(active_stage.id, quest_stage_id);
        let activation_time = active_stage.state.activation_time().unwrap();
        active_stage.state = QuestStageState::Completed {
            activation_time,
            completion_time: time,
        };

        let new_stage_id = self.state.increment_active_stage();
        if let Some(active_stage) = self.stages.get_mut(new_stage_id) {
            assert!(active_stage.state.is_inactive());
            active_stage.state = QuestStageState::Active {
                activation_time: time,
            };
            Box::new(
                Some(CompiledGameEvent::QuestStageActivated {
                    id: QuestStageId {
                        quest_id: self.id,
                        stage_id: self.state.active_stage().unwrap(),
                    },
                })
                .into_iter(),
            )
        } else {
            let activation_time = self.state.activation_time().unwrap();
            debug!("Completing quest {}", self.id_str);
            self.state = QuestState::Completed {
                activation_time,
                completion_time: time,
            };

            complete_quest_callback(activation_time);
            Box::new(Some(CompiledGameEvent::QuestCompleted { id: self.id }).into_iter())
        }
    }

    pub fn fail(
        &mut self,
        time: GameTime,
        fail_quest_callback: impl FnOnce(Option<GameTime>),
    ) -> Box<dyn Iterator<Item = CompiledGameEvent>> // this should be possible with impl Trait once #79415 is fixed
    {
        let quest_id = self.id;
        let failed_event_creator = move |stage_id| CompiledGameEvent::QuestStageFailed {
            id: QuestStageId { quest_id, stage_id },
        };

        assert!(!self.state.is_failed());

        Box::new(match self.state {
            QuestState::Inactive => {
                self.state = QuestState::FailedWhileInactive { failure_time: time };
                fail_quest_callback(None);

                for stage in &mut self.stages {
                    assert!(stage.state.is_inactive());
                    stage.state = QuestStageState::FailedWhileInactive { failure_time: time };
                }

                (0..self.stages.len()).map(failed_event_creator)
            }
            QuestState::Active {
                activation_time,
                active_stage,
            } => {
                self.state = QuestState::FailedWhileActive {
                    activation_time,
                    failure_time: time,
                    failed_stage: active_stage,
                };
                fail_quest_callback(Some(activation_time));

                assert!(self.stages[active_stage].state.is_active());
                let active_stage_activation_time =
                    self.stages[active_stage].state.activation_time().unwrap();
                self.stages[active_stage].state = QuestStageState::FailedWhileActive {
                    activation_time: active_stage_activation_time,
                    failure_time: time,
                };
                for stage in self.stages.iter_mut().skip(active_stage + 1) {
                    assert!(stage.state.is_inactive());
                    stage.state = QuestStageState::FailedWhileInactive { failure_time: time };
                }

                (active_stage..self.stages.len()).map(failed_event_creator)
            }
            _ => unreachable!(),
        })
    }
}

#[allow(dead_code)]
impl QuestState {
    pub fn is_inactive(&self) -> bool {
        matches!(self, QuestState::Inactive)
    }

    pub fn is_active(&self) -> bool {
        matches!(self, QuestState::Active { .. })
    }

    pub fn is_completed(&self) -> bool {
        matches!(self, QuestState::Completed { .. })
    }

    pub fn is_failed(&self) -> bool {
        matches!(
            self,
            QuestState::FailedWhileInactive { .. } | QuestState::FailedWhileActive { .. }
        )
    }

    pub fn activation_time(&self) -> Option<GameTime> {
        match self {
            QuestState::Inactive => None,
            QuestState::Active {
                activation_time, ..
            } => Some(*activation_time),
            QuestState::Completed {
                activation_time, ..
            } => Some(*activation_time),
            QuestState::FailedWhileInactive { .. } => None,
            QuestState::FailedWhileActive {
                activation_time, ..
            } => Some(*activation_time),
        }
    }

    pub fn completion_time(&self) -> Option<GameTime> {
        match self {
            QuestState::Inactive => None,
            QuestState::Active { .. } => None,
            QuestState::Completed {
                completion_time, ..
            } => Some(*completion_time),
            QuestState::FailedWhileInactive { .. } => None,
            QuestState::FailedWhileActive { .. } => None,
        }
    }

    pub fn active_stage(&self) -> Option<usize> {
        match self {
            QuestState::Active { active_stage, .. } => Some(*active_stage),
            _ => None,
        }
    }

    pub fn increment_active_stage(&mut self) -> usize {
        match self {
            QuestState::Active { active_stage, .. } => {
                *active_stage += 1;
                *active_stage
            }
            _ => panic!(),
        }
    }
}

impl From<usize> for QuestId {
    fn from(n: usize) -> Self {
        Self(n)
    }
}
