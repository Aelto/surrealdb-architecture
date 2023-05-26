mod id;
pub use id::*;

pub type Foreign<T> = surreal_simple_querybuilder::foreign_key::ForeignKey<T, Id>;

pub type ForeignVec<T> = surreal_simple_querybuilder::foreign_key::ForeignKey<Vec<T>, Vec<Id>>;
