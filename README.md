
# SurrealDB Architecture
This repository serves as a clean slate for anything related to models. The main
goal is to display how to combine:
  - the [official surrealdb client](https://github.com/surrealdb/surrealdb/tree/main/lib) to easily communicate with the Surreal DB
  - the [surreal-simple-querybuilder](https://github.com/Aelto/surreal-simple-querybuilder) crate
in order to get easy to construct generic queries that accept extra parameters like
pagination, fetch statements, etc...

# Summary
- [SurrealDB Architecture](#surrealdb-architecture)
- [Summary](#summary)
- [Content](#content)
  - [Snippets/Showcase](#snippetsshowcase)
  - [Structure](#structure)
  - [Models, queries \& params](#models-queries--params)

# Content
## Snippets/Showcase
> samples of project to quickly showcase the benefits

[0]: perform a search on a single field
```rs
pub async fn find_by_handle(handle: &str) -> ApiResult<Option<Self>> {
  let user = Self::m_find(Where((model.handle, handle))).await?;

  Ok(user)
}
```
[1]: perform a search on nested fields
```rs
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
```
[2]: update only parts of a node
```rs
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
```
[3]: easily create new nodes
```rs

IMessage::from("Hello").create().await?
```

## Structure

- `src/`
  - `client/` contains the DB client's code
  - `models/` each file in the folder represents a "table" in the database. The structs that represent the tables usually start with a capital `I` to easily recognize them
    - [`mod.rs`](/src/models/mod.rs) contains a generic `Model` trait that all models implement. The trait is the interface between our querybuilder and the DB client, that means the `DB` client should in theory never be seen in the model files themselves. _The generic functions are prefixed with a `m_` to easily notice when a generic `m_create` function is used vs a hand written `create()` one._
    - model files
      - `IMyModel` struct, it is used to hold the data of the nodes and represents the table
      - `model!({})` macro call, it declares the fields on the table & allows us to reference them in our queries so we have compile-time safeties to make sure we references fields that exist.
      - `impl super::Model for IMyModel` implements the generic `Model` trait for the model. It only needs to give the name of the table and a way to get a node's ID
    - [`param/`](/src/models/params/), params are enums that implement `QueryBuilderInjecter` so you can write queries that can be adapted depending on the situation.

## Models, queries & params
The project demonstrates the use an [of an enum](/src/models/params/user.rs) to easily adapt the queries:
```rs
pub enum UserParam {
  None,
  FetchMessages,
}

impl IUser {
  pub async fn find_by_handle(handle: &str, params: UserParam) -> ApiResult<Option<Self>> {
    let filter = Where((model.handle, handle));
    let user = Self::m_find((filter, params)).await?;

    Ok(user)
  }
}
```

Thanks to the `QueryBuilderInjecter` trait, each variant of the enum can be used to inject statements into the queries. We now have a `User::find_by_handle()` function that fetches either a user or a user + its messages.
