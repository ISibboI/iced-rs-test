use crate::game_state::combat::CombatStyle;
use crate::game_state::currency::Currency;
use crate::game_state::time::GameTime;
use enum_iterator::Sequence;
use rand_distr::num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::ops;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct CharacterAttributes {
    pub strength: u64,
    pub stamina: u64,
    pub dexterity: u64,
    pub intelligence: u64,
    pub wisdom: u64,
    pub charisma: u64,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct CharacterAttributeProgress {
    pub strength: u64,
    pub stamina: u64,
    pub dexterity: u64,
    pub intelligence: u64,
    pub wisdom: u64,
    pub charisma: u64,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default)]
pub struct CharacterAttributeProgressFactor {
    pub strength: f64,
    pub stamina: f64,
    pub dexterity: f64,
    pub intelligence: f64,
    pub wisdom: f64,
    pub charisma: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub race: CharacterRace,

    pub level: u64,
    pub level_progress: u64,

    attributes: CharacterAttributes,
    attribute_progress: CharacterAttributeProgress,

    pub currency: Currency,
}

impl Character {
    pub fn new(name: String, race: CharacterRace) -> Self {
        Self {
            name,
            race,

            level: 1,
            level_progress: 0,

            attributes: race.starting_basic_attributes(),
            attribute_progress: Default::default(),

            currency: Currency::from_copper(0),
        }
    }

    pub fn add_attribute_progress(&mut self, progress: CharacterAttributeProgress) {
        let progress = progress * self.race.attribute_progress_factors();
        self.attribute_progress += progress;
        self.attributes.check_progress(&mut self.attribute_progress);

        self.add_level_progress(progress.sum());
    }

    pub fn add_level_progress(&mut self, progress: u64) {
        self.level_progress += progress;
        while self.level_progress > self.required_level_progress() {
            self.level_progress -= self.required_level_progress();
            self.level += 1;
        }
    }

    pub fn required_level_progress(&self) -> u64 {
        let level = self.level as f64;
        GameTime::from_hours(1).milliseconds() as u64
            + (GameTime::from_hours(1).milliseconds() as f64
                * level.powf(1.1)
                * level.max(2.0).log2()) as u64
    }

    pub fn attributes(&self) -> &CharacterAttributes {
        &self.attributes
    }

    pub fn attribute_progress(&self) -> &CharacterAttributeProgress {
        &self.attribute_progress
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default, Sequence, Eq, PartialEq)]
pub enum CharacterRace {
    #[default]
    Human,
    Orc,
    Elf,
    Dwarf,
}

impl CharacterRace {
    pub fn starting_basic_attributes(&self) -> CharacterAttributes {
        match self {
            CharacterRace::Human => CharacterAttributes::new(1, 1, 1, 1, 1, 2),
            CharacterRace::Orc => CharacterAttributes::new(2, 1, 1, 1, 1, 1),
            CharacterRace::Elf => CharacterAttributes::new(1, 1, 2, 1, 1, 1),
            CharacterRace::Dwarf => CharacterAttributes::new(1, 2, 1, 1, 1, 1),
        }
    }

    pub fn attribute_progress_factors(&self) -> CharacterAttributeProgressFactor {
        match self {
            CharacterRace::Human => {
                CharacterAttributeProgressFactor::new(1.0, 1.0, 1.0, 1.1, 1.0, 1.1)
            }
            CharacterRace::Orc => {
                CharacterAttributeProgressFactor::new(1.1, 1.1, 1.0, 1.0, 1.0, 1.0)
            }
            CharacterRace::Elf => {
                CharacterAttributeProgressFactor::new(1.0, 1.0, 1.1, 1.0, 1.1, 1.0)
            }
            CharacterRace::Dwarf => {
                CharacterAttributeProgressFactor::new(1.0, 1.1, 1.1, 1.0, 1.0, 1.0)
            }
        }
    }

