use chrono::*;
use uuid::*;

#[derive(Debug, Queryable)]
pub struct Talk {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub kind: String,
    pub happens_on: NaiveDateTime,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}
