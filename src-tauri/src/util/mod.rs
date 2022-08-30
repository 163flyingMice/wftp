use chrono::{TimeZone, Utc};
use snowflake::ProcessUniqueId;

pub fn get_snow_id() -> String {
    ProcessUniqueId::new().to_string()
}

pub fn get_format_time(ms: i64) -> String {
    let dt = Utc.timestamp(ms, 0);
    format!("{}", dt.format("%Y/%m/%d %H:%M:%S"))
}