    pub fn starting_combat_style(&self) -> CombatStyle {
        match self {
            CharacterRace::Human => CombatStyle::Magic,
            CharacterRace::Orc => CombatStyle::CloseContact,
            CharacterRace::Elf => CombatStyle::Ranged,
            CharacterRace::Dwarf => CombatStyle::CloseContact,
        }
    }
}

impl ToString for CharacterRace {
    fn to_string(&self) -> String {
        match self {
            CharacterRace::Human => "Human".to_string(),
            CharacterRace::Orc => "Orc".to_string(),
            CharacterRace::Elf => "Elf".to_string(),
            CharacterRace::Dwarf => "Dwarf".to_string(),
        }
    }
}

impl CharacterAttributes {
    pub fn new(
        strength: u64,
        stamina: u64,
        dexterity: u64,
        intelligence: u64,
        wisdom: u64,
        charisma: u64,
    ) -> Self {
        Self {
            strength,
            stamina,
            dexterity,
            intelligence,
            wisdom,
            charisma,
        }
    }

    pub fn check_progress(&mut self, progress: &mut CharacterAttributeProgress) {
        while progress.strength >= Self::required_attribute_progress(self.strength) {
            progress.strength -= Self::required_attribute_progress(self.strength);
            self.strength += 1;
        }

        while progress.stamina >= Self::required_attribute_progress(self.stamina) {
            progress.stamina -= Self::required_attribute_progress(self.stamina);
            self.stamina += 1;
        }

        while progress.dexterity >= Self::required_attribute_progress(self.dexterity) {
            progress.dexterity -= Self::required_attribute_progress(self.dexterity);
            self.dexterity += 1;
        }

        while progress.intelligence >= Self::required_attribute_progress(self.intelligence) {
            progress.intelligence -= Self::required_attribute_progress(self.intelligence);
            self.intelligence += 1;
        }

        while progress.wisdom >= Self::required_attribute_progress(self.wisdom) {
            progress.wisdom -= Self::required_attribute_progress(self.wisdom);
            self.wisdom += 1;
        }

        while progress.charisma >= Self::required_attribute_progress(self.charisma) {
            progress.charisma -= Self::required_attribute_progress(self.charisma);
            self.charisma += 1;
        }
    }

    pub fn required_attribute_progress(attribute_level: u64) -> u64 {
        attribute_level * GameTime::from_hours(1).milliseconds() as u64
    }
}

#[allow(dead_code)]
impl CharacterAttributeProgress {
    pub fn new(
        strength: u64,
        stamina: u64,
        dexterity: u64,
        intelligence: u64,
        wisdom: u64,
        charisma: u64,
    ) -> Self {
        Self {
            strength,
            stamina,
            dexterity,
            intelligence,
            wisdom,
            charisma,
        }
    }

    pub fn sum(&self) -> u64 {
        self.strength
            + self.stamina
            + self.dexterity
            + self.intelligence
            + self.wisdom
            + self.charisma
    }

    pub fn zero() -> Self {
        Default::default()
    }

    pub fn from_strength(strength: u64) -> Self {
        let mut result = Self::zero();
        result.strength = strength;
        result
    }

    pub fn from_stamina(stamina: u64) -> Self {
        let mut result = Self::zero();
        result.stamina = stamina;
        result
    }

    pub fn from_dexterity(dexterity: u64) -> Self {
        let mut result = Self::zero();
        result.dexterity = dexterity;
        result
    }

    pub fn from_intelligence(intelligence: u64) -> Self {
        let mut result = Self::zero();
        result.intelligence = intelligence;
        result
    }

    pub fn from_wisdom(wisdom: u64) -> Self {
        let mut result = Self::zero();
        result.wisdom = wisdom;
        result
    }

    pub fn from_charisma(charisma: u64) -> Self {
        let mut result = Self::zero();
        result.charisma = charisma;
        result
    }
}

