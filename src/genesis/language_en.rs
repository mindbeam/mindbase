use crate::{
    artifact::FlatText,
    error::Error,
    MindBase,
};

pub fn genesis(mb: &MindBase) -> Result<(), Error> {
    let _words = mb.assert_artifact(FlatText::new("English words".to_string()))?;

    Ok(())
}
