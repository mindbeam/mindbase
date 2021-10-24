use tonic::{transport::Server, Request, Response, Status};

use proto::entities_server::{Entities, EntitiesServer};
use proto::{PutEntityReply, PutEntityRequest};
use tokio;

pub mod proto {
    tonic::include_proto!("mindbase_proto"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyService {}

#[tonic::async_trait]
impl Entities for MyService {
    async fn put_entity(
        &self,
        request: Request<PutEntityRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<PutEntityReply>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let id = "test".to_string();
        // TODO add storage engine insertion guts here
        let reply = PutEntityReply { id };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = MyService::default();

    Server::builder()
        .add_service(EntitiesServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
