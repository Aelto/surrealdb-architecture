pub static DB: Surreal<surrealdb::engine::local::Db> = Surreal::init();

use surrealdb::Surreal;

use crate::errors::ApiResult;

pub async fn connect(address: &str, namespace: &str, database: &str) -> ApiResult<()> {
  DB.connect::<surrealdb::engine::local::File>(address)
    .await?;

  DB.use_ns(namespace).use_db(database).await?;

  Ok(())
}
