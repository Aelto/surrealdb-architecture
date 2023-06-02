use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use surreal_simple_querybuilder::model;
use surreal_simple_querybuilder::prelude::*;
use surreal_simple_querybuilder::wjson;

use super::IUser;
use super::Model;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IPost {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<Id>,

  pub author: Foreign<IUser>,
  pub title: String,
  pub body: String,
  pub tags: Vec<String>,
}

use crate::errors::ApiResult;
use crate::models::user::schema::User;
use crate::types::Foreign;
use crate::types::Id;

model!(Post {
  id,
  pub author<User>,
  pub title,
  pub body,
  pub tags
});
crate::with_model!(IPost);
use schema::model;

impl IPost {
  pub async fn find_by_title_and_tag(title: &str, tag: &str) -> ApiResult<Option<Self>> {
    let post = Self::m_find(Where((
      // since we want to use an operator a tad more complex than the default =
      // we use the Cmp type to specify what we want.
      Cmp("CONTAINS", json!({ model.tags: tag })),
      // also perform an exact match on the title
      json!({ model.title: title }),
    )))
    .await?;

    Ok(post)
  }

  pub async fn find_by_author_handle(handle: &str) -> ApiResult<Vec<Self>> {
    let posts = Self::m_find((
      wjson!({
        model.author().handle: handle
      }),
      OrderBy::asc(model.title),
    ))
    .await?;

    Ok(posts)
  }
}
