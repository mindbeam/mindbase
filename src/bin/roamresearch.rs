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

    // TODO 1 - reconstitute this from previous runs

    let mut context = RoamParseContext::new(&mb)?;

    context.parse_pages(pages)?;

    Ok(())
}

struct RoamParseContext<'a> {
    mb:                &'a MindBase,
    c_file:            Concept,
    c_page:            Concept,
    c_item:            Concept,
    c_child:           Concept,
    c_block_reference: Concept,
    uid_lookup:        HashMap<String, AllegationId>,
    uid_relations:     HashMap<String, Vec<AllegationId>>,
}
impl<'a> RoamParseContext<'a> {
    pub fn new(mb: &'a MindBase) -> Result<Self, Error> {
        // One thing that may jump out is that a `JSON dumpfile` is not a `Roam Research`
        // Fortunately, we're operating on Allegations, and not artifacts
        // So `JSON Dumpfile` is a member of the category which is _associated to_ the label `Roam Research`
        // of which there may be many. This neatly evades the "Concept Problem" (See README)

        Ok(Self { mb,
                  c_file: mb.get_ground_concept(vec![text("Organization"), text("Roam Research"), text("JSON Dumpfile")])?,
                  c_page: mb.get_ground_concept(vec![text("Organization"), text("Roam Research"), text("Page")])?,
                  c_item: mb.get_ground_concept(vec![text("Organization"), text("Roam Research"), text("Item")])?,
                  c_child: mb.get_ground_concept(vec![text("Organization"), text("Roam Research"), text("Child Of")])?,
                  c_block_reference: mb.get_ground_concept(vec![text("Organization"),
                                                                text("Roam Research"),
                                                                text("Block Reference")])?,
                  uid_lookup: HashMap::new(),
                  uid_relations: HashMap::new() })
    }

    pub fn parse_pages(&mut self, pages: Vec<Page>) -> Result<(), Error> {
        let nodes: Vec<AllegationId> = Vec::new();
        for page in pages {
            let mut relations: Vec<DataNodeRelation> = Vec::new();

            if let Some(children) = page.children {
                self.recurse_children(&mut relations, children)?;
            }

            self.mb.alledge(DataNode { node_type: self.c_page.clone(),
                                        data: page.title.into_bytes(),
                                        relations })?;
        }

        // We're alledging the datagraph, because importing the file "happened" even if the exact file had already been imported
        // before. It doesn't matter if the identitical DataGraph Artifact already existed. That will be deduplicated by the
        // artifact subsystem.
        self.mb.alledge(DataGraph { graph_type: self.c_file.clone(),
                                     bytes: 0,
                                     nodes })?;
        Ok(())
    }

    fn recurse_children(&mut self, parent_relations: &mut Vec<DataNodeRelation>, children: Vec<Item>) -> Result<(), Error> {
        // TODO 1 - use UID lookup

        use lazy_static::*;
        lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new(r"\(\(([A-Za-z0-9_-]{9})\)\)").unwrap();
        }

        for item in children {
            //"string": "((8CgwUWNKt))",
            //"string": "And a block reference to your test ((VfPEod8QQ))",
            //"string": "And an embed block reference to the same {{embed: ((VfPEod8QQ))}}",

            let mut item_relations: Vec<DataNodeRelation> = Vec::new();

            if let Some(children) = item.children {
                self.recurse_children(&mut item_relations, children)?;
            }

            // TODO 2 - materialize child nodes to represent the split terms
            if let Some(captures) = RE.captures(&item.string) {
                let terms = captures.len() - 1;
                if terms == 1 {
                    let other_uid = captures.get(1).unwrap().as_str();
                    match self.uid_lookup.get(other_uid) {
                        Some(a_item_id) => {
                            // TODO other relation types

                            println!("{}: Record Block reference to {}", item.uid, other_uid);
                            item_relations.push(DataNodeRelation { to:            a_item_id.clone(),
                                                                   relation_type: self.c_block_reference.clone(), })
                        },
                        None => {
                            println!("Warning: UID {} not found (This parser presently assumes DAG+topological parse order, \
                                      which is bad)",
                                     other_uid);
                        },
                    }
                } else {
                    println!("Captured terms <> 1 not currently supported");
                }
            }

            // Alledge the item - This should always be a single artifact / allegation, regardless if whether it is split up
            // further. Those split Artifacts/allegations would be related with a special relation type, but wouldn't replace it.
            let a_item = self.mb.alledge(DataNode { node_type: self.c_item.clone(),
                                                     data:      item.uid.clone().into_bytes(),
                                                     relations: item_relations, })?;

            // Store a copy in case anyone is pointing to us
            self.uid_lookup.insert(item.uid, a_item.id().clone());

            // Record the link to the parent
            parent_relations.push(DataNodeRelation { to:            a_item.id().clone(),
                                                     relation_type: self.c_child.clone(), });
        }
        Ok(())
    }
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
