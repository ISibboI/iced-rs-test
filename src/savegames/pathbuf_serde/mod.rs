use async_std::path::PathBuf;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ffi::OsString;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct PathBufSerde(PathBuf);

impl Serialize for PathBufSerde {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.as_os_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PathBufSerde {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        OsString::deserialize(deserializer).map(|os_string| Self(os_string.into()))
    }
}

impl From<PathBuf> for PathBufSerde {
    fn from(value: PathBuf) -> Self {
        Self(value)
    }
}

impl From<PathBufSerde> for PathBuf {
    fn from(value: PathBufSerde) -> Self {
        value.0
    }
}

impl AsRef<std::path::Path> for PathBufSerde {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}
