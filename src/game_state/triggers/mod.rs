use crate::game_template::IdMaps;
use event_trigger_action_system::{Trigger, TriggerAction, TriggerEvent, TriggerIdentifier};
use serde::{Deserialize, Serialize};

pub fn init_triggers() -> Vec<Trigger<GameEvent, GameAction>> {
    vec![]
}

#[derive(Debug)]
pub enum GameEvent {}

#[derive(Debug)]
pub enum GameAction {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompiledGameEvent {
    Action(CompiledGameAction),
}

#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub enum CompiledGameEventIdentifier {}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum CompiledGameAction {}

impl GameEvent {
    pub fn compile(self, id_maps: &IdMaps) -> CompiledGameEvent {
        todo!()
    }
}

impl GameAction {
    pub fn compile(self, id_maps: &IdMaps) -> CompiledGameAction {
        todo!()
    }
}

impl TriggerEvent for CompiledGameEvent {
    type Action = CompiledGameAction;
    type Identifier = CompiledGameEventIdentifier;

    fn identifier(&self) -> Self::Identifier {
        todo!()
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
