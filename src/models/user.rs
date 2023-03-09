use serde::Deserialize;
use serde::Serialize;
use surreal_simple_querybuilder::prelude::ForeignVec;
use utility_types::omit;

use super::message::IMessage;

#[omit(PUserData, [id], [Serialize, Deserialize, Default, Debug])]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IUser {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<String>,
  pub handle: String,
  pub messages: ForeignVec<IMessage>,
}

#[async_trait::async_trait]
impl super::Model for IUser {
  fn table() -> &'static str {
    "User"
  }

  fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }
}
