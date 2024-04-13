use diesel::{self, prelude::*, result::QueryResult};
use serde::Serialize;

use crate::schema::entries;
use crate::schema::entries::dsl::entries as all_entries;

#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
#[table_name = "entries"]
pub struct Entry {
    pub id: Option<i32>,
    pub body: String,
}

impl Entry {
    pub fn all(conn: &SqliteConnection) -> QueryResult<Vec<Entry>> {
        all_entries.order(entries::id.desc()).load::<Entry>(conn)
    }

    pub fn add(entry: Entry, conn: &SqliteConnection) -> QueryResult<usize> {
        let p = Entry {
            id: None,
            body: entry.body,
        };
        diesel::insert_into(entries::table).values(&p).execute(conn)
    }
}
