use std::ops::Deref;

use serde::Deserialize;
use serde::Serialize;
use surrealdb::sql::thing;
use surrealdb::sql::Thing;

#[derive(Debug, PartialEq, Clone)]
pub struct Id(String);

impl Serialize for Id {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    match thing(&self.0) {
      Ok(thing) => thing.serialize(serializer),
      Err(_) => Err(serde::ser::Error::custom("invalid-id-to-thing")),
    }
  }
}

impl<'de> Deserialize<'de> for Id {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    Thing::deserialize(deserializer)
      .map(|thing| thing.to_string())
      .map(Self::from)
  }
}

impl std::fmt::Display for Id {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.fmt(f)
  }
}

impl From<String> for Id {
  fn from(value: String) -> Self {
    Self(value)
  }
}

impl From<&str> for Id {
  fn from(value: &str) -> Self {
    value.to_owned().into()
  }
}

impl Deref for Id {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl AsRef<str> for Id {
  fn as_ref(&self) -> &str {
    &self
  }
}

/// unique implementations to the surreal-simple-querybuilder crate:
///
/// Makes it convenient to create a foreign key to a node by doing `id.into()`
impl<T> Into<super::Foreign<T>> for Id {
  fn into(self) -> super::Foreign<T> {
    super::Foreign::new_key(self)
  }
}
