use crate::game_state::combat::SpawnedMonster;
use crate::game_state::currency::Currency;
use crate::game_state::time::GameTime;
use enum_iterator::Sequence;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

lazy_static! {
    pub static ref ACTIONS: BTreeMap<String, Action> = [
        Action::new("Wait", "waiting", ActionType::Rest, (0.0, 0.0, 0.0, 0.0), Currency::zero(), 0),
        Action::new("Sleep", "sleeping", ActionType::Rest, (0.0, 0.0, 0.0, 0.0), Currency::zero(), 0),
        Action::new("Tavern", "relaxing in the tavern", ActionType::Rest, (0.0, 0.0, 0.0, 1.0), Currency::from_copper(-10), 0),
        Action::new("WeightLift", "lifting weights", ActionType::Train, (1.0, 0.0, 0.0, 0.0), Currency::zero(), 0),
        Action::new("Jog", "jogging", ActionType::Train, (0.0, 1.0, 0.0, 0.0), Currency::zero(), 0),
        Action::new("Read", "reading", ActionType::Train, (0.0, 0.0, 1.0, 0.0), Currency::zero(), 0),
        // most values computed depending on fighting style, monster, etc.
        Action::new("Fight monsters", "fighting monsters", ActionType::Combat, (0.0, 0.0, 0.0, 0.0), Currency::zero(), 0),
    ].into_iter().map(|action| (action.name.clone(), action)).collect();
}

pub static ACTION_WAIT: &str = "Wait";
pub static ACTION_SLEEP: &str = "Sleep";
pub static ACTION_TAVERN: &str = "Tavern";
pub static ACTION_FIGHT_MONSTERS: &str = "Fight monsters";

#[derive(Clone, Debug, Serialize, Deserialize, Sequence, Eq, PartialEq)]
pub enum ActionType {
    Rest,
    Train,
    Work,
    Combat,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Action {
    pub name: String,
    pub verb_progressive: String,
    pub action_type: ActionType,
    pub attribute_progress_str_dex_int_chr: (f64, f64, f64, f64),
    pub currency_gain: Currency,
    pub required_level: usize,
}

impl Action {
    pub fn new(
        name: impl ToString,
        verb_progressive: impl ToString,
        action_type: ActionType,
        attribute_progress_str_dex_int_chr: (f64, f64, f64, f64),
        currency_gain: Currency,
        required_level: usize,
    ) -> Self {
        Self {
            name: name.to_string(),
            verb_progressive: verb_progressive.to_string(),
            action_type,
            attribute_progress_str_dex_int_chr,
            currency_gain,
            required_level,
        }
    }
}

impl ToString for Action {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionInProgress {
    pub action: Action,
    pub start: GameTime,
    pub end: GameTime,
    pub attribute_progress: (f64, f64, f64, f64),
    pub monster: Option<SpawnedMonster>,
    pub currency_reward: Currency,
    pub success: bool,
}

impl ActionInProgress {
    pub fn length(&self) -> GameTime {
        self.end - self.start
    }
}
