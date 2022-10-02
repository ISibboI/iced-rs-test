use crate::game_state::time::GameTime;
use crate::game_template::IdMaps;
use event_trigger_action_system::TriggerHandle;
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

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, Default, Eq, PartialEq, Hash, Ord, PartialOrd,
)]
pub struct QuestId(pub usize);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub id_str: String,
    pub title: String,
    pub description: String,
    pub activation_condition: String,
    pub completion_condition: String,
    pub failure_condition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledQuest {
    pub id: QuestId,
    pub id_str: String,
    pub title: String,
    pub description: String,
    pub activation_condition: TriggerHandle,
    pub completion_condition: TriggerHandle,
    pub failure_condition: TriggerHandle,
    pub state: QuestState,
}

impl Quest {
    pub fn compile(self, id_maps: &IdMaps) -> CompiledQuest {
        CompiledQuest {
            id: *id_maps.quests.get(&self.id_str).unwrap(),
            id_str: self.id_str,
            title: self.title,
            description: self.description,
            activation_condition: *id_maps.triggers.get(&self.activation_condition).unwrap(),
            completion_condition: *id_maps.triggers.get(&self.completion_condition).unwrap(),
            failure_condition: *id_maps.triggers.get(&self.failure_condition).unwrap(),
            state: QuestState::Inactive,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum QuestState {
    Inactive,
    Active {
        activation_time: GameTime,
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
    },
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

    pub fn activation_time(&self) -> Option<GameTime> {
        match self {
            QuestState::Inactive => None,
            QuestState::Active { activation_time } => Some(*activation_time),
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
}

impl From<usize> for QuestId {
    fn from(n: usize) -> Self {
        QuestId(n)
    }
}
