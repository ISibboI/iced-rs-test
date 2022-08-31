use crate::game_state::character::{CharacterAttributeProgress, CharacterAttributeProgressFactor};
use crate::game_state::combat::SpawnedMonster;
use crate::game_state::currency::Currency;
use crate::game_state::time::GameTime;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;

pub static ACTION_WAIT: ActionId = ActionId(0);
pub static ACTION_SLEEP: ActionId = ActionId(1);
pub static ACTION_TAVERN: ActionId = ActionId(2);
pub static ACTION_FIGHT_MONSTERS: ActionId = ActionId(3);

impl Actions {
    pub fn new() -> Self {
        let actions = vec![
            Action::new(
                "Wait",
                "waiting",
                "waited",
                ActionType::Rest,
                CharacterAttributeProgressFactor::zero(),
                Currency::zero(),
            ),
            Action::new(
                "Sleep",
                "sleeping",
                "slept",
                ActionType::Rest,
                CharacterAttributeProgressFactor::zero(),
                Currency::zero(),
            ),
            Action::new(
                "Tavern",
                "relaxing in the tavern",
                "relaxed in the tavern",
                ActionType::Rest,
                CharacterAttributeProgressFactor::from_charisma(1.0),
                Currency::from_copper(-10),
            ),
            // most values computed depending on fighting style, monster, etc.
            Action::new(
                "Fight monsters",
                "fighting monsters",
                "fought monsters",
                ActionType::Combat,
                CharacterAttributeProgressFactor::zero(),
                Currency::zero(),
            ),
            Action::new(
                "Lift weights",
                "lifting weights",
                "lifted weights",
                ActionType::Train,
                CharacterAttributeProgressFactor::from_strength(1.0),
                Currency::zero(),
            ),
            Action::new(
                "Jog",
                "jogging",
                "jogged",
                ActionType::Train,
                CharacterAttributeProgressFactor::from_stamina(1.0),
                Currency::zero(),
            ),
            Action::new(
                "Practice juggling",
                "juggled",
                "practiced juggling",
                ActionType::Train,
                CharacterAttributeProgressFactor::from_dexterity(1.0),
                Currency::zero(),
            ),
            Action::new(
                "Study logic",
                "studying logic",
                "studied logic",
                ActionType::Train,
                CharacterAttributeProgressFactor::from_intelligence(1.0),
                Currency::zero(),
            ),
            Action::new(
                "Read",
                "reading",
                "read",
                ActionType::Train,
                CharacterAttributeProgressFactor::from_wisdom(1.0),
                Currency::zero(),
            ),
        ];
        let id_map: HashMap<_, _> = actions
            .iter()
            .enumerate()
            .map(|(id, action)| (action.name.clone(), id.into()))
            .collect();

        assert_eq!(id_map.get("Wait"), Some(&ACTION_WAIT));
        assert_eq!(id_map.get("Sleep"), Some(&ACTION_SLEEP));
        assert_eq!(id_map.get("Tavern"), Some(&ACTION_TAVERN));
        assert_eq!(id_map.get("Fight monsters"), Some(&ACTION_FIGHT_MONSTERS));

        Self {
            actions: actions
                .into_iter()
                .map(|action| action.compile(&id_map))
                .collect(),
            actions_by_name: id_map,
            in_progress: None,
            selected_action: ACTION_WAIT,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Actions {
    actions: Vec<CompiledAction>,
    actions_by_name: HashMap<String, ActionId>,
    in_progress: Option<ActionInProgress>,
    pub selected_action: ActionId,
}

impl Actions {
    pub fn action(&self, action_id: ActionId) -> &CompiledAction {
        &self.actions[action_id.0]
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

    pub fn list_choosable(&self) -> impl '_ + Iterator<Item = ActionId> {
        self.actions.iter().filter_map(|action| {
            if action.id != ACTION_SLEEP {
                Some(action.id)
            } else {
                None
            }
        })
    }

    pub fn actions_by_name(&self) -> &HashMap<String, ActionId> {
        &self.actions_by_name
    }

    pub fn select_action(&mut self, action: &str) {
        self.selected_action = *self.actions_by_name.get(action).unwrap();
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Sequence, Eq, PartialEq)]
pub enum ActionType {
    Rest,
    Train,
    Work,
    Combat,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Action {
    pub name: String,
    pub verb_progressive: String,
    pub verb_simple_past: String,
    pub action_type: ActionType,
    pub attribute_progress_factor: CharacterAttributeProgressFactor,
    pub currency_gain: Currency,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompiledAction {
    pub id: ActionId,
    pub name: String,
    pub verb_progressive: String,
    pub verb_simple_past: String,
    pub action_type: ActionType,
    pub attribute_progress_factor: CharacterAttributeProgressFactor,
    pub currency_gain: Currency,
}

impl Action {
    fn new(
        name: impl ToString,
        verb_progressive: impl ToString,
        verb_simple_past: impl ToString,
        action_type: ActionType,
        attribute_progress_factor: CharacterAttributeProgressFactor,
        currency_gain: Currency,
    ) -> Self {
        Self {
            name: name.to_string(),
            verb_progressive: verb_progressive.to_string(),
            verb_simple_past: verb_simple_past.to_string(),
            action_type,
            attribute_progress_factor,
            currency_gain,
        }
    }

    fn compile(self, id_map: &HashMap<String, ActionId>) -> CompiledAction {
        CompiledAction {
            id: *id_map.get(&self.name).unwrap(),
            name: self.name,
            verb_progressive: self.verb_progressive,
            verb_simple_past: self.verb_simple_past,
            action_type: self.action_type,
            attribute_progress_factor: self.attribute_progress_factor,
            currency_gain: self.currency_gain,
        }
    }
}

impl ToString for CompiledAction {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionInProgress {
    pub action: ActionId,
    pub start: GameTime,
    pub end: GameTime,
    pub attribute_progress: CharacterAttributeProgress,
    pub monster: Option<SpawnedMonster>,
    pub currency_reward: Currency,
    pub success: bool,
}

impl ActionInProgress {
    pub fn resolve<'result>(
        &'result self,
        actions: &'result Actions,
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

#[derive(Clone, Debug)]
pub struct DerefActionInProgress<'a> {
    pub action: &'a CompiledAction,
    deref: &'a ActionInProgress,
}

impl<'a> Deref for DerefActionInProgress<'a> {
    type Target = ActionInProgress;

    fn deref(&self) -> &Self::Target {
        self.deref
    }
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, Default, Eq, PartialEq, Hash, Ord, PartialOrd,
)]
pub struct ActionId(usize);

impl From<usize> for ActionId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
