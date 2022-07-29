use crate::game_state::combat::CombatStyle;
use crate::game_state::currency::Currency;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub level: usize,
    pub race: CharacterRace,

    pub strength: usize,
    pub dexterity: usize,
    pub intelligence: usize,
    pub charisma: usize,

    pub level_progress: f64,
    pub strength_progress: f64,
    pub dexterity_progress: f64,
    pub intelligence_progress: f64,
    pub charisma_progress: f64,

    pub currency: Currency,
}

impl Character {
    pub fn new(name: String, race: CharacterRace) -> Self {
        let (strength, dexterity, intelligence, charisma) = race.starting_str_dex_int_chr();
        Self {
            name,
            level: 1,
            race,

            strength,
            dexterity,
            intelligence,
            charisma,

            level_progress: 0.0,
            strength_progress: 0.0,
            dexterity_progress: 0.0,
            intelligence_progress: 0.0,
            charisma_progress: 0.0,

            currency: Currency::from_copper(0),
        }
    }

    pub fn add_attribute_progress(&mut self, (str, dex, int, chr): (f64, f64, f64, f64)) {
        self.strength_progress += str;
        self.dexterity_progress += dex;
        self.intelligence_progress += int;
        self.charisma_progress += chr;

        while self.strength_progress > Self::required_attribute_progress(self.strength) {
            self.strength_progress -= Self::required_attribute_progress(self.strength);
            self.strength += 1;
        }

        while self.dexterity_progress > Self::required_attribute_progress(self.dexterity) {
            self.dexterity_progress -= Self::required_attribute_progress(self.dexterity);
            self.dexterity += 1;
        }

        while self.intelligence_progress > Self::required_attribute_progress(self.intelligence) {
            self.intelligence_progress -= Self::required_attribute_progress(self.intelligence);
            self.intelligence += 1;
        }

        while self.charisma_progress > Self::required_attribute_progress(self.charisma) {
            self.charisma_progress -= Self::required_attribute_progress(self.charisma);
            self.charisma += 1;
        }

        self.add_level_progress(str + dex + int + chr);
    }

    pub fn add_level_progress(&mut self, progress: f64) {
        self.level_progress += progress;
        while self.level_progress > self.required_level_progress() {
            self.level_progress -= self.required_level_progress();
            self.level += 1;
        }
    }

    pub fn required_attribute_progress(attribute_level: usize) -> f64 {
        attribute_level as f64
    }

    pub fn required_level_progress(&self) -> f64 {
        let level = self.level as f64;
        10.0 * level.powf(1.1) * level.max(2.0).log2()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Sequence, Eq, PartialEq)]
pub enum CharacterRace {
    #[default]
    Human,
    Orc,
    Elf,
    Dwarf,
}

impl CharacterRace {
    pub fn starting_str_dex_int_chr(&self) -> (usize, usize, usize, usize) {
        match self {
            CharacterRace::Human => (10, 10, 10, 10),
            CharacterRace::Orc => (20, 5, 5, 5),
            CharacterRace::Elf => (8, 15, 15, 5),
            CharacterRace::Dwarf => (15, 11, 8, 15),
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
