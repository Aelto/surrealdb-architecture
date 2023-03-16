
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
  - [Models, queries \& recommandations](#models-queries--recommandations)

# Content
## Snippets/Showcase
> samples of project to quickly showcase the benefits

[1]: perform a search on nested fields
```rs
let handle = "john doe";
let johndoe_posts: Vec<IPost> = IPost::find((
  Where(json!({
    post.author().handle: handle
  })),
  OrderBy::asc(post.title),
))
.await?;
```
[2]: update only parts of a node
```rs
let mut updated_user = created_user
    .merge(
      PartialUser::new()
        .handle("Jean-Dupont")
        .messages(vec![message_2, message_3])
        .ok()?,
    )
    .await?;
```
[3]: easily create new nodes
```rs

IMessage::from("Hello").create().await?
```

## Structure

- `src/`
  - `client/` contains the DB client's code
  - `models/` each file in the folder represents a "table" in the database. The structs that represent the tables usually start with a capital `I` to easily recognize them
    - [`mod.rs`](/src/models/mod.rs) contains a generic `Model` trait that all models implement. The trait is the interface between our querybuilder and the DB client, that means the `DB` client should in theory never be seen in the model files.
    - model files
      - `IMyModel` struct, it is used to hold the data of the nodes and represents the table
      - `model!({})` macro call, it declares the fields on the table allows us to reference them in our queries so we have compile-time safety to make sure we references fields that exist.
      - `impl super::Model for IMyModel` implements the generic `Model` trait for the model. It only needs to give the name of the table and a way to get a node's ID

## Models, queries & recommandations
Since queries can be constructed as easily as:
```rs
let users: Vec<IUser> = IUser::find(Where(json!({
  model.handle: "John"
}))).await?;
```

It may feel nice to always do it like this and avoid creating yourself a few functions like `IUser::find_many_by_handle(&str)` like the examples do, but i'd still recommend to write functions for the common queries  to improve code quality (and possibly compile time)

The first function in the example below is easier to read and is also more future-proof for the day you'll have to find where in your entire codebase you fetch users by their handle.
```rs
#[get("/u/{handle}")]
async fn index_one(info: Path<Info>) -> Result<Option<IUser>> {
  let some_user = IUser::find_by_handle(&info.handle).await?;

  Ok(some_user)
}

#[get("/u/{handle}")]
async fn index_two(info: Path<Info>) -> Result<Option<IUser>> {
  use crate::models::user::schema as user;

  let some_user = IUser::find(Where(json!({
    user.handle: info.handle
  }))).await?;

  Ok(some_user)
}
```
