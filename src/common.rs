use std::fmt::Display;
use chrono::prelude::*;

pub enum OpenStatus {
    Open,
    Closed,
    OpenLater(String),
    Error,
}

pub trait ToTodayHour {
    fn to_today(&self, now: &DateTime<Local>) -> Option<DateTime<Local>>;
}

impl ToTodayHour for i32 {
    fn to_today(&self, now: &DateTime<Local>) -> Option<DateTime<Local>> {
        now.timezone().with_ymd_and_hms(
            now.year(),
            now.month(),
            now.day(),
            *self as u32,
            0, // Data format currently assumes opening times on the hour, this should probably be adjusted
            0,
        ).single()
    }
}


impl Display for OpenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = match self {
            OpenStatus::Open => "OPEN".to_string(),
            OpenStatus::Closed => "CLOSED".to_string(),
            OpenStatus::OpenLater(o) => o.to_string(),
            OpenStatus::Error => "ERR".to_string(),
        };
        f.write_str(data.as_str())
    }
}

// TODO: probably just use a time formatting crate
pub fn format_time(date: Option<DateTime<Local>>, ampm: bool) -> String {
    let ampm_format = "%I:%M %p";
    let hr24_format = "%H:%M";

    match (date, ampm) {
        (Some(date), true) => date.format(ampm_format).to_string(),
        (Some(date), false) => date.format(hr24_format).to_string(),
        (None, _) => "CLOSED".to_string(),
    }
}

pub fn format_upcoming(now: &DateTime<Local>, open: Option<&DateTime<Local>>, close: Option<&DateTime<Local>>) -> OpenStatus {
    match (open, close) {
        (Some(o), Some(c)) => {
            match (now.cmp(o), now.cmp(c)) {
                (std::cmp::Ordering::Less, _) => OpenStatus::OpenLater(format!("{} min", (*o - *now).num_minutes())), // TODO: consider an display option for hours/mins?
                (_, std::cmp::Ordering::Less) | (_, std::cmp::Ordering::Equal)  => OpenStatus::Open,
                (_, std::cmp::Ordering::Greater) => OpenStatus::Closed,
            }
        }
        (None, None) => OpenStatus::Closed,
        (Some(_), None) | (None, Some(_)) => {
            log::error!("data is probably wrong, open and close aren't both -1 (closed). open = {open:?}, close = {close:?}");
            OpenStatus::Error
        },
    }
}

pub fn format_hours(open: Option<DateTime<Local>>, close: Option<DateTime<Local>>, ampm: bool) -> String {
    match (open, close) {
        (Some(_), Some(_)) => format!("{} - {}", format_time(open, ampm), format_time(close, ampm)),
        (None, None) => "CLOSED".to_string(),
        (Some(_), None) | (None, Some(_)) => {
            log::error!("data is probably wrong, open and close aren't both -1 (closed). open = {open:?}, close = {close:?}");
            "ERROR".to_string()
        },
    }
}