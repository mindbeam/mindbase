use crate::{
    artifact::FlatText,
    error::MBError,
    MindBase,
};

pub fn genesis(mb: &MindBase) -> Result<(), MBError> {
    let _words = mb.put_artifact(FlatText::new("English words"))?;

    Ok(())
}
