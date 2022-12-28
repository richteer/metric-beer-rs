use serde::{
    Serialize, Deserialize
};
use chrono::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Hours {
    pub open: i32,
    pub close: i32,
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


#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Brewery {
    pub name: String,
    pub hours: HashMap<String, Hours>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BreweryData {
    #[serde(rename(deserialize = "Breweries"))]
    pub breweries: Vec<Brewery>,
    #[serde(rename(deserialize = "Crawl"))]
    pub crawl: Vec<String>,
}
