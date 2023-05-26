pub static DB: Surreal<surrealdb::engine::local::Db> = Surreal::init();

use surrealdb::Surreal;

use crate::errors::ApiResult;

pub async fn connect(address: &str, namespace: &str, database: &str) -> ApiResult<()> {
  if let Err(_) = std::fs::create_dir_all(address) {}
  let absolute_path = dunce::canonicalize(address).expect("invalid path");

  DB.connect::<surrealdb::engine::local::File>(absolute_path.as_path())
    .await?;

  DB.use_ns(namespace).use_db(database).await?;

  Ok(())
}
