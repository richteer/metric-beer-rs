use serde::{
    Serialize, Deserialize
};
use std::{collections::{
    HashMap,
    HashSet,
}, fmt::Display};

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

// TODO: This is a little awkward, maybe just store the formatted error string?
pub enum BreweryError {
    OpenCloseMismatch(String, String),
    OpenBeforeClose(String, String, i32, i32),
    BadHourOpen(String, String, i32),
    BadHourClose(String, String, i32),
    MissingDays(String, String),
    ExtraKeys(String, String),
}

impl Display for BreweryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BreweryError::OpenCloseMismatch(name, day)
                => format!("Error in '{}', day {}: Only one of open or close is set to -1", name, day),
            BreweryError::OpenBeforeClose(name, day, open, close)
                => format!("Error in '{}' on {}: Open is after Close: {} < {}", name, day, open, close),
            BreweryError::BadHourOpen(name, day, open)
                => format!("Error in '{}' on {}: Open uses an invalid hour (0-23): {}", name, day, open),
            BreweryError::BadHourClose(name, day, close)
                => format!("Error in '{}' on {}: Open uses an invalid hour (0-23): {}", name, day, close),
            BreweryError::MissingDays(name, missing)
                => format!("Error in '{}': Missing the following day(s): {}", name, missing),
            BreweryError::ExtraKeys(name, extra)
                => format!("Error in '{}': Extra key(s): {:?}", name, extra),
        }.as_str())
    }
}


impl BreweryData {
    #[allow(dead_code)]
    pub fn validate(&self) -> Result<(), BreweryError> {
        self.check_open_before_close()?;
        self.check_open_close_match()?;
        self.check_24_hour()?;
        self.check_days()?;
        Ok(())
    }

    fn check_open_close_match(&self) -> Result<(), BreweryError> {
        for brew in self.breweries.iter() {
            for (day, hours) in brew.hours.iter() {
                if match (hours.open, hours.close) {
                    (-1, -1) => false,
                    (-1, _)|(_, -1) => true,
                    (_, _) => false,
                } {
                    return Err(BreweryError::OpenCloseMismatch(brew.name.clone(), day.clone()));
                };
            }
        }
        Ok(())
    }

    // Ensure all non-closed hours have the open hour before the close hour
    fn check_open_before_close(&self) -> Result<(), BreweryError> {
        for brew in self.breweries.iter() {
            for (day, hours) in brew.hours.iter()
                    // Filter out closed breweries, also remove any that close at midnight.
                    // No breweries currently close after midnight.
                    // Will either need to rewrite this or remove this check if that changes.
                    .filter(|(_, h)| h.open != -1 && h.close != -1 && h.close != 0) {
                if !(hours.open < hours.close) {
                    return Err(BreweryError::OpenBeforeClose(brew.name.clone(), day.clone(), hours.open, hours.close));
                };
            }
        }
        Ok(())
    }

    // Ensure all open/close times are within 0-23
    fn check_24_hour(&self) -> Result<(), BreweryError> {
        for brew in self.breweries.iter() {
            for (day, hours) in brew.hours.iter()
                    .filter(|(_, h)| h.open != -1 && h.close != -1) {
                if !(hours.open >= 0 && hours.open < 24) {
                    return Err(BreweryError::BadHourOpen(brew.name.clone(), day.clone(), hours.open));
                }
                if !(hours.close >= 0 && hours.close < 24) {
                    return Err(BreweryError::BadHourClose(brew.name.clone(), day.clone(), hours.close));
                }
            }
        }
        Ok(())
    }

    fn check_days(&self) -> Result<(), BreweryError> {
        let days: HashSet<&str> = HashSet::from_iter(vec!["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"]);

        for brew in self.breweries.iter() {
            let brew_days: HashSet<&str> = HashSet::from_iter(brew.hours.keys().map(|a| a.as_str()));

            let missing: Vec<&&str> = days.difference(&brew_days).collect();
            let extra: Vec<&&str> = brew_days.difference(&days).collect();

            if missing.len() != 0 {
                return Err(BreweryError::MissingDays(brew.name.clone(), format!("{:?}", missing)));
            }
            if extra.len() != 0 {
                return Err(BreweryError::ExtraKeys(brew.name.clone(), format!("{:?}", extra)));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
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

        if let Err(e) = data.check_open_close_match() {
            panic!("{e}");
        };
    }

    // Ensure all non-closed hours have the open hour before the close hour
    #[test]
    fn check_open_before_close() {
        let data = setup().unwrap();

        if let Err(e) = data.check_open_before_close() {
            panic!("{e}");
        };
    }

    // Ensure all open/close times are within 0-23
    #[test]
    fn check_24_hour() {
        let data = setup().unwrap();

        if let Err(e) = data.check_24_hour() {
            panic!("{e}");
        };
    }

    #[test]
    fn check_days() {
        let data = setup().unwrap();

        if let Err(e) = data.check_days() {
            panic!("{e}");
        };
    }

}
