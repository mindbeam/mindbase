use crate::{
    artifact::Text,
    error::MBError,
    MindBase,
};

pub fn genesis(mb: &MindBase) -> Result<(), MBError> {
    let _words = mb.put_artifact(Text::new("English words"))?;

    Ok(())
}
