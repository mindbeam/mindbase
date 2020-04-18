use crate::{
    artifact::Text,
    error::MBError,
    MindBase,
};

// TODO 2 - move this to MBQL, and load it up with lots of stuffs
pub fn genesis(mb: &MindBase) -> Result<(), MBError> {
    // let _words = mb.put_artifact(Text::new("English words"))?;

    Ok(())
}
