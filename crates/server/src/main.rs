use std::net::SocketAddr;
use std::path::PathBuf;

use mindbase_artifact::Artifact;
use tonic::{transport::Server, Request, Response, Status};

use proto::entities_server::{Entities, EntitiesServer};
use proto::{PutEntityReply, PutEntityRequest};
use tokio;

pub mod proto {
    tonic::include_proto!("mindbase_proto"); // The string specified here must match the proto package name
}

use structopt::StructOpt;

/// Commandline tool for importing and exporting RoamResearch files for MindBase
#[derive(StructOpt, Debug)]
#[structopt(name = "mindbase-server")]
struct Opt {
    /// Path to your MindBase storage
    #[structopt(short, long, parse(from_os_str))]
    mindbase: Option<PathBuf>,

    // /// Verbose mode (-v, -vv, -vvv, etc.)
    // #[structopt(short, long, parse(from_occurrences))]
    // verbose: u8,
    /// Files to process
    #[structopt(parse(try_from_str), default_value = "[::1]:50051")]
    bind_to: SocketAddr,
}

use mindbase_hypergraph::Hypergraph;
use toboggan_kv::adapter::SledAdapter;

#[derive(Debug, Default)]
pub struct MyService {
    hg: Hypergraph<SledStore, Artifact<String>>,
}

#[tonic::async_trait]
impl Entities for MyService {
    async fn put_entity(
        &self,
        request: Request<PutEntityRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<PutEntityReply>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        // self.hg.
        let id = "test".to_string();
        // TODO add storage engine insertion guts here
        let reply = PutEntityReply { id };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let cwd = std::env::current_dir().unwrap();

    let path = match &opt.mindbase {
        Some(path) => path,
        None => cwd.as_path(),
    };

    println!("Loading database in {}", path.display());

    let hg: Hypergraph<_, Artifact<String>> = Hypergraph::open(SledStore::open(path)?)?;
    let service = MyService { hg };

    Server::builder()
        .add_service(EntitiesServer::new(service))
        .serve(opt.bind_to)
        .await?;

    Ok(())
}
