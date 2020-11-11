use crate::{
    error::MBError,
    MindBase,
};

// TODO 2 - move this to MBQL, and load it up with lots of stuffs
pub fn genesis(_mb: &MindBase) -> Result<(), MBError> {
    // let _words = mb.put_artifact(Text::new("English words"))?;

    Ok(())
}
