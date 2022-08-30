use chrono::{TimeZone, Utc};
use snowflake::ProcessUniqueId;

pub fn get_snow_id() -> String {
    ProcessUniqueId::new().to_string()
}

pub fn get_format_time(ms: i64) -> String {
    let dt = Utc.timestamp(ms, 0);
    format!("{}", dt.format("%Y/%m/%d %H:%M:%S"))
}

pub fn get_format_perm(perm: u32) -> String {
    let temp_perm = format!("{:o}", perm);
    let perm_g = temp_perm[(temp_perm.len() - 1)..].parse::<usize>().unwrap();
    let perm_o = temp_perm[(temp_perm.len() - 2)..(temp_perm.len() - 1)]
        .parse::<usize>()
        .unwrap();
    let perm_other = temp_perm[(temp_perm.len() - 3)..(temp_perm.len() - 2)]
        .parse::<usize>()
        .unwrap();
    format!(
        "{}{}{}",
        format_perm(perm_g),
        format_perm(perm_o),
        format_perm(perm_other)
    )
}

fn format_perm(param: usize) -> String {
    let mut result: String = String::from("");
    if param / 4 != 0 {
        result += "r";
    } else {
        result += "-"
    }
    if (param % 4) / 2 != 0 {
        result += "w";
    } else {
        result += "-";
    }
    if (param % 4) % 2 != 0 {
        result += "x";
    } else {
        result += "-";
    }
    result
}
