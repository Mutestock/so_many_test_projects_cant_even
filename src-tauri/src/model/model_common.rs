use rusqlite::Row;

pub trait ModelCommon<T> {
    // T - Primary read param, e.g. id could be i32
    fn init_script(connection: &rusqlite::Connection) -> Result<(), rusqlite::Error>;
    fn create(&self, connection: &rusqlite::Connection) -> Result<(), rusqlite::Error>;
    fn read(t: T, connection: &rusqlite::Connection) -> Result<Option<Self>, rusqlite::Error>
    where
        Self: Sized;
    fn read_list(connection: &rusqlite::Connection) -> Result<Vec<Self>, rusqlite::Error>
    where
        Self: Sized;
    fn update(&self, t: T, connection: &rusqlite::Connection) -> Result<(), rusqlite::Error>;
    fn delete(t: T, connection: &rusqlite::Connection) -> Result<(), rusqlite::Error>;

    // This needs to go
    fn from_row(p_key: Option<&str>, row: &Row) -> Result<Self, rusqlite::Error>
    where
        Self: Sized;
}
