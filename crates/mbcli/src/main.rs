use std::{fs::File, io::BufReader};

use mindbase_core::MindBase;
use mindbase_crypto::KeyManager;
use std::path::PathBuf;
use structopt::StructOpt;

mod subcommand;

/// Commandline tool for importing and exporting RoamResearch files for MindBase
#[derive(StructOpt, Debug)]
#[structopt(name = "mbcli")]
struct Opt {
    /// Path to your MindBase storage
    #[structopt(short, long, parse(from_os_str))]
    mindbase: Option<PathBuf>,

    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt, Debug)]
enum Command {
    Auth {
        #[structopt(subcommand)]
        cmd: crate::auth::Command,
    },
    Import {
        /// Echo the parsed MBQL back to the display
        #[structopt(long)]
        echo: bool,

        #[structopt(parse(from_os_str))]
        file: PathBuf,
    },
    Export {
        #[structopt(parse(from_os_str))]
        file: PathBuf,
    },
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
    let path = match &opt.mindbase {
        Some(path) => path,
        None => std::env::current_dir().unwrap(),
    };

    println!("Loading database in {}", path.as_path().display());

    let mb = MindBase::open(path);
    let keymanager = KeyManager::new();
    match opt.cmd {
        Command::Auth => crate::subcommand::auth::run(mb, keymanager),
        Command::Import { echo, file } => crate::subcommand::import::run(mb, keymanager, file, echo),
        Command::Export { file } => crate::subcommand::export::run(mb, keymanager, file),
    }

    // let agent = mb.default_agent().unwrap();
    // println!("Using Agent {}", agent);

    Ok(())
}
