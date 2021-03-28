use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool };

pub fn establish_connection() -> Pool<ConnectionManager<SqliteConnection>> {
    let database_url = "sample.db";
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().max_size(4).build(manager).expect("Failed to create pool")
}
