use crate::game_state::currency::Currency;
use crate::game_state::player_actions::PlayerActionId;
use crate::game_state::story::quests::QuestId;
use crate::game_state::world::monsters::MonsterId;
use crate::game_template::IdMaps;
use event_trigger_action_system::{Trigger, TriggerAction, TriggerEvent, TriggerIdentifier};
use serde::{Deserialize, Serialize};

pub fn init_triggers() -> Vec<Trigger<GameEvent, GameAction>> {
    vec![]
}

#[derive(Debug)]
pub enum GameEvent {
    CurrencyChanged { value: Currency },
    PlayerLevelChanged { value: u64 },
    PlayerStrengthChanged { value: u64 },
    PlayerStaminaChanged { value: u64 },
    PlayerDexterityChanged { value: u64 },
    PlayerIntelligenceChanged { value: u64 },
    PlayerWisdomChanged { value: u64 },
    PlayerCharismaChanged { value: u64 },
    ActionStarted { id: String },
    ActionCompleted { id: String },
    MonsterKilled { id: String },
    MonsterFailed { id: String },
}

#[derive(Debug)]
pub enum GameAction {
    ActivateQuest { id: String },
    CompleteQuest { id: String },
    FailQuest { id: String },
    ActivateAction { id: String },
    DeactivateAction { id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompiledGameEvent {
    Action(CompiledGameAction),
    CurrencyChanged { value: Currency },
    PlayerLevelChanged { value: u64 },
    PlayerStrengthChanged { value: u64 },
    PlayerStaminaChanged { value: u64 },
    PlayerDexterityChanged { value: u64 },
    PlayerIntelligenceChanged { value: u64 },
    PlayerWisdomChanged { value: u64 },
    PlayerCharismaChanged { value: u64 },
    ActionStarted { id: PlayerActionId },
    ActionCompleted { id: PlayerActionId },
    MonsterKilled { id: MonsterId },
    MonsterFailed { id: MonsterId },
}

#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub enum CompiledGameEventIdentifier {
    Action(CompiledGameAction),
    CurrencyChanged,
    PlayerLevelChanged,
    PlayerStrengthChanged,
    PlayerStaminaChanged,
    PlayerDexterityChanged,
    PlayerIntelligenceChanged,
    PlayerWisdomChanged,
    PlayerCharismaChanged,
    ActionStarted { id: PlayerActionId },
    ActionCompleted { id: PlayerActionId },
    MonsterKilled { id: MonsterId },
    MonsterFailed { id: MonsterId },
}

#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub enum CompiledGameAction {
    ActivateQuest { id: QuestId },
    CompleteQuest { id: QuestId },
    FailQuest { id: QuestId },
    ActivateAction { id: PlayerActionId },
    DeactivateAction { id: PlayerActionId },
}

impl GameEvent {
    pub fn compile(self, id_maps: &IdMaps) -> CompiledGameEvent {
        match self {
            GameEvent::CurrencyChanged { value } => CompiledGameEvent::CurrencyChanged { value },
            GameEvent::PlayerLevelChanged { value } => {
                CompiledGameEvent::PlayerLevelChanged { value }
            }
            GameEvent::PlayerStrengthChanged { value } => {
                CompiledGameEvent::PlayerStrengthChanged { value }
            }
            GameEvent::PlayerStaminaChanged { value } => {
                CompiledGameEvent::PlayerStaminaChanged { value }
            }
            GameEvent::PlayerDexterityChanged { value } => {
                CompiledGameEvent::PlayerDexterityChanged { value }
            }
            GameEvent::PlayerIntelligenceChanged { value } => {
                CompiledGameEvent::PlayerIntelligenceChanged { value }
            }
            GameEvent::PlayerWisdomChanged { value } => {
                CompiledGameEvent::PlayerWisdomChanged { value }
            }
            GameEvent::PlayerCharismaChanged { value } => {
                CompiledGameEvent::PlayerCharismaChanged { value }
            }
            GameEvent::ActionStarted { id } => CompiledGameEvent::ActionStarted {
                id: *id_maps.actions.get(&id).unwrap(),
            },
            GameEvent::ActionCompleted { id } => CompiledGameEvent::ActionCompleted {
                id: *id_maps.actions.get(&id).unwrap(),
            },
            GameEvent::MonsterKilled { id } => CompiledGameEvent::MonsterKilled {
                id: *id_maps.monsters.get(&id).unwrap(),
            },
            GameEvent::MonsterFailed { id } => CompiledGameEvent::MonsterFailed {
                id: *id_maps.monsters.get(&id).unwrap(),
            },
        }
    }
}

