use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use surreal_simple_querybuilder::model;
use surreal_simple_querybuilder::prelude::*;

use super::Model;
use crate::errors::ApiResult;
use crate::types::ForeignVec;
use crate::types::Id;

use super::message::IMessage;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IUser {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<Id>,
  pub handle: String,
  pub messages: ForeignVec<IMessage>,
}

pub use super::params::UserParam;

model!(User {
  id,
  pub handle,
  pub messages
});
crate::with_model!(IUser);
use schema::model;

impl IUser {
  /// Find the first user with the supplied handle
  pub async fn find_by_handle(handle: &str, params: UserParam) -> ApiResult<Option<Self>> {
    let filter = Where((model.handle, handle));
    let user = Self::m_find((filter, params)).await?;

    Ok(user)
  }

  pub async fn update_handle_and_messages(
    self, handle: &str, messages: Vec<IMessage>,
  ) -> ApiResult<Self> {
    let user = self
      .merge(json!({
        model.handle: handle,
        model.messages: messages
      }))
      .await?;

    Ok(user)
  }
}

impl IntoKey<Id> for IUser {
  fn into_key<E>(&self) -> Result<Id, E>
  where
    E: serde::ser::Error,
  {
    self
      .id
      .as_ref()
      .map(Id::clone)
      .ok_or(serde::ser::Error::custom("The user has no ID"))
  }
}
