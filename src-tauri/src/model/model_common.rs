use crate::connection::{sqlite_connection::SqliteConnection};

pub trait ModelCommon<T> {
    // T - Primary read param, e.g. id could be i32
    // Q - Struct type
    fn init_script(&self, connector: SqliteConnection);
    fn get_init_order(&self) -> u8;
    fn create(&self, connector: SqliteConnection);
    fn read(t: T, connector: SqliteConnection) -> Self;
    fn read_list(connector: SqliteConnection) -> Vec<Self> where Self: Sized;
    fn update(&self, t: T, connector: SqliteConnection);
    fn delete(t: T, connector: SqliteConnection);
}
