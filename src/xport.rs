use crate::{
    allegation::{
        Allegation,
        AllegationId,
    },
    artifact::{
        Artifact,
        ArtifactId,
    },
    error::Error,
    MindBase,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize)]
enum JSONLine {
    Artifact((ArtifactId, Artifact)),
    Allegation((AllegationId, Allegation)),
}

#[allow(unused)]
pub fn dump_json<T: std::io::Write>(mb: &MindBase, mut writer: T) -> Result<(), Error> {
    for result in mb.artifact_iter() {
        let (id, artifact) = result?; // we may have failed to retrieve/decode one of them
        let string = serde_json::to_writer(&mut writer, &JSONLine::Artifact((id, artifact)))?;
        writer.write(b"\n");
    }

    for result in mb.allegation_iter() {
        let (id, allegation) = result?; // we may have failed to retrieve/decode one of them
        let string = serde_json::to_writer(&mut writer, &JSONLine::Allegation((id, allegation)))?;
        writer.write(b"\n");
    }

    Ok(())
}

#[allow(unused)]
pub fn load_json<T: std::io::BufRead>(mb: &MindBase, mut reader: T) -> Result<(), Error> {
    for line in reader.lines() {
        let line: JSONLine = serde_json::from_str(&line?[..])?;

        match line {
            JSONLine::Allegation((_id, allegation)) => {
                mb.put_allegation(&allegation)?;
            },
            JSONLine::Artifact((_id, artifact)) => {
                mb.put_artifact(artifact)?;
            },
        }
    }

    Ok(())
}
