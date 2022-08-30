use crate::game_state::story::quests::CompiledQuest;
use crate::game_state::story::Story;
use crate::utils::serde::IteratorSerializer;
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;

impl Serialize for Story {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut serialize_struct = serializer.serialize_struct("Story", 3)?;
        serialize_struct.serialize_field(
            "inactive_quests",
            &IteratorSerializer::new(self.inactive_quests.values()),
        )?;
        serialize_struct.serialize_field(
            "active_quests",
            &IteratorSerializer::new(self.active_quests.values()),
        )?;
        serialize_struct.serialize_field(
            "completed_quests",
            &IteratorSerializer::new(self.completed_quests.values()),
        )?;
        serialize_struct.end()
    }
}

impl<'de> Deserialize<'de> for Story {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        #[allow(clippy::enum_variant_names)]
        enum Field {
            InactiveQuests,
            ActiveQuests,
            CompletedQuests,
        }

        struct StoryVisitor;

        impl<'de> Visitor<'de> for StoryVisitor {
            type Value = Story;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                write!(formatter, "struct Story")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let inactive_quests = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let active_quests = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let completed_quests = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                Ok(Story::from_deserialization(
                    inactive_quests,
                    active_quests,
                    completed_quests,
                ))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut inactive_quests = None;
                let mut active_quests = None;
                let mut completed_quests = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::InactiveQuests => {
                            if inactive_quests.is_some() {
                                return Err(de::Error::duplicate_field("inactive_quests"));
                            }
                            inactive_quests = Some(map.next_value()?);
                        }
                        Field::ActiveQuests => {
                            if active_quests.is_some() {
                                return Err(de::Error::duplicate_field("active_quests"));
                            }
                            active_quests = Some(map.next_value()?);
                        }
                        Field::CompletedQuests => {
                            if completed_quests.is_some() {
                                return Err(de::Error::duplicate_field("completed_quests"));
                            }
                            completed_quests = Some(map.next_value()?);
                        }
                    }
                }
                let inactive_quests =
                    inactive_quests.ok_or_else(|| de::Error::missing_field("inactive_quests"))?;
                let active_quests =
                    active_quests.ok_or_else(|| de::Error::missing_field("active_quests"))?;
                let completed_quests =
                    completed_quests.ok_or_else(|| de::Error::missing_field("completed_quests"))?;
                Ok(Story::from_deserialization(
                    inactive_quests,
                    active_quests,
                    completed_quests,
                ))
            }
        }

        const FIELDS: &[&str] = &["inactive_quests", "active_quests", "completed_quests"];
        deserializer.deserialize_struct("Story", FIELDS, StoryVisitor)
    }
}

impl Story {
    fn from_deserialization(
        inactive_quests: Vec<CompiledQuest>,
        active_quests: Vec<CompiledQuest>,
        completed_quests: Vec<CompiledQuest>,
    ) -> Self {
        let active_quests_by_activation_time = active_quests
            .iter()
            .map(|quest| (quest.state.activation_time().unwrap(), quest.id))
            .collect();
        let completed_quests_by_completion_time = completed_quests
            .iter()
            .map(|quest| (quest.state.completion_time().unwrap(), quest.id))
            .collect();

        Self {
            inactive_quests: inactive_quests
                .into_iter()
                .map(|quest| (quest.id, quest))
                .collect(),
            active_quests: active_quests
                .into_iter()
                .map(|quest| (quest.id, quest))
                .collect(),
            active_quests_by_activation_time,
            completed_quests: completed_quests
                .into_iter()
                .map(|quest| (quest.id, quest))
                .collect(),
            completed_quests_by_completion_time,
        }
    }
}
