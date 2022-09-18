use crate::game_state::character::{Character, CharacterAttributeProgress};
use crate::game_state::currency::Currency;
use crate::game_state::player_actions::{
    PlayerActionInProgress, PlayerActionInProgressKind, PlayerActionInProgressSource,
};
use crate::game_state::time::GameTime;
use crate::game_state::world::locations::LocationId;
use crate::game_state::world::monsters::{CompiledMonster, MonsterId};
use crate::game_state::{MAX_COMBAT_DURATION, MIN_COMBAT_DURATION};
use crate::game_template::parser::WeightedIdentifier;
use crate::game_template::IdMaps;
use crate::utils::text::a_or_an;
use event_trigger_action_system::TriggerHandle;
use rand::distributions::Distribution;
use rand::Rng;
use rand_distr::{Gamma, Normal};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExplorationEvent {
    pub id_str: String,
    pub kind: ExplorationEventKind,
    pub attribute_progress: CharacterAttributeProgress,
    pub currency_reward: Currency,
    pub activation_condition: String,
    pub deactivation_condition: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompiledExplorationEvent {
    pub id: ExplorationEventId,
    pub id_str: String,
    pub state: ExplorationEventState,
    pub kind: CompiledExplorationEventKind,
    pub attribute_progress: CharacterAttributeProgress,
    pub currency_reward: Currency,
    pub activation_condition: TriggerHandle,
    pub deactivation_condition: TriggerHandle,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExplorationEventKind {
    Normal {
        name: String,
        verb_progressive: String,
        verb_simple_past: String,
    },
    Monster {
        monster: String,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CompiledExplorationEventKind {
    Normal {
        name: String,
        verb_progressive: String,
        verb_simple_past: String,
    },
    Monster {
        monster: MonsterId,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeightedExplorationEvent {
    pub id_str: String,
    pub weight: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompiledWeightedExplorationEvent {
    pub id: ExplorationEventId,
    pub weight: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExplorationEventState {
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
pub struct ExplorationEventId(pub usize);

impl ExplorationEvent {
    pub fn compile(self, id_maps: &IdMaps) -> CompiledExplorationEvent {
        CompiledExplorationEvent {
            id: *id_maps.exploration_events.get(&self.id_str).unwrap(),
            id_str: self.id_str,
            state: ExplorationEventState::Inactive,
            kind: self.kind.compile(id_maps),
            attribute_progress: self.attribute_progress,
            currency_reward: self.currency_reward,
            activation_condition: *id_maps.triggers.get(&self.activation_condition).unwrap(),
            deactivation_condition: *id_maps.triggers.get(&self.deactivation_condition).unwrap(),
        }
    }
}

impl CompiledExplorationEvent {
    pub fn spawn(
        &self,
        rng: &mut impl Rng,
        start_time: GameTime,
        default_duration: GameTime,
        character: &Character,
        monsters: &[CompiledMonster],
        location: LocationId,
    ) -> PlayerActionInProgress {
        match &self.kind {
            CompiledExplorationEventKind::Monster {
                monster: monster_id,
            } => {
                let monster = &monsters[monster_id.0];

                let damage = character.damage_output();
                let hitpoint_jitter = Normal::new(1.0, 0.1).unwrap().sample(rng);
                let duration = GameTime::from_milliseconds(
                    (monster.hitpoints * hitpoint_jitter / damage * 60_000.0).round() as i128,
                )
                .clamp(MIN_COMBAT_DURATION, MAX_COMBAT_DURATION);
                let success = duration < MAX_COMBAT_DURATION;

                let currency_jitter = Gamma::new(2.0, 0.25).unwrap().sample(rng) + 0.5;
                let currency_reward = if success {
                    Currency::from_copper_f64(
                        self.currency_reward.copper() as f64 * currency_jitter,
                    )
                } else {
                    Currency::zero()
                };

                let attribute_progress = if success {
                    character.evaluate_combat_attribute_progress(duration)
                } else {
                    CharacterAttributeProgress::zero()
                };

                let a_or_an = a_or_an(&monster.name);
                PlayerActionInProgress {
                    verb_progressive: format!("fighting {a_or_an} {}", monster.name.to_lowercase()),
                    verb_simple_past: format!("fought {a_or_an} {}", monster.name.to_lowercase()),
                    source: PlayerActionInProgressSource::Exploration(self.id),
                    kind: PlayerActionInProgressKind::Combat(*monster_id),
                    start: start_time,
                    end: start_time + duration,
                    attribute_progress,
                    currency_reward,
                    location,
                    success,
                }
            }
            CompiledExplorationEventKind::Normal {
                verb_progressive,
                verb_simple_past,
                ..
            } => PlayerActionInProgress {
                verb_progressive: verb_progressive.clone(),
                verb_simple_past: verb_simple_past.clone(),
                source: PlayerActionInProgressSource::Exploration(self.id),
                kind: PlayerActionInProgressKind::None,
                start: start_time,
                end: start_time + default_duration,
                attribute_progress: self.attribute_progress,
                currency_reward: self.currency_reward,
                location,
                success: true,
            },
        }
    }
}

impl ExplorationEventKind {
    pub fn compile(self, id_maps: &IdMaps) -> CompiledExplorationEventKind {
        match self {
            ExplorationEventKind::Normal {
                name,
                verb_progressive,
                verb_simple_past,
            } => CompiledExplorationEventKind::Normal {
                name,
                verb_progressive,
                verb_simple_past,
            },
            ExplorationEventKind::Monster { monster } => CompiledExplorationEventKind::Monster {
                monster: *id_maps.monsters.get(&monster).unwrap(),
            },
        }
    }
}

impl WeightedExplorationEvent {
    pub fn compile(self, id_maps: &IdMaps) -> CompiledWeightedExplorationEvent {
        CompiledWeightedExplorationEvent {
            id: *id_maps.exploration_events.get(&self.id_str).unwrap(),
            weight: self.weight,
        }
    }
}

#[allow(dead_code)]
impl ExplorationEventState {
    pub fn is_inactive(&self) -> bool {
        matches!(self, ExplorationEventState::Inactive)
    }

    pub fn is_active(&self) -> bool {
        matches!(self, ExplorationEventState::Active { .. })
    }

    pub fn is_deactivated(&self) -> bool {
        matches!(self, ExplorationEventState::Deactivated { .. })
    }
}

impl From<usize> for ExplorationEventId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<WeightedIdentifier> for WeightedExplorationEvent {
    fn from(weighted_identifier: WeightedIdentifier) -> Self {
        Self {
            id_str: weighted_identifier.identifier,
            weight: weighted_identifier.weight,
        }
    }
}
