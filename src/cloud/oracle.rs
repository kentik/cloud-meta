use std::collections::HashMap;
use hyper::{Body, Method, Request};
use hyper::body::{to_bytes, Bytes};
use hyper::client::{Client, HttpConnector};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::Error;

pub struct Oracle {
    client:   Client<HttpConnector, Body>,
    endpoint: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    pub id:           String,
    pub display_name: String,
    pub image:        String,
    pub hostname:     String,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl Oracle {
    pub fn new(client: Client<HttpConnector, Body>) -> Self {
        let endpoint = "http://169.254.169.254/opc/v2";
        Self {
            client:   client,
            endpoint: endpoint.to_owned(),
        }
    }

    pub async fn instance(&self) -> Result<Instance, Error> {
        let bytes = self.get("instance").await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn get(&self, path: &str) -> Result<Bytes, Error> {
        let endpoint = format!("{}/{}", self.endpoint, path);

        let mut request = Request::new(Body::empty());
        *request.method_mut() = Method::GET;
        *request.uri_mut() = endpoint.try_into()?;

        let header = "Authorization";
        let value  = "Bearer Oracle".as_bytes().try_into()?;
        request.headers_mut().insert(header, value);

        let response = self.client.request(request).await?;
        if !response.status().is_success() {
            return Err(response.status().into());
        }

        Ok(to_bytes(response).await?)
    }
}