impl GameAction {
    pub fn compile(self, id_maps: &IdMaps) -> CompiledGameAction {
        match self {
            GameAction::ActivateQuest { id } => CompiledGameAction::ActivateQuest {
                id: *id_maps.quests.get(&id).unwrap(),
            },
            GameAction::CompleteQuest { id } => CompiledGameAction::CompleteQuest {
                id: *id_maps.quests.get(&id).unwrap(),
            },
            GameAction::FailQuest { id } => CompiledGameAction::FailQuest {
                id: *id_maps.quests.get(&id).unwrap(),
            },
            GameAction::ActivateAction { id } => CompiledGameAction::ActivateAction {
                id: *id_maps.actions.get(&id).unwrap(),
            },
            GameAction::DeactivateAction { id } => CompiledGameAction::DeactivateAction {
                id: *id_maps.actions.get(&id).unwrap(),
            },
        }
    }
}

impl TriggerEvent for CompiledGameEvent {
    type Action = CompiledGameAction;
    type Identifier = CompiledGameEventIdentifier;

    fn identifier(&self) -> Self::Identifier {
        match self {
            CompiledGameEvent::Action(action) => {
                CompiledGameEventIdentifier::Action(action.clone())
            }
            CompiledGameEvent::CurrencyChanged { .. } => {
                CompiledGameEventIdentifier::CurrencyChanged
            }
            CompiledGameEvent::PlayerLevelChanged { .. } => {
                CompiledGameEventIdentifier::PlayerLevelChanged
            }
            CompiledGameEvent::PlayerStrengthChanged { .. } => {
                CompiledGameEventIdentifier::PlayerStrengthChanged
            }
            CompiledGameEvent::PlayerStaminaChanged { .. } => {
                CompiledGameEventIdentifier::PlayerStaminaChanged
            }
            CompiledGameEvent::PlayerDexterityChanged { .. } => {
                CompiledGameEventIdentifier::PlayerDexterityChanged
            }
            CompiledGameEvent::PlayerIntelligenceChanged { .. } => {
                CompiledGameEventIdentifier::PlayerIntelligenceChanged
            }
            CompiledGameEvent::PlayerWisdomChanged { .. } => {
                CompiledGameEventIdentifier::PlayerWisdomChanged
            }
            CompiledGameEvent::PlayerCharismaChanged { .. } => {
                CompiledGameEventIdentifier::PlayerCharismaChanged
            }
            CompiledGameEvent::ActionStarted { id } => {
                CompiledGameEventIdentifier::ActionStarted { id: id.clone() }
            }
            CompiledGameEvent::ActionCompleted { id } => {
                CompiledGameEventIdentifier::ActionCompleted { id: id.clone() }
            }
            CompiledGameEvent::MonsterKilled { id } => {
                CompiledGameEventIdentifier::MonsterKilled { id: id.clone() }
            }
            CompiledGameEvent::MonsterFailed { id } => {
                CompiledGameEventIdentifier::MonsterFailed { id: id.clone() }
            }
        }
    }

    fn value_geq(&self, other: &Self) -> Option<bool> {
        todo!()
    }

    fn value_geq_progress(&self, other: &Self) -> Option<f64> {
        todo!()
    }
}

impl TriggerAction for CompiledGameAction {}

impl TriggerIdentifier for CompiledGameEventIdentifier {}

impl From<CompiledGameAction> for CompiledGameEvent {
    fn from(action: CompiledGameAction) -> Self {
        Self::Action(action)
    }
}
