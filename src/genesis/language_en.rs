use crate::{
    artifact::FlatText,
    error::Error,
    MindBase,
};

pub fn genesis(mb: &MindBase) -> Result<(), Error> {
    let _words = mb.put_artifact(FlatText::new("English words"))?;

    Ok(())
}
