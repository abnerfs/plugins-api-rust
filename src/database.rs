use std::env;

pub fn create_pool() -> mysql::Pool {
    let connection_string = env::var("CN_STRING").expect("Failed to get env var CN_STRING");
    let pool = mysql::Pool::new(&connection_string).expect("Failed to initialize connection pool");
    pool
}
