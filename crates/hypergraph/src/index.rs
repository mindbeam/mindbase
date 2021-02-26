use typenum::Unsigned;

pub fn merge_byte_list<U: Unsigned>(
    _key: &[u8],               // the key being merged
    last_bytes: Option<&[u8]>, // the previous value, if one existed
    op_bytes: &[u8],           /* the new bytes being merged in */
) -> Option<Vec<u8>> {
    // set the new value, return None to delete

    use inverted_index_util::entity_list::{insert_entity_immut, ImmutResult};

    Some(match last_bytes {
        Some(prior) => match insert_entity_immut::<U>(prior, op_bytes) {
            ImmutResult::Changed(newvec) => newvec,
            ImmutResult::Unchanged => prior.to_vec(),
        },
        None => op_bytes.to_vec(),
    })
}
