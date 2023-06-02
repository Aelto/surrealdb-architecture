#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use errors::ApiResult;

use crate::client::DB;
use crate::models::Model;
use crate::models::UserParam;
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
/// - to use an enum to adapt the queries to our needs (ex: fetch 'user.messages' only when asked)
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

  let user: Option<IUser> = IUser::find_by_handle("John-Doe", UserParam::FetchMessages).await?;
  let found_user = user.unwrap_or_default();

  assert_eq!(created_user.id, found_user.id);
  assert!(found_user.messages.value().is_some());

  // to confirm the messages were fetched we get the text of our first message
  // and we'll compare it to message_0.text
  let first_message_text = found_user
    .messages
    .value()
    .and_then(|messages| messages.first())
    .map(|m| &m.text);

  assert_eq!(first_message_text, Some(&String::from("Hello")));

  found_user.m_delete().await?;
  let user: Option<IUser> = IUser::find_by_handle("John-Doe", UserParam::None).await?;
  assert!(user.is_none(), "no user found as it was deleted");

  Ok(())
}

/// This example demonstrates how:
/// - to update certain fields of a node using `merge`
/// - to mutate & update the entire node using `update`
async fn example_1() -> ApiResult<()> {
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
    .update_handle_and_messages("Jean-Dupont", vec![message_2, message_3])
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
  let found_post: Option<IPost> = IPost::find_by_title_and_tag(&post_1.title, "second").await?;

  assert!(found_post.is_some());
  if let Some(found_post) = found_post {
    assert_eq!(found_post.id, post_1.id);
  }

  // 1.
  // How to perform a deep/nested search, search for all posts whose author
  // handle is John Doe
  let johndoe_posts: Vec<IPost> = IPost::find_by_author_handle(&created_user.handle).await?;

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
