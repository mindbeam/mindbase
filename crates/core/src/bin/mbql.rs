use std::{
    fs::File,
    io::BufReader,
};

use mindbase_core::MindBase;

use std::path::PathBuf;
use structopt::StructOpt;

/// Commandline tool for importing and exporting RoamResearch files for MindBase
#[derive(StructOpt, Debug)]
#[structopt(name = "cli")]
struct Opt {
    /// Path to your MindBase
    #[structopt(short, long, parse(from_os_str))]
    mindbase: PathBuf,

    /// Import MBQL file into MindBase
    #[structopt(short, long, parse(from_os_str))]
    import: Option<PathBuf>,

    /// Export Mindbase contents into MBQL file
    #[structopt(short, long, parse(from_os_str))]
    export: Option<PathBuf>,

    /// Echo the parsed MBQL back to the display
    #[structopt(long)]
    echo: bool,
}
fn main() -> Result<(), std::io::Error> {
    let opt = Opt::from_args();

    if let Err(e) = run(opt) {
        println!("MBQL Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

fn run(opt: Opt) -> Result<(), std::io::Error> {
    let mb = MindBase::open(&opt.mindbase.as_path()).unwrap();

    if let Some(file) = opt.import {
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
    } else if let Some(_file) = opt.export {
        unimplemented!()
        // TODO
    }
    Ok(())
}
