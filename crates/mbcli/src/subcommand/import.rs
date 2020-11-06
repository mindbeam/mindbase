use std::{fs::File, io::BufReader, path::PathBuf};

use mindbase_core::MindBase;
use mindbase_crypto::KeyManager;

pub(crate) fn run(mb: MindBase, keymanager: KeyManager, file: PathBuf, echo: bool) -> Result<(), std::io::Error> {
    let path = file.as_path();
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let query = mindbase_core::mbql::Query::new(&mb, reader)?;

    if echo {
        println!("Echo Output:\n");

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        query.dump(&mut handle)?;
    }

    query.apply()?;

    Ok(())
}