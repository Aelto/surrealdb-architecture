use serde::Deserialize;
use serde::Serialize;
use surreal_simple_querybuilder::model;
use surreal_simple_querybuilder::prelude::IntoKey;

use crate::types::Id;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct IMessage {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<Id>,
  pub text: String,
}

model!(Message with(partial) {
  id,
  pub text
});
crate::with_model!(IMessage);

impl IntoKey<Id> for IMessage {
  fn into_key<E>(&self) -> Result<Id, E>
  where
    E: serde::ser::Error,
  {
    self
      .id
      .as_ref()
      .map(Id::clone)
      .ok_or(serde::ser::Error::custom("The message has no ID"))
  }
}

impl From<&str> for IMessage {
  fn from(value: &str) -> Self {
    value.to_owned().into()
  }
}

impl From<String> for IMessage {
  fn from(value: String) -> Self {
    Self {
      text: value,
      ..Default::default()
    }
  }
}
