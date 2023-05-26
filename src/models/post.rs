use serde::Deserialize;
use serde::Serialize;
use surreal_simple_querybuilder::model;

use super::IUser;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPost {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<Id>,

  pub author: Foreign<IUser>,
  pub title: String,
  pub body: String,
  pub tags: Vec<String>,
}

use crate::models::user::schema::User;
use crate::types::Foreign;
use crate::types::Id;

model!(Post with(partial) {
  id,
  pub author<User>,
  pub title,
  pub body,
  pub tags
});
crate::with_model!(IPost);
