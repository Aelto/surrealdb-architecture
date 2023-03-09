use errors::ApiResult;
use surreal_simple_querybuilder::prelude::ForeignVec;
use surreal_simple_querybuilder::types::Where;

use crate::models::Model;

mod client;
mod errors;
mod models;

#[tokio::main]
async fn main() -> errors::ApiResult<()> {
  client::connect("database.db", "namespace", "database").await?;

  example_0().await?;
  example_1().await?;

  Ok(())
}

/// This example demonstrates how:
/// - to create nodes and retrieve them
/// - to create nested types using Foreign & ForeignVec fields
async fn example_0() -> ApiResult<()> {
  use crate::models::IMessage;
  use crate::models::IUser;

  let message_0 = IMessage::from("Hello").create().await?;
  let message_1 = IMessage::from("World!").create().await?;

  let created_user = IUser {
    handle: "John-Doe".to_owned(),
    messages: ForeignVec::new_value(vec![message_0, message_1]),
    id: None,
  }
  .create()
  .await?;

  let user: Option<IUser> = IUser::find(Where(("handle", "John-Doe"))).await?;
  let found_user = user.unwrap_or_default();

  assert_eq!(created_user.id, found_user.id);

  found_user.delete().await?;
  let user: Option<IUser> = IUser::find(Where(("handle", "John-Doe"))).await?;
  assert!(user.is_none(), "no user found as it was deleted");

  Ok(())
}

/// This example demonstrates how:
/// - to update certain fields of a node using `merge`
/// - to mutate & update the entire node using `update`
async fn example_1() -> ApiResult<()> {
  use crate::models::IMessage;
  use crate::models::IUser;
  use crate::models::PUserData;

  let message_0 = IMessage::from("Hello").create().await?;
  let message_1 = IMessage::from("World!").create().await?;
  let message_2 = IMessage::from("Bonjour").create().await?;
  let message_3 = IMessage::from("Le monde !").create().await?;

  let created_user = IUser {
    handle: "John-Doe".to_owned(),
    messages: ForeignVec::new_value(vec![message_0, message_1]),
    id: None,
  }
  .create()
  .await?;

  let original_id = created_user.id.clone();
  let mut updated_user = created_user
    .merge(PUserData {
      handle: "Jean-Dupont".to_owned(),
      messages: ForeignVec::new_value(vec![message_2, message_3]),
    })
    .await?;

  assert_eq!(original_id, updated_user.id);
  assert_eq!(updated_user.handle, "Jean-Dupont".to_owned());

  updated_user.handle = "Foo-Bar".to_owned();
  let updated_user = updated_user.update().await?;

  assert_eq!(updated_user.handle, "Foo-Bar".to_owned());

  updated_user.delete().await?;

  Ok(())
}
