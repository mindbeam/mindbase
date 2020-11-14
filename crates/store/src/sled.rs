use sled::IVec;
pub struct SledStore {}

impl SledStore {
    pub fn open_temp() -> Result<Self, Error> {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmpdirpath = tmpdir.path();

        Self::open(tmpdirpath)
    }

    #[allow(dead_code)]
    pub fn open(basedir: &std::path::Path) -> Result<Self, Error> {
        let pathbuf = basedir.join(format!("./mindbase.sled"));

        let db = sled::open(pathbuf.as_path())?;

        let artifacts = db.open_tree("artifacts")?;
        let allegations = db.open_tree("allegations")?;
        let atoms_by_artifact_agent = db.open_tree("allegation_rev")?;
        // let analogy_rev = db.open_tree("allegation_rev")?;

        // Both of these are &k[..] / Vec<sorted u8;16 chunks>
        atoms_by_artifact_agent.set_merge_operator(merge_16byte_list);
        // analogy_rev.set_merge_operator(merge_16byte_list);

        // let default_agent = _default_agent(&my_agents)?;
        let _known_agents = db.open_tree("known_agents")?;

        let me = MindBase {
            allegations,
            // my_agents,
            artifacts,
            _known_agents,
            atoms_by_artifact_agent,
            // analogy_rev,
            // default_agent,
        };

        // me.genesis()?;

        Ok(me)
    }
}
