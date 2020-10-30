use std::path::PathBuf;

use mindbase_core::MindBase;

fn run(mb: MindBase, file: PathBuf, echo: bool) {
    let path = file.as_path();
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let query = mindbase_core::mbql::Query::new(&mb, reader)?;

    if opt.echo {
        println!("Echo Output:\n");

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        query.dump(&mut handle)?;
    }

    query.apply()?;
}
