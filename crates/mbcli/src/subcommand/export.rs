use std::{fs::File, path::PathBuf};

use keyplace::KeyManager;
use mindbase_core::MindBase;

pub(crate) fn run(mb: MindBase, keymanager: KeyManager, file: PathBuf) -> Result<(), std::io::Error> {
    let path = file.as_path();
    let display = path.display();
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    mindbase_core::xport::dump_json(&mb, file).unwrap();

    Ok(())
}
