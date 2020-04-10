use std::{
    fs::File,
    io::BufReader,
};

use mindbase::MindBase;

use std::path::PathBuf;
use structopt::StructOpt;

/// Commandline tool for importing and exporting RoamResearch files for MindBase
#[derive(StructOpt, Debug)]
#[structopt(name = "cli")]
struct Opt {
    /// Path to your MindBase
    #[structopt(short, long, parse(from_os_str))]
    mindbase: PathBuf,
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

    let stdout = std::io::stdout();
    let handle = stdout.lock();

    mindbase::xport::dump_json(&mb, handle).unwrap();

    Ok(())
}
