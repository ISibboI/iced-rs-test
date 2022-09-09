use crate::game_template::IdMaps;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Monster {
    pub id_str: String,
    pub name: String,
    pub hitpoints: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompiledMonster {
    pub id: MonsterId,
    pub id_str: String,
    pub name: String,
    pub hitpoints: f64,
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
            name: self.name,
            hitpoints: self.hitpoints,
        }
    }
}

impl CompiledMonster {
    pub fn to_lowercase_string(&self) -> String {
        self.name.to_lowercase()
    }
}

impl From<usize> for MonsterId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
