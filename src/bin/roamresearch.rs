// TODO 2 - import RoamResearch dump files
// TODO 3 - export RoamResearch dump files
use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};

fn main() -> Result<(), std::io::Error> {
    use std::{
        fs::File,
        path::Path,
    };

    let path = Path::new("roam-test-dump-1.json");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let _pages: Vec<Page> = serde_json::from_reader(&file)?;

    Ok(())
}

// Consider writing a stateful serializer/deserializer so we don't have to buffer the whole JSON file in memory
// #[derive(Serialize, Deserialize, PartialEq, Debug)]
// struct Space {
//     #[serde(flatten)]
//     pages: Vec<Page>,
// }

// use chrono::serde::ts_seconds::{
//     deserialize as from_ts,
//     serialize as to_ts,
// };

// pub fn deserialize<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
//     where D: serde::de::Deserializer<'de>
// {
//     Ok(d.deserialize_i64(SecondsTimestampVisitor)?)
// }

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Page {
    title:      String,
    #[serde(default)]
    children:   Option<Vec<Item>>,
    #[serde(rename = "edit-time", default)]
    edit_time:  Option<u64>, // DateTime<Utc>,
    #[serde(rename = "edit-email")]
    edit_email: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Item {
    string:       String,
    #[serde(rename = "create-email", default)]
    create_email: Option<String>,
    #[serde(rename = "create-time", default)] // deserialize_with = "from_ts",
    create_time:  Option<u64>, // DateTime<Utc>>,
    uid:          String,
    #[serde(rename = "edit-time", default)]
    edit_time:    Option<u64>, // DateTime<Utc>,
    #[serde(rename = "edit-email")]
    edit_email:   String,
    #[serde(default)]
    children:     Option<Vec<Item>>,
}
