pub mod message;
pub mod post;
pub mod user;

use serde::de::DeserializeOwned;
use serde::Serialize;
use surreal_simple_querybuilder::queries::QueryBuilderInjecter;
use surrealdb::opt::QueryResult;
use surrealdb::sql::thing;

pub use message::*;
pub use post::*;
pub use user::*;

use crate::client::DB;
use crate::errors::ApiResult;

#[async_trait::async_trait]
pub trait Model
where
  Self: Sized + Serialize + DeserializeOwned + Send + Sync,
{
  fn table() -> &'static str;
  fn id(&self) -> Option<&String>;

  //////////////////////////////////////////////////////////////////////////////

  async fn create(self) -> ApiResult<Self> {
    let item = DB.create(Self::table()).content(self).await?;

    Ok(item)
  }

  //////////////////////////////////////////////////////////////////////////////

  async fn delete(&self) -> ApiResult<()> {
    if let Some(id) = self.id() {
      DB.delete(thing(id)?).await?;
    }

    Ok(())
  }

  //////////////////////////////////////////////////////////////////////////////

  async fn update(self) -> ApiResult<Self> {
    if let Some(id) = self.id() {
      let item = DB.update(thing(id)?).content(self).await?;

      Ok(item)
    } else {
      Ok(self)
    }
  }

  //////////////////////////////////////////////////////////////////////////////

  async fn merge_one(id: &str, merge: impl Serialize + Send) -> ApiResult<Self> {
    let item = DB.update(thing(id)?).merge(merge).await?;

    Ok(item)
  }

  async fn merge(self, merge: impl Serialize + Send) -> ApiResult<Self> {
    match self.id() {
      Some(id) => Self::merge_one(id, merge).await,
      None => Ok(self),
    }
  }

  //////////////////////////////////////////////////////////////////////////////

  /// Add a value to a field, it can be an item in an array or a suffix to a string.
  ///
  async fn add_one(id: &str, field: &str, value: impl Serialize + Send) -> ApiResult<()> {
    let _diff: serde_json::Value = DB
      .update(thing(id)?)
      .patch(surrealdb::opt::PatchOp::add(field, value))
      .await?;

    Ok(())
  }

  async fn add(self, field: &str, value: impl Serialize + Send) -> ApiResult<()> {
    match self.id() {
      Some(id) => Self::add_one(id, field, value).await,
      None => Ok(()),
    }
  }

  //////////////////////////////////////////////////////////////////////////////

  async fn find<'a, R>(params: impl QueryBuilderInjecter<'a> + Send + 'a) -> ApiResult<R>
  where
    R: DeserializeOwned,
    usize: QueryResult<R>,
  {
    let (query, params) = surreal_simple_querybuilder::queries::select("*", Self::table(), params)?;
    let items = DB.query(query).bind(params).await?.take(0)?;

    Ok(items)
  }
}
