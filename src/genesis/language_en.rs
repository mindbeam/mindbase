use crate::{
    entity::{
        ArtifactKind,
        FlatText,
    },
    error::Error,
    MindBase,
};

pub fn genesis(mb: &MindBase) -> Result<(), Error> {
    let _words = mb.assert_artifact(ArtifactKind::FlatText(FlatText { text: "English words".to_string(), }))?;

    Ok(())
}
