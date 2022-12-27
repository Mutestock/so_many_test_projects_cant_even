use chrono::{Datelike, Local, NaiveDateTime};
use mindmap::misc::time_management::NaiveDateTimeExtension;

#[test]
fn test_parse_dates() {
    let naive_method = NaiveDateTime::now();
    let naive_manual = Local::now().naive_local();
    assert_eq!(
        naive_method.num_days_from_ce(),
        naive_manual.num_days_from_ce()
    );
}
