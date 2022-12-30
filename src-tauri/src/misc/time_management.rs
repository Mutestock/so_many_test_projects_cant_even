use chrono::{Local, NaiveDateTime};
use rusqlite::Row;

lazy_static::lazy_static! {
    pub static ref DATE_TIME_FORMAT: String =  "%d-%m-%Y %H:%M:%S".to_owned();
}

pub trait NaiveDateTimeExtension {
    fn from_row(row: &Row, row_index: usize) -> NaiveDateTime;
    fn to_format(&self) -> String;
    fn now() -> NaiveDateTime;
}

impl NaiveDateTimeExtension for NaiveDateTime {
    fn from_row(row: &Row, row_index: usize) -> NaiveDateTime {
        let res: String = row.get(row_index).unwrap();
        let res = res.as_str();
        NaiveDateTime::parse_from_str(res, &DATE_TIME_FORMAT)
            .expect("Parse error on from_row in time management")
    }

    fn to_format(&self) -> String {
        self.format(&DATE_TIME_FORMAT).to_string()
    }
    fn now() -> NaiveDateTime {
        NaiveDateTime::parse_from_str(
            &Local::now().format(&DATE_TIME_FORMAT).to_string(),
            &DATE_TIME_FORMAT,
        )
        .expect("Parse error on 'now' time management")
    }
}
