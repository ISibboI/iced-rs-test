use crate::game_state::character::{CharacterAttributeProgress, CharacterAttributeProgressFactor};
use crate::game_state::combat::SpawnedMonster;
use crate::game_state::currency::Currency;
use crate::game_state::time::GameTime;
use crate::game_state::triggers::CompiledGameEvent;
use crate::game_template::IdMaps;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::iter;
use std::ops::Deref;

pub static ACTION_WAIT: PlayerActionId = PlayerActionId(0);
pub static ACTION_SLEEP: PlayerActionId = PlayerActionId(1);
pub static ACTION_TAVERN: PlayerActionId = PlayerActionId(2);
pub static ACTION_FIGHT_MONSTERS: PlayerActionId = PlayerActionId(3);

pub fn init_actions() -> Vec<PlayerAction> {
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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerActions {
    actions: Vec<CompiledPlayerAction>,
    inactive_actions: HashSet<PlayerActionId>,
    active_actions: HashSet<PlayerActionId>,
    deactivated_actions: HashSet<PlayerActionId>,
    actions_by_name: HashMap<String, PlayerActionId>,
    in_progress: Option<ActionInProgress>,
    pub selected_action: PlayerActionId,
}

#[derive(Clone, Debug, Serialize, Deserialize, Sequence, Eq, PartialEq)]
pub enum PlayerActionType {
    Rest,
    Train,
    Work,
    Combat,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerAction {
    pub id_str: String,
    pub name: String,
    pub verb_progressive: String,
    pub verb_simple_past: String,
    pub action_type: PlayerActionType,
    pub attribute_progress_factor: CharacterAttributeProgressFactor,
    pub currency_gain: Currency,
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
    pub attribute_progress_factor: CharacterAttributeProgressFactor,
    pub currency_gain: Currency,
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
pub struct ActionInProgress {
    pub action: PlayerActionId,
    pub start: GameTime,
    pub end: GameTime,
    pub attribute_progress: CharacterAttributeProgress,
    pub monster: Option<SpawnedMonster>,
    pub currency_reward: Currency,
    pub success: bool,
}

#[derive(Clone, Debug)]
pub struct DerefActionInProgress<'a> {
    pub action: &'a CompiledPlayerAction,
    deref: &'a ActionInProgress,
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, Default, Eq, PartialEq, Hash, Ord, PartialOrd,
)]
pub struct PlayerActionId(usize);

impl PlayerActions {
    pub fn new(actions: Vec<CompiledPlayerAction>) -> Self {
        let inactive_actions = actions.iter().map(|action| action.id).collect();
        let actions_by_name = actions
            .iter()
            .map(|action| (action.name.clone(), action.id))
            .collect();
        Self {
            actions,
            inactive_actions,
            active_actions: Default::default(),
            deactivated_actions: Default::default(),
            actions_by_name,
            in_progress: None,
            selected_action: ACTION_WAIT,
        }
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

    pub fn in_progress(&self) -> DerefActionInProgress {
        self.in_progress.as_ref().unwrap().resolve(self)
    }

    pub fn set_in_progress(&mut self, in_progress: ActionInProgress) {
        self.in_progress = Some(in_progress);
    }

    pub fn list_choosable(&self) -> impl '_ + Iterator<Item = PlayerActionId> {
        self.actions.iter().filter_map(|action| {
            if action.id != ACTION_SLEEP {
                Some(action.id)
            } else {
                None
            }
        })
    }

    #[deprecated]
    pub fn actions_by_name(&self) -> &HashMap<String, PlayerActionId> {
        &self.actions_by_name
    }

    pub fn select_action(&mut self, action: &str) {
        self.selected_action = *self.actions_by_name.get(action).unwrap();
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
            }
            _ => unreachable!(),
        }
        iter::empty()
    }
}

impl PlayerAction {
    fn new(
        id_str: impl ToString,
        name: impl ToString,
        verb_progressive: impl ToString,
        verb_simple_past: impl ToString,
        action_type: PlayerActionType,
        attribute_progress_factor: CharacterAttributeProgressFactor,
        currency_gain: Currency,
    ) -> Self {
        Self {
            id_str: id_str.to_string(),
            name: name.to_string(),
            verb_progressive: verb_progressive.to_string(),
            verb_simple_past: verb_simple_past.to_string(),
            action_type,
            attribute_progress_factor,
            currency_gain,
        }
    }

    pub fn compile(self, id_maps: &IdMaps) -> CompiledPlayerAction {
        CompiledPlayerAction {
            id: *id_maps.actions.get(&self.name).unwrap(),
            id_str: self.id_str,
            state: PlayerActionState::Inactive,
            name: self.name,
            verb_progressive: self.verb_progressive,
            verb_simple_past: self.verb_simple_past,
            action_type: self.action_type,
            attribute_progress_factor: self.attribute_progress_factor,
            currency_gain: self.currency_gain,
        }
    }
}

impl ToString for CompiledPlayerAction {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl PlayerActionState {
    pub fn is_inactive(&self) -> bool {
        match self {
            PlayerActionState::Inactive => true,
            _ => false,
        }
    }

    pub fn is_active(&self) -> bool {
        match self {
            PlayerActionState::Active { .. } => true,
            _ => false,
        }
    }

    pub fn is_deactivated(&self) -> bool {
        match self {
            PlayerActionState::Deactivated { .. } => true,
            _ => false,
        }
    }
}

impl ActionInProgress {
    pub fn resolve<'result>(
        &'result self,
        actions: &'result PlayerActions,
    ) -> DerefActionInProgress<'result> {
        DerefActionInProgress {
            action: actions.action(self.action),
            deref: self,
        }
    }

    pub fn length(&self) -> GameTime {
        self.end - self.start
    }
}

impl<'a> Deref for DerefActionInProgress<'a> {
    type Target = ActionInProgress;

    fn deref(&self) -> &Self::Target {
        self.deref
    }
}

impl From<usize> for PlayerActionId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
