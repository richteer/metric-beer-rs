use serde::{
    Serialize, Deserialize
};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Hours {
    pub open: i32,
    pub close: i32,
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
