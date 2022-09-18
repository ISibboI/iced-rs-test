use crate::game_state::time::GameTime;
use crate::game_template::IdMaps;
use event_trigger_action_system::TriggerHandle;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Monster {
    pub id_str: String,
    pub name: String,
    pub hitpoints: f64,
    pub activation_condition: String,
    pub deactivation_condition: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompiledMonster {
    pub id: MonsterId,
    pub id_str: String,
    pub state: MonsterState,
    pub name: String,
    pub hitpoints: f64,
    pub activation: TriggerHandle,
    pub deactivation: TriggerHandle,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MonsterState {
    Inactive,
    Active {
        activation_time: GameTime,
    },
    Deactivated {
        activation_time: GameTime,
        deactivation_time: GameTime,
    },
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, Default, Eq, PartialEq, Hash, Ord, PartialOrd,
)]
pub struct MonsterId(pub usize);

impl Monster {
    pub fn compile(self, id_maps: &IdMaps) -> CompiledMonster {
        CompiledMonster {
            id: *id_maps.monsters.get(&self.id_str).unwrap(),
            id_str: self.id_str,
            state: MonsterState::Inactive,
            name: self.name,
            hitpoints: self.hitpoints,
            activation: *id_maps.triggers.get(&self.activation_condition).unwrap(),
            deactivation: *id_maps.triggers.get(&self.deactivation_condition).unwrap(),
        }
    }
}

#[allow(dead_code)]
impl MonsterState {
    pub fn is_inactive(&self) -> bool {
        matches!(self, MonsterState::Inactive)
    }

    pub fn is_active(&self) -> bool {
        matches!(self, MonsterState::Active { .. })
    }

    pub fn is_deactivated(&self) -> bool {
        matches!(self, MonsterState::Deactivated { .. })
    }
}

impl From<usize> for MonsterId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
