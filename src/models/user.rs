use serde::Deserialize;
use serde::Serialize;
use surreal_simple_querybuilder::model;
use surreal_simple_querybuilder::prelude::ForeignVec;
use surreal_simple_querybuilder::prelude::IntoKey;

use super::message::IMessage;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IUser {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<String>,
  pub handle: String,
  pub messages: ForeignVec<IMessage>,
}

model!(User with(partial) {
  id,
  pub handle,
  pub messages
});
crate::with_model!(IUser);

impl IntoKey<String> for IUser {
  fn into_key<E>(&self) -> Result<String, E>
  where
    E: serde::ser::Error,
  {
    self
      .id
      .as_ref()
      .map(String::clone)
      .ok_or(serde::ser::Error::custom("The user has no ID"))
  }
}
