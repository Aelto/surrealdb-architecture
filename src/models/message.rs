use serde::Deserialize;
use serde::Serialize;
use surreal_simple_querybuilder::model;
use surreal_simple_querybuilder::prelude::IntoKey;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct IMessage {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<String>,
  pub text: String,
}

model!(Message with(partial) {
  id,
  pub text
});
crate::with_model!(IMessage);

impl IntoKey<String> for IMessage {
  fn into_key<E>(&self) -> Result<String, E>
  where
    E: serde::ser::Error,
  {
    self
      .id
      .as_ref()
      .map(String::clone)
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
