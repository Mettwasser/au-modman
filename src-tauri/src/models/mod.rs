use surrealdb::sql::Thing;

pub mod modification;
pub mod profile;

pub trait GetThing {
    fn get_thing(&self) -> Thing;
}
