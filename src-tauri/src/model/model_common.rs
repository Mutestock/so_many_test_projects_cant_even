

pub trait ModelCommon<T> {
    // T - Primary read param, e.g. id could be i32
    fn init_script(connection: &rusqlite::Connection) -> Result<(), rusqlite::Error>;
    fn create(&self, connection: &rusqlite::Connection) -> Result<(), rusqlite::Error>;
    fn read(t: T, connection: &rusqlite::Connection) -> Result<Self, rusqlite::Error>
    where
        Self: Sized;
    fn read_list(connection: &rusqlite::Connection) -> Vec<Self>
    where
        Self: Sized;
    fn update(&self, t: T, connection: &rusqlite::Connection);
    fn delete(t: T, connection: &rusqlite::Connection);
}
