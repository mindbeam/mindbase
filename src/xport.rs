use crate::{
    allegation::Allegation,
    artifact::Artifact,
    error::Error,
    MindBase,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize)]
enum JSONLine<'a> {
    Artifact(&'a Artifact),
    Allegation(&'a Allegation),
}

#[allow(unused)]
pub fn dump_json<T: std::io::Write>(mb: &MindBase, mut writer: T) -> Result<(), Error> {
    for maybe_artifact in mb.artifact_iter() {
        let artifact = maybe_artifact?; // we may have failed to retrieve/decode one of them
        let string = serde_json::to_writer(&mut writer, &JSONLine::Artifact(&artifact))?;
        writer.write(b"\n");
    }

    for maybe_allegation in mb.allegation_iter() {
        let allegation = maybe_allegation?; // we may have failed to retrieve/decode one of them
        let string = serde_json::to_writer(&mut writer, &JSONLine::Allegation(&allegation))?;
        writer.write(b"\n");
    }

    Ok(())
}

#[allow(unused)]
pub fn load_json<T: std::io::BufRead>(mb: &MindBase, mut reader: T) -> Result<(), Error> {
    for line in reader.lines() {
        let allegation: Allegation = serde_json::from_str(&line?[..])?;
        mb.put_allegation(&allegation)?;
    }

    Ok(())
}
