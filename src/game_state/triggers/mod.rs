use crate::game_state::currency::Currency;
use crate::game_state::player_actions::PlayerActionId;
use crate::game_state::story::quests::QuestId;
use crate::game_state::world::events::ExplorationEventId;
use crate::game_state::world::locations::LocationId;
use crate::game_state::world::monsters::MonsterId;
use crate::game_template::IdMaps;
use event_trigger_action_system::{TriggerAction, TriggerEvent, TriggerIdentifier};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum GameEvent {
    Action(GameAction),
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
    ExplorationStarted { id: String },
    ExplorationCompleted { id: String },
    MonsterKilled { id: String },
    MonsterFailed { id: String },
}

#[derive(Debug, Clone)]
pub enum GameAction {
    ActivateQuest { id: String },
    CompleteQuest { id: String },
    FailQuest { id: String },
    ActivateAction { id: String },
    DeactivateAction { id: String },
    ActivateLocation { id: String },
    DeactivateLocation { id: String },
    ActivateExplorationEvent { id: String },
    DeactivateExplorationEvent { id: String },
    ActivateMonster { id: String },
    DeactivateMonster { id: String },
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
    ExplorationStarted { id: LocationId },
    ExplorationCompleted { id: LocationId },
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
    ExplorationStarted { id: LocationId },
    ExplorationCompleted { id: LocationId },
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
    ActivateLocation { id: LocationId },
    DeactivateLocation { id: LocationId },
    ActivateExplorationEvent { id: ExplorationEventId },
    DeactivateExplorationEvent { id: ExplorationEventId },
    ActivateMonster { id: MonsterId },
    DeactivateMonster { id: MonsterId },
}

