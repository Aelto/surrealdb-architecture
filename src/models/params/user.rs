use surreal_simple_querybuilder::prelude::*;
use surreal_simple_querybuilder::queries::QueryBuilderInjecter;

pub enum UserParam {
  None,
  FetchMessages,
}

impl<'a> QueryBuilderInjecter<'a> for UserParam {
  fn inject(
    &self, querybuilder: surreal_simple_querybuilder::querybuilder::QueryBuilder<'a>,
  ) -> surreal_simple_querybuilder::querybuilder::QueryBuilder<'a> {
    use crate::models::user::schema::model;
    match self {
      Self::None => ().inject(querybuilder),
      Self::FetchMessages => Fetch([model.messages.as_ref()]).inject(querybuilder),
    }
  }
}
