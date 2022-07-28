use enum_iterator::{all, Sequence};
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, Serialize, Deserialize, Sequence, Eq, PartialEq)]
pub enum Monster {
    Rat,
    Hare,
    Deer,
    Wolf,
    Goblin,
    Orc,
    Dragon,
}

impl Monster {
    pub fn required_level(&self) -> usize {
        match self {
            Monster::Rat => 0,
            Monster::Hare => 2,
            Monster::Deer => 4,
            Monster::Wolf => 8,
            Monster::Goblin => 10,
            Monster::Orc => 13,
            Monster::Dragon => 18,
        }
    }

    pub fn likelyhood(&self, level: usize) -> f64 {
        let base_likelyhood = match self {
            Monster::Rat => 1.0,
            Monster::Hare => 1.0,
            Monster::Deer => 1.0,
            Monster::Wolf => 1.0,
            Monster::Goblin => 1.0,
            Monster::Orc => 1.0,
            Monster::Dragon => 1.0,
        };

        base_likelyhood / (level.max(self.required_level() + 1) - self.required_level()) as f64
    }

    pub fn base_hitpoints(&self) -> f64 {
        match self {
            Monster::Rat => 300.0,
            Monster::Hare => 500.0,
            Monster::Deer => 1_000.0,
            Monster::Wolf => 4_000.0,
            Monster::Goblin => 8_000.0,
            Monster::Orc => 13_000.0,
            Monster::Dragon => 500_000.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Sequence, Eq, PartialEq)]
pub enum MonsterModifier {
    None,
    Weak,
    Strong,
    Young,
    Old,
    Veteran,
    Elite,
}

impl MonsterModifier {
    pub fn likelyhood(&self) -> f64 {
        match self {
            MonsterModifier::None => 1.0,
            MonsterModifier::Weak => 0.1,
            MonsterModifier::Strong => 0.2,
            MonsterModifier::Young => 0.01,
            MonsterModifier::Old => 0.01,
            MonsterModifier::Veteran => 0.1,
            MonsterModifier::Elite => 0.1,
        }
    }

    pub fn hitpoint_factor(&self) -> f64 {
        match self {
            MonsterModifier::None => 1.0,
            MonsterModifier::Weak => 0.5,
            MonsterModifier::Strong => 1.8,
            MonsterModifier::Young => 0.3,
            MonsterModifier::Old => 1.1,
            MonsterModifier::Veteran => 3.0,
            MonsterModifier::Elite => 5.0,
        }
    }

    pub fn choose_random() -> Self {
        let candidates: Vec<_> = all::<MonsterModifier>()
            .map(|modifier| (modifier.likelyhood(), modifier))
            .collect();
        let modifier = candidates
            .choose_weighted(&mut thread_rng(), |(weight, _)| *weight)
            .unwrap()
            .1
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
        let candidates: Vec<_> = all::<Monster>()
            .filter_map(|monster| {
                if monster.required_level() <= level {
                    Some((monster.likelyhood(level), monster))
                } else {
                    None
                }
            })
            .collect();

        let monster = candidates
            .choose_weighted(&mut thread_rng(), |(weight, _)| *weight)
            .unwrap()
            .1
            .clone();
        let modifier = MonsterModifier::choose_random();

        let hitpoint_jitter = Uniform::new(1.0 / 1.1, 1.1);
        Self {
            hitpoints: hitpoint_jitter.sample(&mut thread_rng())
                * monster.base_hitpoints()
                * modifier.hitpoint_factor(),
            monster,
            modifier,
        }
    }
}
