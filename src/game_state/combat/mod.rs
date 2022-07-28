use enum_iterator::Sequence;
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
