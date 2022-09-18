use crate::game_state::character::{CharacterAttributeProgress, CharacterAttributeProgressFactor};
use crate::game_state::currency::Currency;
use crate::game_state::time::GameTime;
use crate::game_state::triggers::CompiledGameEvent;
use crate::game_state::world::events::ExplorationEventId;
use crate::game_state::world::monsters::MonsterId;
use crate::game_template::parser::error::{ParserError, ParserErrorKind};
use crate::game_template::IdMaps;
use enum_iterator::Sequence;
use event_trigger_action_system::TriggerHandle;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::iter;
use std::str::FromStr;

pub static ACTION_WAIT: PlayerActionId = PlayerActionId(0);
pub static ACTION_SLEEP: PlayerActionId = PlayerActionId(1);
pub static ACTION_TAVERN: PlayerActionId = PlayerActionId(2);
pub static ACTION_EXPLORE: PlayerActionId = PlayerActionId(3);

/*pub fn init_actions() -> Vec<PlayerAction> {
    vec![
        PlayerAction::new(
            "wait",
            "Wait",
            "waiting",
            "waited",
            PlayerActionType::Rest,
            CharacterAttributeProgressFactor::zero(),
            Currency::zero(),
        ),
        PlayerAction::new(
            "sleep",
            "Sleep",
            "sleeping",
            "slept",
            PlayerActionType::Rest,
            CharacterAttributeProgressFactor::zero(),
            Currency::zero(),
        ),
        PlayerAction::new(
            "tavern",
            "Tavern",
            "relaxing in the tavern",
            "relaxed in the tavern",
            PlayerActionType::Rest,
            CharacterAttributeProgressFactor::from_charisma(1.0),
            Currency::from_copper(-10),
        ),
        // most values computed depending on fighting style, monster, etc.
        PlayerAction::new(
            "fight",
            "Fight monsters",
            "fighting monsters",
            "fought monsters",
            PlayerActionType::Combat,
            CharacterAttributeProgressFactor::zero(),
            Currency::zero(),
        ),
        PlayerAction::new(
            "train_str",
            "Lift weights",
            "lifting weights",
            "lifted weights",
            PlayerActionType::Train,
            CharacterAttributeProgressFactor::from_strength(1.0),
            Currency::zero(),
        ),
        PlayerAction::new(
            "train_sta",
            "Jog",
            "jogging",
            "jogged",
            PlayerActionType::Train,
            CharacterAttributeProgressFactor::from_stamina(1.0),
            Currency::zero(),
        ),
        PlayerAction::new(
            "train_dex",
            "Practice juggling",
            "juggled",
            "practiced juggling",
            PlayerActionType::Train,
            CharacterAttributeProgressFactor::from_dexterity(1.0),
            Currency::zero(),
        ),
        PlayerAction::new(
            "train_int",
            "Study logic",
            "studying logic",
            "studied logic",
            PlayerActionType::Train,
            CharacterAttributeProgressFactor::from_intelligence(1.0),
            Currency::zero(),
        ),
        PlayerAction::new(
            "train_wis",
            "Read",
            "reading",
            "read",
            PlayerActionType::Train,
            CharacterAttributeProgressFactor::from_wisdom(1.0),
            Currency::zero(),
        ),
    ]
}*/

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerActions {
    actions: Vec<CompiledPlayerAction>,
    inactive_actions: HashSet<PlayerActionId>,
    active_actions: HashSet<PlayerActionId>,
    deactivated_actions: HashSet<PlayerActionId>,
    actions_by_name: HashMap<String, PlayerActionId>,
    in_progress: Option<PlayerActionInProgress>,
    pub selected_action: PlayerActionId,
}

