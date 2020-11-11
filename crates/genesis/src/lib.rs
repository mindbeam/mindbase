pub mod language_en;
// pub mod pos;

/// Include whatever batteries we want to include
pub fn genesis(mb: &Mindbase) -> Result<(), MBError> {
    // TODO 2 - use the genesis Agent, not ours
    // TODO 2 - make this NoOp when an exact artifact exists
    //    Other entity types should NOT deduplicate, only artifacts. This means they have to be hashed, but other entity types
    // should be enumerated
    crate::language_en::genesis(mb)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
