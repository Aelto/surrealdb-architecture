mod message;
mod user;

use serde::de::DeserializeOwned;
use serde::Serialize;
use surreal_simple_querybuilder::queries::QueryBuilderInjecter;
use surrealdb::opt::QueryResult;
use surrealdb::sql::thing;

pub use message::*;
pub use user::*;

use crate::client::DB;
use crate::errors::ApiResult;

#[async_trait::async_trait]
pub trait Model
where
  Self: Sized + Serialize + DeserializeOwned + Send + Sync,
{
  fn table() -> &'static str {
    "Model"
  }

  fn id(&self) -> Option<&String>;

  async fn create(self) -> ApiResult<Self> {
    let item = DB.create(Self::table()).content(self).await?;

    Ok(item)
  }

  async fn delete(&self) -> ApiResult<()> {
    if let Some(id) = self.id() {
      DB.delete(thing(id)?).await?;
    }

    Ok(())
  }

  async fn update(self) -> ApiResult<Self> {
    if let Some(id) = self.id() {
      let item = DB.update(thing(id)?).content(self).await?;

      Ok(item)
    } else {
      Ok(self)
    }
  }

  async fn merge(self, merge: impl Serialize + Send) -> ApiResult<Self> {
    if let Some(id) = self.id() {
      let item = DB.update(thing(id)?).merge(merge).await?;

      Ok(item)
    } else {
      Ok(self)
    }
  }

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
