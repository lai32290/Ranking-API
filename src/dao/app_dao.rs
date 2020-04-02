use postgres::{NoTls};
use r2d2_postgres::{PostgresConnectionManager, r2d2};
use r2d2::Pool;

pub struct AppDao{
    pool: Pool<PostgresConnectionManager<NoTls>>
}

impl AppDao {
    pub fn new(pool :Pool<PostgresConnectionManager<NoTls>>) -> AppDao{
        AppDao{pool}
    }

    pub fn exists(&self, name: String) -> bool{
        return match self.pool.clone().get().unwrap().query("select count() from app where name = $1;", &[&name]) {
            Ok(rows) => {
                match rows.first() {
                    Some(row) => {
                        let count: i32 = row.get(0);
                        if count > 0 {
                            true
                        } else {
                            false
                        }
                    }
                    _ => { false }
                }
            }
            _ => { false }
        }
    }

    pub fn create(&self, name: String) -> Option<String>{
        return match self.pool.clone().get().unwrap().query("insert into app (name) values ($1) returning id::text;",&[&name]) {
            Ok(rows) => {
                match rows.first() {
                    None => {None},
                    Some(row) => {
                        return Some(row.get(0));
                    },
                }
            },
            Err(_) => {None},
        }
    }
}