use enum_iterator::Sequence;
use lazy_static::lazy_static;
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref MONSTERS: Vec<Monster> = vec![
        Monster::new("rat", 0, 1.0, 300.0),
        Monster::new("hare", 2, 1.0, 500.0),
        Monster::new("deer", 4, 1.0, 1_000.0),
        Monster::new("wolf", 8, 1.0, 4_000.0),
        Monster::new("goblin", 10, 1.0, 8_000.0),
        Monster::new("orc", 13, 1.0, 13_000.0),
        Monster::new("dragon", 18, 0.1, 500_000.0),
    ];
    pub static ref MONSTER_MODIFIERS: Vec<MonsterModifier> = vec![
        MonsterModifier::new("normal", 0, 1.0, 1.0),
        MonsterModifier::new("weak", 2, 0.1, 0.5),
        MonsterModifier::new("strong", 4, 0.2, 1.8),
        MonsterModifier::new("young", 0, 0.01, 0.3),
        MonsterModifier::new("old", 5, 0.01, 1.1),
        MonsterModifier::new("veteran", 10, 0.1, 3.0),
        MonsterModifier::new("elite", 20, 0.1, 5.0),
    ];
}

#[derive(Clone, Debug, Serialize, Deserialize, Sequence, Eq, PartialEq)]
pub enum CombatStyle {
    CloseContact,
    Ranged,
    Magic,
}

impl ToString for CombatStyle {
    fn to_string(&self) -> String {
        match self {
            CombatStyle::CloseContact => "Close contact",
            CombatStyle::Ranged => "Ranged",
            CombatStyle::Magic => "Magic",
        }
        .to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Monster {
    pub name: String,
    pub required_level: usize,
    pub base_likelihood: f64,
    pub hitpoints: f64,
}

impl Monster {
    pub fn new(
        name: impl ToString,
        required_level: usize,
        base_likelihood: f64,
        hitpoints: f64,
    ) -> Self {
        Self {
            name: name.to_string(),
            required_level,
            base_likelihood,
            hitpoints,
        }
    }

    pub fn likelihood(&self, level: usize) -> f64 {
        self.base_likelihood / (level.max(self.required_level + 1) - self.required_level) as f64
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MonsterModifier {
    pub name: String,
    pub required_level: usize,
    pub likelihood: f64,
    pub hitpoint_factor: f64,
}

impl MonsterModifier {
    pub fn new(
        name: impl ToString,
        required_level: usize,
        likelihood: f64,
        hitpoint_factor: f64,
    ) -> Self {
        Self {
            name: name.to_string(),
            required_level,
            likelihood,
            hitpoint_factor,
        }
    }

    pub fn choose_random(level: usize) -> Self {
        let modifier = MONSTER_MODIFIERS
            .choose_weighted(&mut thread_rng(), |modifier| {
                if level >= modifier.required_level {
                    modifier.likelihood
                } else {
                    0.0
                }
            })
            .unwrap()
            .clone();
        modifier
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpawnedMonster {
    pub hitpoints: f64,
    pub monster: Monster,
    pub modifier: MonsterModifier,
}

impl SpawnedMonster {
    pub fn spawn(level: usize) -> Self {
        let monster = MONSTERS
            .choose_weighted(&mut thread_rng(), |monster| {
                if level >= monster.required_level {
                    monster.likelihood(level)
                } else {
                    0.0
                }
            })
            .unwrap()
            .clone();
        let modifier = MonsterModifier::choose_random(level);

        let hitpoint_jitter = Uniform::new(1.0 / 1.1, 1.1);
        Self {
            hitpoints: hitpoint_jitter.sample(&mut thread_rng())
                * monster.hitpoints
                * modifier.hitpoint_factor,
            monster,
            modifier,
        }
    }

    pub fn to_lowercase_string(&self) -> String {
        if self.modifier.name == "normal" {
            self.monster.name.clone()
        } else {
            format!("{} {}", self.modifier.name, self.monster.name)
        }
    }
}
