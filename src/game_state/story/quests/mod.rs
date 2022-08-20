use crate::game_state::actions::{
    ActionInProgress, ACTION_FIGHT_MONSTERS, ACTION_SLEEP, ACTION_TAVERN,
};
use crate::game_state::story::quests::quest_conditions::*;
use crate::game_state::time::GameTime;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod quest_conditions;

lazy_static! {
    pub static ref QUESTS: HashMap<String, Quest> = init_quests();
}

fn init_quests() -> HashMap<String, Quest> {
    [
        Quest::new(
            "init",
            "Wake up!",
            "Wait until six o'clock, and you will wake up to a new day full of adventure!",
            [],
            action_is(ACTION_SLEEP) & time_geq(GameTime::from_seconds(1)) // dodge the initial dummy sleeping action that ends at time 0
        ),
        Quest::new("train_str", "Lift weights", "Lift weights a few times to gain some strength.", ["init"], action_count("Lift weights", 5)),
        Quest::new("train_sta", "Go for a run", "Jog around a bit to increase your stamina.", ["init"], action_count("Jog", 5)),
        Quest::new("train_dex", "Try out juggling", "Practice some juggling to improve your dexterity.", ["init"], action_count("Practice juggling", 5)),
        Quest::new("train_int", "Train your brain", "Read a book about logic to improve your intelligence.", ["init"], action_count("Study logic", 5)),
        Quest::new("train_wis", "Read a book", "Read a book about the world to increase your wisdom.", ["init"], action_count("Read", 5)),
        Quest::new("train_chr", "Talk to some strangers", "Visit the tavern and talk to some people to gain some charisma.", ["init"], action_count(ACTION_TAVERN, 5)),
        Quest::hidden("fight_monsters_pre", [], any_n([completed("train_str"), completed("train_sta"), completed("train_dex"), completed("train_int"), completed("train_wis"), completed("train_chr")], 2)),
        Quest::new("fight_monsters", "Fight some monsters", "You have done some basic training. Put it to work by being a hero and killing some beasts and bad guys!", ["fight_monsters_pre"], action_count(ACTION_FIGHT_MONSTERS, 10)),
    ]
        .into_iter()
        .map(|quest| (quest.id.clone(), quest))
        .collect()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub id: String,
    pub title: String,
    pub description: String,
    pub precondition: Vec<String>,
    pub condition: QuestCondition,
    pub hidden: bool,
}

impl Quest {
    fn new<'a>(
        id: impl ToString,
        title: impl ToString,
        description: impl ToString,
        precondition: impl AsRef<[&'a str]>,
        condition: impl Into<QuestCondition>,
    ) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            precondition: precondition
                .as_ref()
                .iter()
                .map(|s| s.to_string())
                .collect(),
            condition: condition.into(),
            hidden: false,
        }
    }

    fn hidden<'a>(
        id: impl ToString,
        precondition: impl AsRef<[&'a str]>,
        condition: impl Into<QuestCondition>,
    ) -> Self {
        Self {
            id: id.to_string(),
            title: Default::default(),
            description: Default::default(),
            precondition: precondition
                .as_ref()
                .iter()
                .map(|s| s.to_string())
                .collect(),
            condition: condition.into(),
            hidden: true,
        }
    }

    pub fn update(
        &mut self,
        action_in_progress: &ActionInProgress,
        completed_quests: &HashMap<String, Quest>,
    ) -> bool {
        self.condition.update(action_in_progress, completed_quests)
    }
}
