use crate::game_state::combat::SpawnedMonster;
use crate::game_state::time::GameTime;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default, Sequence, Eq, PartialEq)]
pub enum Action {
    #[default]
    Wait,
    Sleep,
    Tavern,
    WeightLift,
    Jog,
    Read,
    FightMonsters,
}

impl Action {
    pub fn verb_progressive(&self) -> &str {
        match self {
            Action::Wait => "waiting",
            Action::Sleep => "sleeping",
            Action::Tavern => "relaxing in the tavern",
            Action::WeightLift => "lifting weights",
            Action::Jog => "jogging",
            Action::Read => "reading",
            Action::FightMonsters => "fighting monsters",
        }
    }

    pub fn attribute_progress_str_dex_int_chr(&self) -> (f64, f64, f64, f64) {
        match self {
            Action::Wait => (0.0, 0.0, 0.0, 0.0),
            Action::Sleep => (0.0, 0.0, 0.0, 0.0),
            Action::Tavern => (0.0, 0.0, 0.0, 1.0),
            Action::WeightLift => (1.0, 0.0, 0.0, 0.0),
            Action::Jog => (0.0, 1.0, 0.0, 0.0),
            Action::Read => (0.0, 0.0, 1.0, 0.0),
            Action::FightMonsters => (0.0, 0.0, 0.0, 0.0), // progress is computed depending on combat style
        }
    }
}

impl ToString for Action {
    fn to_string(&self) -> String {
        match self {
            Action::Wait => "Wait",
            Action::Sleep => "Sleep",
            Action::Tavern => "Relax in the tavern",
            Action::WeightLift => "Lift weights",
            Action::Jog => "Jog",
            Action::Read => "Read",
            Action::FightMonsters => "Fight monsters",
        }
        .to_string()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionInProgress {
    pub action: Action,
    pub start: GameTime,
    pub end: GameTime,
    pub attribute_progress: (f64, f64, f64, f64),
    pub monster: Option<SpawnedMonster>,
}

impl ActionInProgress {
    pub fn length(&self) -> GameTime {
        self.end - self.start
    }
}
