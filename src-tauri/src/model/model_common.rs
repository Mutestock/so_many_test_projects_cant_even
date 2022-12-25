use chrono::{Local, DateTime, TimeZone, FixedOffset};
use rusqlite::Row;

use crate::connection::{sqlite_connection::SqliteConnection};


lazy_static!{
    static ref DATE_TIME_OFFSET: FixedOffset = *Local::now().offset();
}


pub trait ModelCommon<T> {
    // T - Primary read param, e.g. id could be i32
    // Q - Struct type
    fn init_script( connector: SqliteConnection ) -> Result<(), rusqlite::Error>;
    fn create(&self, connector: SqliteConnection) -> Result<(), rusqlite::Error>;
    fn read(t: T, connector: SqliteConnection) -> Result<Self, rusqlite::Error> where Self: Sized;
    fn read_list(connector: SqliteConnection) -> Vec<Self> where Self: Sized;
    fn update(&self, t: T, connector: SqliteConnection);
    fn delete(t: T, connector: SqliteConnection);
}


pub trait DateTimeRusqlite<T: TimeZone>{
    fn from_row(row: &Row, row_index: usize) -> DateTime<T>;
}

impl DateTimeRusqlite<Local> for DateTime<Local>{
    fn from_row(row: &Row, row_index: usize) -> DateTime<Local> { 
        let res: String = row.get(row_index).unwrap();
        let res = res.as_str();
        Local.datetime_from_str(res, &DATE_TIME_OFFSET.to_string()).unwrap().to_owned()
    }
}