use crate::game_state::character::{CharacterAttributeProgress, CharacterAttributeProgressFactor};
use crate::game_state::combat::SpawnedMonster;
use crate::game_state::currency::Currency;
use crate::game_state::time::GameTime;
use enum_iterator::Sequence;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

lazy_static! {
    pub static ref ACTIONS: BTreeMap<String, Action> = [
        Action::new("Wait", "waiting", "waited", ActionType::Rest, CharacterAttributeProgressFactor::zero(), Currency::zero(), 0),
        Action::new("Sleep", "sleeping", "slept", ActionType::Rest, CharacterAttributeProgressFactor::zero(), Currency::zero(), 0),
        Action::new("Tavern", "relaxing in the tavern", "relaxed in the tavern", ActionType::Rest, CharacterAttributeProgressFactor::from_charisma(1.0), Currency::from_copper(-10), 0),
        Action::new("Lift weights", "lifting weights", "lifted weights", ActionType::Train, CharacterAttributeProgressFactor::from_strength(1.0), Currency::zero(), 0),
        Action::new("Jog", "jogging", "jogged", ActionType::Train, CharacterAttributeProgressFactor::from_stamina(1.0), Currency::zero(), 0),
        Action::new("Practice juggling", "juggled", "practicing juggling", ActionType::Train, CharacterAttributeProgressFactor::from_dexterity(1.0), Currency::zero(), 0),
        Action::new("Study logic", "studying logic", "studied logic", ActionType::Train, CharacterAttributeProgressFactor::from_intelligence(1.0), Currency::zero(), 0),
        Action::new("Read", "reading", "read", ActionType::Train, CharacterAttributeProgressFactor::from_wisdom(1.0), Currency::zero(), 0),
        // most values computed depending on fighting style, monster, etc.
        Action::new("Fight monsters", "fighting monsters", "fought monsters", ActionType::Combat, CharacterAttributeProgressFactor::zero(), Currency::zero(), 0),
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Action {
    pub name: String,
    pub verb_progressive: String,
    pub verb_simple_past: String,
    pub action_type: ActionType,
    pub attribute_progress_factor: CharacterAttributeProgressFactor,
    pub currency_gain: Currency,
    pub required_level: u64,
}

impl Action {
    pub fn new(
        name: impl ToString,
        verb_progressive: impl ToString,
        verb_simple_past: impl ToString,
        action_type: ActionType,
        attribute_progress_factor: CharacterAttributeProgressFactor,
        currency_gain: Currency,
        required_level: u64,
    ) -> Self {
        Self {
            name: name.to_string(),
            verb_progressive: verb_progressive.to_string(),
            verb_simple_past: verb_simple_past.to_string(),
            action_type,
            attribute_progress_factor,
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
    pub attribute_progress: CharacterAttributeProgress,
    pub monster: Option<SpawnedMonster>,
    pub currency_reward: Currency,
    pub success: bool,
}

impl ActionInProgress {
    pub fn length(&self) -> GameTime {
        self.end - self.start
    }
}
