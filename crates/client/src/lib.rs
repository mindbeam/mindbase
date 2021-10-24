pub use tonic::transport::Endpoint;

use proto::entities_client::EntitiesClient;
use proto::{PutEntityReply, PutEntityRequest};

pub mod proto {
    tonic::include_proto!("mindbase_proto"); // The string specified here must match the proto package name
}

#[derive(Clone)]
pub struct Client {
    ec: EntitiesClient<tonic::transport::Channel>,
}

impl Client {
    pub async fn connect<E: Into<Endpoint>>(endpoint: E) -> Result<Self, Box<dyn std::error::Error>> {
        let ec = EntitiesClient::connect(endpoint).await?;
        Ok(Client { ec })
    }
    pub async fn put_entity(&mut self, req: PutEntityRequest) -> Result<PutEntityReply, tonic::Status> {
        let request = tonic::Request::new(req);
        let response = self.ec.put_entity(request).await?;
        Ok(response.into_inner())
    }
}

impl Into<proto::PropertyValue> for String {
    fn into(self) -> proto::PropertyValue {
        proto::PropertyValue {
            value: Some(proto::property_value::Value::String(self)),
        }
    }
}
impl Into<proto::PropertyValue> for u32 {
    fn into(self) -> proto::PropertyValue {
        proto::PropertyValue {
            value: Some(proto::property_value::Value::Uint32(self)),
        }
    }
}

impl Into<proto::PropertyValue> for Vec<u8> {
    fn into(self) -> proto::PropertyValue {
        proto::PropertyValue {
            value: Some(proto::property_value::Value::Bytes(self)),
        }
    }
}