impl GameEvent {
    pub fn compile(self, id_maps: &IdMaps) -> CompiledGameEvent {
        match self {
            GameEvent::Action(action) => CompiledGameEvent::Action(action.compile(id_maps)),
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
            GameEvent::ExplorationStarted { id } => CompiledGameEvent::ExplorationStarted {
                id: *id_maps.locations.get(&id).unwrap(),
            },
            GameEvent::ExplorationCompleted { id } => CompiledGameEvent::ExplorationCompleted {
                id: *id_maps.locations.get(&id).unwrap(),
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
            GameAction::ActivateLocation { id } => CompiledGameAction::ActivateLocation {
                id: *id_maps.locations.get(&id).unwrap(),
            },
            GameAction::DeactivateLocation { id } => CompiledGameAction::DeactivateLocation {
                id: *id_maps.locations.get(&id).unwrap(),
            },
            GameAction::ActivateExplorationEvent { id } => {
                CompiledGameAction::ActivateExplorationEvent {
                    id: *id_maps.exploration_events.get(&id).unwrap(),
                }
            }
            GameAction::DeactivateExplorationEvent { id } => {
                CompiledGameAction::DeactivateExplorationEvent {
                    id: *id_maps.exploration_events.get(&id).unwrap(),
                }
            }
            GameAction::ActivateMonster { id } => CompiledGameAction::ActivateMonster {
                id: *id_maps.monsters.get(&id).unwrap(),
            },
            GameAction::DeactivateMonster { id } => CompiledGameAction::DeactivateMonster {
                id: *id_maps.monsters.get(&id).unwrap(),
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
                CompiledGameEventIdentifier::ActionStarted { id: *id }
            }
            CompiledGameEvent::ActionCompleted { id } => {
                CompiledGameEventIdentifier::ActionCompleted { id: *id }
            }
            CompiledGameEvent::ExplorationStarted { id } => {
                CompiledGameEventIdentifier::ExplorationStarted { id: *id }
            }
            CompiledGameEvent::ExplorationCompleted { id } => {
                CompiledGameEventIdentifier::ExplorationCompleted { id: *id }
            }
            CompiledGameEvent::MonsterKilled { id } => {
                CompiledGameEventIdentifier::MonsterKilled { id: *id }
            }
            CompiledGameEvent::MonsterFailed { id } => {
                CompiledGameEventIdentifier::MonsterFailed { id: *id }
            }
        }
    }

    fn value_geq(&self, other: &Self) -> Option<bool> {
        match (self, other) {
            (
                CompiledGameEvent::CurrencyChanged { value: value_lhs },
                CompiledGameEvent::CurrencyChanged { value: value_rhs },
            ) => Some(value_lhs >= value_rhs),
            (
                CompiledGameEvent::PlayerLevelChanged { value: value_lhs },
                CompiledGameEvent::PlayerLevelChanged { value: value_rhs },
            ) => Some(value_lhs >= value_rhs),
            (
                CompiledGameEvent::PlayerStrengthChanged { value: value_lhs },
                CompiledGameEvent::PlayerStrengthChanged { value: value_rhs },
            ) => Some(value_lhs >= value_rhs),
            (
                CompiledGameEvent::PlayerStaminaChanged { value: value_lhs },
                CompiledGameEvent::PlayerStaminaChanged { value: value_rhs },
            ) => Some(value_lhs >= value_rhs),
            (
                CompiledGameEvent::PlayerDexterityChanged { value: value_lhs },
                CompiledGameEvent::PlayerDexterityChanged { value: value_rhs },
            ) => Some(value_lhs >= value_rhs),
            (
                CompiledGameEvent::PlayerIntelligenceChanged { value: value_lhs },
                CompiledGameEvent::PlayerIntelligenceChanged { value: value_rhs },
            ) => Some(value_lhs >= value_rhs),
            (
                CompiledGameEvent::PlayerWisdomChanged { value: value_lhs },
                CompiledGameEvent::PlayerWisdomChanged { value: value_rhs },
            ) => Some(value_lhs >= value_rhs),
            (
                CompiledGameEvent::PlayerCharismaChanged { value: value_lhs },
                CompiledGameEvent::PlayerCharismaChanged { value: value_rhs },
            ) => Some(value_lhs >= value_rhs),
            _ => None,
        }
    }

    fn value_geq_progress(&self, other: &Self) -> Option<f64> {
        match (self, other) {
            (
                CompiledGameEvent::CurrencyChanged { value: value_lhs },
                CompiledGameEvent::CurrencyChanged { value: value_rhs },
            ) => Some(value_lhs.copper() as f64 / value_rhs.copper() as f64),
            (
                CompiledGameEvent::PlayerLevelChanged { value: value_lhs },
                CompiledGameEvent::PlayerLevelChanged { value: value_rhs },
            ) => Some(*value_lhs as f64 / *value_rhs as f64),
            (
                CompiledGameEvent::PlayerStrengthChanged { value: value_lhs },
                CompiledGameEvent::PlayerStrengthChanged { value: value_rhs },
            ) => Some(*value_lhs as f64 / *value_rhs as f64),
            (
                CompiledGameEvent::PlayerStaminaChanged { value: value_lhs },
                CompiledGameEvent::PlayerStaminaChanged { value: value_rhs },
            ) => Some(*value_lhs as f64 / *value_rhs as f64),
            (
                CompiledGameEvent::PlayerDexterityChanged { value: value_lhs },
                CompiledGameEvent::PlayerDexterityChanged { value: value_rhs },
            ) => Some(*value_lhs as f64 / *value_rhs as f64),
            (
                CompiledGameEvent::PlayerIntelligenceChanged { value: value_lhs },
                CompiledGameEvent::PlayerIntelligenceChanged { value: value_rhs },
            ) => Some(*value_lhs as f64 / *value_rhs as f64),
            (
                CompiledGameEvent::PlayerWisdomChanged { value: value_lhs },
                CompiledGameEvent::PlayerWisdomChanged { value: value_rhs },
            ) => Some(*value_lhs as f64 / *value_rhs as f64),
            (
                CompiledGameEvent::PlayerCharismaChanged { value: value_lhs },
                CompiledGameEvent::PlayerCharismaChanged { value: value_rhs },
            ) => Some(*value_lhs as f64 / *value_rhs as f64),
            _ => None,
        }
    }
}

impl TriggerAction for CompiledGameAction {}

impl TriggerIdentifier for CompiledGameEventIdentifier {}

impl From<CompiledGameAction> for CompiledGameEvent {
    fn from(action: CompiledGameAction) -> Self {
        Self::Action(action)
    }
}
