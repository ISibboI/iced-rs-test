use crate::game_state::combat::{SpawnedMonster, MONSTERS, MONSTER_MODIFIERS};
use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

lazy_static! {
    pub static ref LOCATIONS: HashMap<String, Location> = init_locations();
}

fn init_locations() -> HashMap<String, Location> {
    [
        Location::new(LOCATION_VILLAGE, [("rat", 1.0)], [("normal", 1.0)]),
        Location::new(
            "Forest",
            [("rat", 0.1), ("hare", 1.0), ("deer", 1.0)],
            [("normal", 1.0), ("young", 0.1), ("old", 0.1)],
        ),
        Location::new(
            "Deep forest",
            [
                ("rat", 0.01),
                ("hare", 0.1),
                ("deer", 1.0),
                ("fox", 1.0),
                ("boar", 1.0),
                ("wolf", 0.1),
            ],
            [
                ("normal", 1.0),
                ("young", 0.1),
                ("old", 0.1),
                ("strong", 0.1),
                ("weak", 0.1),
            ],
        ),
    ]
    .into_iter()
    .map(|location| (location.name.clone(), location))
    .collect()
}

pub static LOCATION_VILLAGE: &str = "Village";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Location {
    pub name: String,
    pub monsters: Vec<WeightedMonster>,
    pub monster_modifiers: Vec<WeightedMonsterModifier>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WeightedMonster {
    monster: String,
    weight: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WeightedMonsterModifier {
    monster_modifier: String,
    weight: f64,
}

impl Location {
    pub fn new<MonsterName: ToString, MonsterModifierName: ToString>(
        name: impl ToString,
        monsters: impl IntoIterator<Item = (MonsterName, f64)>,
        monster_modifiers: impl IntoIterator<Item = (MonsterModifierName, f64)>,
    ) -> Self {
        Self {
            name: name.to_string(),
            monsters: monsters.into_iter().map(Into::into).collect(),
            monster_modifiers: monster_modifiers.into_iter().map(Into::into).collect(),
        }
    }

    pub fn spawn(&self) -> SpawnedMonster {
        let mut rng = thread_rng();
        let monster = MONSTERS
            .get(
                &self
                    .monsters
                    .choose_weighted(&mut rng, |weighted_monster| weighted_monster.weight)
                    .unwrap()
                    .monster,
            )
            .unwrap()
            .clone();
        let modifier = MONSTER_MODIFIERS
            .get(
                &self
                    .monster_modifiers
                    .choose_weighted(&mut rng, |weighted_monster_modifier| {
                        weighted_monster_modifier.weight
                    })
                    .unwrap()
                    .monster_modifier,
            )
            .unwrap()
            .clone();
        monster.spawn(modifier)
    }
}

impl<Name: ToString> From<(Name, f64)> for WeightedMonster {
    fn from((monster, weight): (Name, f64)) -> Self {
        Self {
            monster: monster.to_string(),
            weight,
        }
    }
}

impl<Name: ToString> From<(Name, f64)> for WeightedMonsterModifier {
    fn from((monster_modifier, weight): (Name, f64)) -> Self {
        Self {
            monster_modifier: monster_modifier.to_string(),
            weight,
        }
    }
}