#[derive(Clone, Debug, Serialize, Deserialize, Sequence, Eq, PartialEq)]
pub enum PlayerActionType {
    Wait,
    Sleep,
    Tavern,
    Train,
    Work,
    Explore,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerAction {
    pub id_str: String,
    pub name: String,
    pub verb_progressive: String,
    pub verb_simple_past: String,
    pub action_type: PlayerActionType,
    pub duration: GameTime,
    pub attribute_progress_factor: CharacterAttributeProgressFactor,
    pub currency_reward: Currency,
    pub activation_condition: String,
    pub deactivation_condition: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompiledPlayerAction {
    pub id: PlayerActionId,
    pub id_str: String,
    pub state: PlayerActionState,
    pub name: String,
    pub verb_progressive: String,
    pub verb_simple_past: String,
    pub action_type: PlayerActionType,
    pub duration: GameTime,
    pub attribute_progress_factor: CharacterAttributeProgressFactor,
    pub currency_reward: Currency,
    pub activation_condition: TriggerHandle,
    pub deactivation_condition: TriggerHandle,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PlayerActionState {
    Inactive,
    Active {
        activation_time: GameTime,
    },
    Deactivated {
        activation_time: GameTime,
        deactivation_time: GameTime,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerActionInProgress {
    pub verb_progressive: String,
    pub verb_simple_past: String,
    pub source: PlayerActionInProgressSource,
    pub kind: PlayerActionInProgressKind,
    pub start: GameTime,
    pub end: GameTime,
    pub attribute_progress: CharacterAttributeProgress,
    pub currency_reward: Currency,
    pub success: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PlayerActionInProgressSource {
    Action(PlayerActionId),
    Exploration(ExplorationEventId),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PlayerActionInProgressKind {
    Combat(MonsterId),
    None,
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, Default, Eq, PartialEq, Hash, Ord, PartialOrd,
)]
pub struct PlayerActionId(usize);

impl PlayerActions {
    pub fn new(actions: Vec<CompiledPlayerAction>) -> Result<Self, ParserError> {
        let action_wait = actions
            .get(ACTION_WAIT.0)
            .ok_or_else(|| ParserError::without_coordinates(ParserErrorKind::MissingActionWait))?;
        let action_sleep = actions
            .get(ACTION_SLEEP.0)
            .ok_or_else(|| ParserError::without_coordinates(ParserErrorKind::MissingActionSleep))?;
        let action_tavern = actions.get(ACTION_TAVERN.0).ok_or_else(|| {
            ParserError::without_coordinates(ParserErrorKind::MissingActionTavern)
        })?;
        let action_explore = actions.get(ACTION_EXPLORE.0).ok_or_else(|| {
            ParserError::without_coordinates(ParserErrorKind::MissingActionExplore)
        })?;
        if action_wait.action_type != PlayerActionType::Wait {
            return Err(ParserError::without_coordinates(
                ParserErrorKind::MissingActionWait,
            ));
        }
        if action_sleep.action_type != PlayerActionType::Sleep {
            return Err(ParserError::without_coordinates(
                ParserErrorKind::MissingActionSleep,
            ));
        }
        if action_tavern.action_type != PlayerActionType::Tavern {
            return Err(ParserError::without_coordinates(
                ParserErrorKind::MissingActionTavern,
            ));
        }
        if action_explore.action_type != PlayerActionType::Explore {
            return Err(ParserError::without_coordinates(
                ParserErrorKind::MissingActionExplore,
            ));
        }

        let inactive_actions = actions.iter().map(|action| action.id).collect();
        let actions_by_name = actions
            .iter()
            .map(|action| (action.name.clone(), action.id))
            .collect();
        Ok(Self {
            actions,
            inactive_actions,
            active_actions: Default::default(),
            deactivated_actions: Default::default(),
            actions_by_name,
            in_progress: None,
            selected_action: ACTION_WAIT,
        })
    }

    pub fn action(&self, action_id: PlayerActionId) -> &CompiledPlayerAction {
        &self.actions[action_id.0]
    }

    pub fn action_mut(&mut self, action_id: PlayerActionId) -> &mut CompiledPlayerAction {
        &mut self.actions[action_id.0]
    }

    pub fn has_action_in_progress(&self) -> bool {
        self.in_progress.is_some()
    }

    pub fn in_progress(&self) -> &PlayerActionInProgress {
        self.in_progress.as_ref().unwrap()
    }

    pub fn set_in_progress(&mut self, in_progress: PlayerActionInProgress) {
        self.in_progress = Some(in_progress);
    }

    pub fn list_choosable(&self) -> impl '_ + Iterator<Item = PlayerActionId> {
        self.active_actions
            .iter()
            .copied()
            .filter(|action_id| action_id != &ACTION_SLEEP)
    }

    pub fn activate_action(
        &mut self,
        action_id: PlayerActionId,
        time: GameTime,
    ) -> impl Iterator<Item = CompiledGameEvent> {
        let action = self.action_mut(action_id);
        assert!(action.state.is_inactive());
        action.state = PlayerActionState::Active {
            activation_time: time,
        };
        assert!(self.inactive_actions.remove(&action_id));
        assert!(self.active_actions.insert(action_id));
        iter::empty()
    }

    pub fn deactivate_action(
        &mut self,
        action_id: PlayerActionId,
        time: GameTime,
    ) -> impl Iterator<Item = CompiledGameEvent> {
        let action = self.action_mut(action_id);
        assert!(action.state.is_active());
        match action.state {
            PlayerActionState::Active { activation_time } => {
                action.state = PlayerActionState::Deactivated {
                    activation_time,
                    deactivation_time: time,
                };
                assert!(self.active_actions.remove(&action_id));
                assert!(self.deactivated_actions.insert(action_id));
                if self.selected_action == action_id {
                    self.selected_action = ACTION_WAIT;
                }
            }
            _ => unreachable!(),
        }
        iter::empty()
    }
}

impl PlayerAction {
    pub fn compile(self, id_maps: &IdMaps) -> CompiledPlayerAction {
        CompiledPlayerAction {
            id: *id_maps
                .actions
                .get(&self.id_str)
                .unwrap_or_else(|| panic!("Did not find action {:?} in id_map", self.id_str)),
            id_str: self.id_str,
            state: PlayerActionState::Inactive,
            name: self.name,
            verb_progressive: self.verb_progressive,
            verb_simple_past: self.verb_simple_past,
            action_type: self.action_type,
            duration: self.duration,
            attribute_progress_factor: self.attribute_progress_factor,
            currency_reward: self.currency_reward,
            activation_condition: *id_maps.triggers.get(&self.activation_condition).unwrap(),
            deactivation_condition: *id_maps.triggers.get(&self.deactivation_condition).unwrap(),
        }
    }
}

impl CompiledPlayerAction {
    pub fn spawn(&self, start_time: GameTime) -> PlayerActionInProgress {
        PlayerActionInProgress {
            verb_progressive: self.verb_progressive.clone(),
            verb_simple_past: self.verb_simple_past.clone(),
            source: PlayerActionInProgressSource::Action(self.id),
            kind: PlayerActionInProgressKind::None,
            start: start_time,
            end: start_time + self.duration,
            attribute_progress: self.attribute_progress_factor.into_progress(self.duration),
            currency_reward: self.currency_reward,
            success: true,
        }
    }
}

impl PlayerActionInProgressSource {
    pub fn action_id(&self) -> PlayerActionId {
        match self {
            PlayerActionInProgressSource::Action(action_id) => *action_id,
            PlayerActionInProgressSource::Exploration(_) => ACTION_EXPLORE,
        }
    }
}

impl ToString for CompiledPlayerAction {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

#[allow(dead_code)]
impl PlayerActionState {
    pub fn is_inactive(&self) -> bool {
        matches!(self, PlayerActionState::Inactive)
    }

    pub fn is_active(&self) -> bool {
        matches!(self, PlayerActionState::Active { .. })
    }

    pub fn is_deactivated(&self) -> bool {
        matches!(self, PlayerActionState::Deactivated { .. })
    }
}

impl PlayerActionInProgress {
    pub fn length(&self) -> GameTime {
        self.end - self.start
    }
}

impl From<usize> for PlayerActionId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl FromStr for PlayerActionType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "WAIT" => PlayerActionType::Wait,
            "SLEEP" => PlayerActionType::Sleep,
            "TAVERN" => PlayerActionType::Tavern,
            "TRAIN" => PlayerActionType::Train,
            "WORK" => PlayerActionType::Work,
            "EXPLORE" => PlayerActionType::Explore,
            _ => return Err(()),
        })
    }
}
