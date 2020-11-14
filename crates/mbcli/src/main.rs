use std::{fs::File, io::BufReader};

use mindbase_core::*;
use mindbase_crypto::*;
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
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Create, login, and manage local agents
    Auth {
        #[structopt(subcommand)]
        cmd: crate::subcommand::auth::Command,
    },
    /// Import a .mbql file
    Import {
        /// Echo the parsed MBQL back to the display
        #[structopt(long)]
        echo: bool,

        #[structopt(parse(from_os_str))]
        file: PathBuf,
    },
    /// Export a .mbql file
    Export {
        #[structopt(parse(from_os_str))]
        file: PathBuf,
    },

    /// Run the Mindbase REPL
    REPL,
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
    let homedir = dirs::home_dir().expect("HOME directory environment variable is required");
    let cwd = std::env::current_dir().unwrap();
    let path = match &opt.mindbase {
        Some(path) => path,
        None => cwd.as_path(),
    };

    println!("Loading database in {}", path.display());

    let mb = Service::new(SledStore::open(path)?)?;
    let keymanager = KeyManager::new(SledAdapter::open(homedir.as_path())?);
    match opt.cmd {
        Command::Auth { cmd } => crate::subcommand::auth::run(mb, keymanager, cmd)?,
        Command::Import { echo, file } => crate::subcommand::import::run(mb, keymanager, file, echo)?,
        Command::Export { file } => crate::subcommand::export::run(mb, keymanager, file)?,
        Command::REPL => crate::subcommand::repl::run(mb, keymanager)?,
    }

    // let agent = mb.default_agent().unwrap();
    // println!("Using Agent {}", agent);

    Ok(())
}