#[allow(dead_code)]
impl CharacterAttributeProgressFactor {
    pub fn new(
        strength: f64,
        stamina: f64,
        dexterity: f64,
        intelligence: f64,
        wisdom: f64,
        charisma: f64,
    ) -> Self {
        Self {
            strength,
            stamina,
            dexterity,
            intelligence,
            wisdom,
            charisma,
        }
    }

    pub fn zero() -> Self {
        Default::default()
    }

    pub fn from_strength(strength: f64) -> Self {
        let mut result = Self::zero();
        result.strength = strength;
        result
    }

    pub fn from_stamina(stamina: f64) -> Self {
        let mut result = Self::zero();
        result.stamina = stamina;
        result
    }

    pub fn from_dexterity(dexterity: f64) -> Self {
        let mut result = Self::zero();
        result.dexterity = dexterity;
        result
    }

    pub fn from_intelligence(intelligence: f64) -> Self {
        let mut result = Self::zero();
        result.intelligence = intelligence;
        result
    }

    pub fn from_wisdom(wisdom: f64) -> Self {
        let mut result = Self::zero();
        result.wisdom = wisdom;
        result
    }

    pub fn from_charisma(charisma: f64) -> Self {
        let mut result = Self::zero();
        result.charisma = charisma;
        result
    }

    pub fn into_progress(self, time: GameTime) -> CharacterAttributeProgress {
        CharacterAttributeProgress {
            strength: (self.strength * time.milliseconds() as f64).round() as u64,
            stamina: (self.stamina * time.milliseconds() as f64).round() as u64,
            dexterity: (self.dexterity * time.milliseconds() as f64).round() as u64,
            intelligence: (self.intelligence * time.milliseconds() as f64).round() as u64,
            wisdom: (self.wisdom * time.milliseconds() as f64).round() as u64,
            charisma: (self.charisma * time.milliseconds() as f64).round() as u64,
        }
    }

    pub fn assert_float_normal(&self) {
        assert!(self.strength.is_normal() || self.strength.is_zero());
        assert!(self.stamina.is_normal() || self.stamina.is_zero());
        assert!(self.dexterity.is_normal() || self.dexterity.is_zero());
        assert!(self.intelligence.is_normal() || self.intelligence.is_zero());
        assert!(self.wisdom.is_normal() || self.wisdom.is_zero());
        assert!(self.charisma.is_normal() || self.charisma.is_zero());
    }
}

impl ops::Mul<CharacterAttributeProgressFactor> for CharacterAttributeProgress {
    type Output = Self;

    fn mul(self, rhs: CharacterAttributeProgressFactor) -> Self::Output {
        Self {
            strength: (self.strength as f64 * rhs.strength).round() as u64,
            stamina: (self.stamina as f64 * rhs.stamina).round() as u64,
            dexterity: (self.dexterity as f64 * rhs.dexterity).round() as u64,
            intelligence: (self.intelligence as f64 * rhs.intelligence).round() as u64,
            wisdom: (self.wisdom as f64 * rhs.wisdom).round() as u64,
            charisma: (self.charisma as f64 * rhs.charisma).round() as u64,
        }
    }
}

impl ops::Add for CharacterAttributeProgress {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            strength: self.strength + rhs.strength,
            stamina: self.stamina + rhs.stamina,
            dexterity: self.dexterity + rhs.dexterity,
            intelligence: self.intelligence + rhs.intelligence,
            wisdom: self.wisdom + rhs.wisdom,
            charisma: self.charisma + rhs.charisma,
        }
    }
}

impl ops::AddAssign for CharacterAttributeProgress {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl PartialEq for CharacterAttributeProgressFactor {
    fn eq(&self, other: &Self) -> bool {
        self.assert_float_normal();
        other.assert_float_normal();
        self.strength == other.strength
            && self.stamina == other.stamina
            && self.dexterity == other.dexterity
            && self.intelligence == other.intelligence
            && self.wisdom == other.wisdom
            && self.charisma == other.charisma
    }
}

impl Eq for CharacterAttributeProgressFactor {}
