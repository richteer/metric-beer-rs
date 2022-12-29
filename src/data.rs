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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn setup() -> Result<BreweryData, serde_json::Error> {
        // TODO: probably handle a missing local test file
        let data = std::fs::read_to_string("./testdata/beer.json").unwrap();
        serde_json::from_str(&data)
    }

    #[test]
    fn test_json_deserialize() {
        setup().unwrap();
    }

    // A brewery is considered closed if BOTH "open" and "close" are -1.
    //  It is an error for only one of "open" or "close" to be -1.
    #[test]
    fn check_open_close_match() {
        let data = setup().unwrap();

        for brew in data.breweries.iter() {
            for (day, hours) in brew.hours.iter() {
                if match (hours.open, hours.close) {
                    (-1, -1) => false,
                    (-1, _)|(_, -1) => true,
                    (_, _) => false,
                } {
                    panic!("Error in '{}', day {}: Only one of open or close is set to -1", brew.name, day)
                };
            }
        }
    }

    // Ensure all non-closed hours have the open hour before the close hour
    #[test]
    fn check_open_before_close() {
        let data = setup().unwrap();

        for brew in data.breweries.iter() {
            for (day, hours) in brew.hours.iter()
                    // Filter out closed breweries, also remove any that close at midnight.
                    // No breweries currently close after midnight.
                    // Will either need to rewrite this or remove this check if that changes.
                    .filter(|(_, h)| h.open != -1 && h.close != -1 && h.close != 0) {
                if !(hours.open < hours.close) {
                    panic!("Error in '{}' on {}: Open is after Close: {} < {}", brew.name, day, hours.open, hours.close);
                };
            }
        }
    }

    // Ensure all open/close times are within 0-23
    #[test]
    fn check_24_hour() {
        let data = setup().unwrap();

        for brew in data.breweries.iter() {
            for (day, hours) in brew.hours.iter()
                    .filter(|(_, h)| h.open != -1 && h.close != -1) {
                if !(hours.open >= 0 && hours.open < 24) {
                    panic!("Error in '{}' on {}: Open uses an invalid hour (0-23): {}", brew.name, day, hours.open);
                }
                if !(hours.close >= 0 && hours.close < 24) {
                    panic!("Error in '{}' on {}: Close uses an invalid hour (0-23): {}", brew.name, day, hours.close);
                }
            }
        }
    }

    #[test]
    fn check_days() {
        let data = setup().unwrap();

        let days: HashSet<&str> = HashSet::from_iter(vec!["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"]);

        for brew in data.breweries.iter() {
            let brew_days: HashSet<&str> = HashSet::from_iter(brew.hours.keys().map(|a| a.as_str()));

            let missing: Vec<&&str> = days.difference(&brew_days).collect();
            let extra: Vec<&&str> = brew_days.difference(&days).collect();

            if missing.len() != 0 {
                panic!("Error in '{}': Missing the following day(s): {:?}", brew.name, missing);
            }
            if extra.len() != 0 {
                panic!("Error in '{}': Extra key(s): {:?}", brew.name, extra);
            }
        }
    }

}
