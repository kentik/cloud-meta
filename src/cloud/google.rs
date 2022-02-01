use std::collections::HashMap;
use hyper::{Body, Method, Request};
use hyper::body::{to_bytes, Bytes};
use hyper::client::{Client, HttpConnector};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::Error;

pub struct Google {
    client:   Client<HttpConnector, Body>,
    endpoint: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    pub id:           u64,
    pub name:         String,
    pub image:        String,
    pub hostname:     String,
    pub cpu_platform: String,
    pub machine_type: String,
    pub zone:         String,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl Google {
    pub fn new(client: Client<HttpConnector, Body>) -> Self {
        let endpoint = "http://metadata.google.internal/computeMetadata/v1";
        Self {
            client:   client,
            endpoint: endpoint.to_owned(),
        }
    }

    pub async fn instance(&self) -> Result<Instance, Error> {
        let bytes = self.get("instance/", true).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn get(&self, path: &str, recursive: bool) -> Result<Bytes, Error> {
        let endpoint = format!("{}/{}?recursive={}", self.endpoint, path, recursive);

        let mut request = Request::new(Body::empty());
        *request.method_mut() = Method::GET;
        *request.uri_mut() = endpoint.try_into()?;

        let header = "Metadata-Flavor";
        let value  = "Google".as_bytes().try_into()?;
        request.headers_mut().insert(header, value);

        let response = self.client.request(request).await?;
        if !response.status().is_success() {
            return Err(response.status().into());
        }

        Ok(to_bytes(response).await?)
    }
}
