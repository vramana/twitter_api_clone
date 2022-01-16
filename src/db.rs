use diesel::r2d2::ConnectionManager;
use diesel::{Connection, PgConnection};
use r2d2::{Pool, PooledConnection};

pub struct Database<T>
where
    T: Connection + 'static,
{
    connection_pool: Pool<ConnectionManager<T>>,
}

impl<T> Database<T>
where
    T: Connection + 'static,
{
    pub fn new(database_url: &str) -> Self {
        Self::from_pool_builder(database_url, r2d2::Builder::default())
    }

    fn from_pool_builder(database_url: &str, builder: r2d2::Builder<ConnectionManager<T>>) -> Self {
        let manager = ConnectionManager::new(database_url);
        let connection_pool = builder
            .max_size(5)
            .build(manager)
            .expect("could not initial database pool");

        Self { connection_pool }
    }

    pub fn conn(&self) -> PooledConnection<ConnectionManager<T>> {
        self.connection_pool.get().unwrap()
    }
}

pub type PgDB = Database<PgConnection>;
