use crate::game_state::currency::Currency;
use enum_iterator::Sequence;
use lazy_static::lazy_static;
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand_distr::Gamma;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref MONSTERS: Vec<Monster> = vec![
        Monster::new("rat", 0, 1.0, 60, Currency::from_copper(1)),
        Monster::new("hare", 2, 1.0, 120, Currency::from_copper(4)),
        Monster::new("fox", 4, 1.0, 300, Currency::from_copper(6)),
        Monster::new("deer", 6, 1.0, 600, Currency::from_copper(8)),
        Monster::new("boar", 10, 1.0, 2000, Currency::from_copper(17)),
        Monster::new("wolf", 12, 1.0, 4000, Currency::from_copper(31)),
        Monster::new("goblin", 14, 1.0, 8000, Currency::from_copper(60)),
        Monster::new("orc", 16, 1.0, 13000, Currency::from_copper(150)),
        Monster::new("dragon", 30, 0.1, 500_000, Currency::from_gold(1)),
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
    pub required_level: u64,
    pub base_likelihood: f64,
    pub hitpoints: u64,
    pub currency_reward: Currency,
}

impl Monster {
    pub fn new(
        name: impl ToString,
        required_level: u64,
        base_likelihood: f64,
        hitpoints: u64,
        currency_reward: Currency,
    ) -> Self {
        Self {
            name: name.to_string(),
            required_level,
            base_likelihood,
            hitpoints,
            currency_reward,
        }
    }

    pub fn likelihood(&self, level: u64) -> f64 {
        self.base_likelihood / (level.max(self.required_level + 1) - self.required_level) as f64
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MonsterModifier {
    pub name: String,
    pub required_level: u64,
    pub likelihood: f64,
    pub hitpoint_factor: f64,
}

impl MonsterModifier {
    pub fn new(
        name: impl ToString,
        required_level: u64,
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

    pub fn choose_random(level: u64) -> Self {
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
    pub hitpoints: u64,
    pub monster: Monster,
    pub modifier: MonsterModifier,
    pub currency_reward: Currency,
}

impl SpawnedMonster {
    pub fn spawn(level: u64) -> Self {
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

        let hitpoint_jitter = Uniform::new(1.0 / 1.1, 1.1).sample(&mut thread_rng());
        let currency_jitter = Gamma::new(2.0, 0.25).unwrap().sample(&mut thread_rng()) + 0.5;
        Self {
            hitpoints: (hitpoint_jitter * (monster.hitpoints as f64) * modifier.hitpoint_factor)
                .round() as u64,
            currency_reward: Currency::from_copper(
                (monster.currency_reward.copper() as f64 * currency_jitter).round() as i128,
            ),
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
