// TODO 2 - import RoamResearch dump files
// TODO 3 - export RoamResearch dump files
use chrono::{
    DateTime,
    Utc,
};
use concept::Concept;
use mindbase::{
    analogy::Analogy,
    artifact::{
        text,
        DataGraph,
        DataNode,
        DataNodeRelation,
    },
    *,
};
use serde::{
    Deserialize,
    Serialize,
};
use std::collections::HashMap;

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

    let pages: Vec<Page> = serde_json::from_reader(&file)?;

    let dir = std::env::current_dir().unwrap();
    println!("Loading database in {}", dir.as_path().display());
    let mb = MindBase::open(&dir.as_path()).unwrap();

    // One thing that may jump out is that a `JSON dumpfile` is not a `Roam Research`
    // Fortunately, we're operating on Allegations, and not artifacts
    // So `JSON Dumpfile` is a member of the category which is _associated to_ the label `Roam Research`
    // of which there may be many. This neatly evades the "Concept Problem" (See README)
    let c_file = mb.get_ground_concept(vec![text("Organization"), text("Roam Research"), text("JSON Dumpfile")])?;
    let c_page = mb.get_ground_concept(vec![text("Organization"), text("Roam Research"), text("Page")])?;
    let c_item = mb.get_ground_concept(vec![text("Organization"), text("Roam Research"), text("Item")])?;
    let c_child = mb.get_ground_concept(vec![text("Organization"), text("Roam Research"), text("Child Of")])?;

    // TODO 1 - reconstitute this from previous runs
    let mut uid_lookup = UidLookup::new();

    let nodes: Vec<AllegationId> = Vec::new();
    for page in pages {
        let mut relations: Vec<DataNodeRelation> = Vec::new();

        if let Some(children) = page.children {
            recurse_children(&mb, &mut relations, &c_item, &c_child, &mut uid_lookup, children)?;
        }

        mb.alledge(DataNode { node_type: c_page.clone(),
                              data: page.title.into_bytes(),
                              relations })?;
    }

    // We're alledging the datagraph, because importing the file "happened" even if the exact file had already been imported
    // before. It doesn't matter if the identitical DataGraph Artifact already existed. That will be deduplicated by the artifact
    // subsystem.
    mb.alledge(DataGraph { graph_type: c_file,
                           bytes: 0,
                           nodes })?;

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

fn recurse_children(mb: &MindBase, relations: &mut Vec<DataNodeRelation>, c_item: &Concept, c_child: &Concept,
                    uid_lookup: &mut UidLookup, children: Vec<Item>)
                    -> Result<(), Error> {
    // TODO 1 - use UID lookup

    for item in children {
        let mut child_relations: Vec<DataNodeRelation> = Vec::new();

        if let Some(children) = item.children {
            recurse_children(&mb, &mut child_relations, &c_item, c_child, uid_lookup, children)?;
        }

        let item_id = mb.alledge(DataNode { node_type: c_item.clone(),
                                            data:      item.uid.into_bytes(),
                                            relations: child_relations, })?;

        relations.push(DataNodeRelation { to:            item_id.id().clone(),
                                          relation_type: c_child.clone(), });
    }
    Ok(())
}

struct UidLookup {
    map: HashMap<String, Vec<AllegationId>>,
}
impl UidLookup {
    pub fn new() -> Self {
        UidLookup { map: HashMap::new() }
    }
}

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
