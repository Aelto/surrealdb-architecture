#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use errors::ApiResult;
use serde_json::json;
use surreal_simple_querybuilder::types::Cmp;
use surreal_simple_querybuilder::types::OrderBy;
use surreal_simple_querybuilder::types::Where;
use surreal_simple_querybuilder::wjson;

use crate::client::DB;
use crate::models::Model;
use crate::types::ForeignVec;

mod client;
mod errors;
mod models;
pub mod types;

#[tokio::main]
async fn main() -> errors::ApiResult<()> {
  client::connect("database.db", "namespace", "database").await?;

  delete_everything().await?;
  example_0().await?;

  delete_everything().await?;
  example_1().await?;

  delete_everything().await?;
  example_2().await?;

  Ok(())
}

/// This example demonstrates how:
/// - to create nodes and retrieve them
/// - to create nested types using Foreign & ForeignVec fields
async fn example_0() -> ApiResult<()> {
  use crate::models::IMessage;
  use crate::models::IUser;

  let message_0 = IMessage::from("Hello").m_create().await?;
  let message_1 = IMessage::from("World!").m_create().await?;

  let created_user = IUser {
    handle: "John-Doe".to_owned(),
    messages: ForeignVec::new_value(vec![message_0, message_1]),
    id: None,
  }
  .m_create()
  .await?;

  let user: Option<IUser> = IUser::m_find(Where(("handle", "John-Doe"))).await?;
  let found_user = user.unwrap_or_default();

  assert_eq!(created_user.id, found_user.id);

  found_user.m_delete().await?;
  let user: Option<IUser> = IUser::m_find(Where(("handle", "John-Doe"))).await?;
  assert!(user.is_none(), "no user found as it was deleted");

  Ok(())
}

/// This example demonstrates how:
/// - to update certain fields of a node using `merge`
/// - to mutate & update the entire node using `update`
async fn example_1() -> ApiResult<()> {
  use crate::models::user::schema::PartialUser;
  use crate::models::IMessage;
  use crate::models::IUser;

  let message_0 = IMessage::from("Hello").m_create().await?;
  let message_1 = IMessage::from("World!").m_create().await?;
  let message_2 = IMessage::from("Bonjour").m_create().await?;
  let message_3 = IMessage::from("Le monde !").m_create().await?;

  let created_user = IUser {
    handle: "John-Doe".to_owned(),
    messages: ForeignVec::new_value(vec![message_0, message_1]),
    id: None,
  }
  .m_create()
  .await?;

  let original_id = created_user.id.clone();
  let mut updated_user = created_user
    .merge(
      PartialUser::new()
        .handle("Jean-Dupont")
        .messages(vec![message_2, message_3])
        .ok()?,
    )
    .await?;

  assert_eq!(original_id, updated_user.id);
  assert_eq!(updated_user.handle, "Jean-Dupont".to_owned());

  updated_user.handle = "Foo-Bar".to_owned();
  let updated_user = updated_user.m_update().await?;

  assert_eq!(updated_user.handle, "Foo-Bar".to_owned());

  updated_user.m_delete().await?;

  Ok(())
}

/// This example demonstrates how:
/// - to perform simple searches over few fields
/// - to perform deep searches over nested fields
async fn example_2() -> ApiResult<()> {
  use crate::models::post::schema::model as post;
  use crate::models::post::schema::PartialPost;
  use crate::models::IPost;
  use crate::models::IUser;

  let created_user = IUser {
    handle: "John-Doe".to_owned(),
    ..Default::default()
  }
  .m_create()
  .await?;

  let post_0 = IPost {
    author: created_user.id.clone().unwrap().into(),
    body: "Lorem ipsum dolor sit amet consectitur".into(),
    tags: vec!["example".into(), "post".into(), "first".into()],
    // we add the "0:" so we can do an order-by later-on
    title: "0: This is an example post".into(),
    ..Default::default()
  }
  .m_create()
  .await?;

  let post_1 = IPost {
    author: created_user.id.clone().unwrap().into(),
    body: "Lorem ipsum dolor sit amet consectitur".into(),
    tags: vec!["example".into(), "post".into(), "second".into()],
    // we add the "1:" so we can do an order-by later-on
    title: "1: This is another example post".into(),
    ..Default::default()
  }
  .m_create()
  .await?;

  // 0.
  // How to perform a simple shallow (non nested) search
  let found_post: Option<IPost> = IPost::m_find(Where((
    // since we want to use an operator a tag more complex than the default =
    // we use the Cmp type to specify what we want.
    Cmp("CONTAINS", json!({ post.tags: "second" })),
    // also perform an exact match on the title
    PartialPost::new().title(&post_1.title).ok()?,
  )))
  .await?;

  assert!(found_post.is_some());
  if let Some(found_post) = found_post {
    assert_eq!(found_post.id, post_1.id);
  }

  // 1.
  // How to perform a deep/nested search, search for all posts whose author
  // handle is John Doe

  let johndoe_posts: Vec<IPost> = IPost::m_find((
    wjson!({
      post.author().handle: created_user.handle
    }),
    OrderBy::asc(post.title),
  ))
  .await?;

  assert_eq!(johndoe_posts.len(), 2, "found two posts: {johndoe_posts:?}");
  assert_eq!(johndoe_posts[0].id, post_0.id);
  assert_eq!(johndoe_posts[1].id, post_1.id);

  Ok(())
}

async fn delete_everything() -> ApiResult<()> {
  // raw query to quickly delete everything
  DB.query("delete Post; delete User; delete Message").await?;

  Ok(())
}
