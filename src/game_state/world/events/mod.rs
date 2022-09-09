use crate::game_state::character::{Character, CharacterAttributeProgress};
use crate::game_state::currency::Currency;
use crate::game_state::player_actions::{
    PlayerActionInProgress, PlayerActionInProgressKind, PlayerActionInProgressSource,
};
use crate::game_state::time::GameTime;
use crate::game_state::world::monsters::{CompiledMonster, Monster, MonsterId};
use crate::game_state::MAX_COMBAT_DURATION;
use crate::game_template::parser::WeightedIdentifier;
use crate::game_template::IdMaps;
use event_trigger_action_system::TriggerHandle;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExplorationEvent {
    pub id_str: String,
    pub name: String,
    pub verb_progressive: String,
    pub verb_simple_past: String,
    pub monster: Option<String>,
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
    pub name: String,
    pub verb_progressive: String,
    pub verb_simple_past: String,
    pub monster: Option<MonsterId>,
    pub attribute_progress: CharacterAttributeProgress,
    pub currency_reward: Currency,
    pub activation_condition: TriggerHandle,
    pub deactivation_condition: TriggerHandle,
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
            name: self.name,
            verb_progressive: self.verb_progressive,
            verb_simple_past: self.verb_simple_past,
            monster: self
                .monster
                .map(|monster| *id_maps.monsters.get(&monster).unwrap()),
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
    ) -> PlayerActionInProgress {
        if let Some(monster_id) = self.monster {
            let monster = &monsters[monster_id.0];
            let damage = character.damage_output();
            let duration = GameTime::from_milliseconds(
                (monster.hitpoints / damage * 60_000.0).round() as i128,
            )
            .min(MAX_COMBAT_DURATION);
            let attribute_progress = character.evaluate_combat_attribute_progress(duration);
            let success = duration < MAX_COMBAT_DURATION;

            PlayerActionInProgress {
                verb_progressive: self.verb_progressive.clone(),
                verb_simple_past: self.verb_simple_past.clone(),
                source: PlayerActionInProgressSource::Exploration(self.id),
                kind: PlayerActionInProgressKind::Combat(monster_id),
                start: start_time,
                end: start_time + duration,
                attribute_progress,
                currency_reward: self.currency_reward,
                success,
            }
        } else {
            PlayerActionInProgress {
                verb_progressive: self.verb_progressive.clone(),
                verb_simple_past: self.verb_simple_past.clone(),
                source: PlayerActionInProgressSource::Exploration(self.id),
                kind: PlayerActionInProgressKind::None,
                start: start_time,
                end: start_time + default_duration,
                attribute_progress: self.attribute_progress,
                currency_reward: self.currency_reward,
                success: true,
            }
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
