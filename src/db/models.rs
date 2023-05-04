use diesel::prelude::*;

#[derive(Queryable)]
pub struct Quote {
    pub id: u32,
    pub quoted_by: String,
    pub quote_string: String,
    pub author: String,
}
