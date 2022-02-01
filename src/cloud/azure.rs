use std::collections::HashMap;
use http::uri::{Uri, Parts};
use hyper::{Body, Method, Request};
use hyper::body::{to_bytes, Bytes};
use hyper::client::{Client, HttpConnector};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::Error;

pub struct Azure {
    client:   Client<HttpConnector, Body>,
    endpoint: Parts,
    version:  String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    pub compute: Compute,
    pub network: Value,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Compute {
    pub vm_id:   String,
    pub name:    String,
    pub os_type: String,
    pub vm_size: String,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl Azure {
    pub fn new(client: Client<HttpConnector, Body>) -> Self {
        let endpoint = "http://169.254.169.254";
        let version  = "2021-05-01";
        Self {
            client:   client,
            endpoint: Uri::from_static(endpoint).into_parts(),
            version:  version.to_owned(),
        }
    }

    pub async fn instance(&self) -> Result<Instance, Error> {
        let bytes = self.get("instance").await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn get(&self, path: &str) -> Result<Bytes, Error> {
        let version = &self.version;
        let path    = format!("/metadata/{path}?api-version={version}");

        let mut endpoint = Parts::default();
        endpoint.scheme         = self.endpoint.scheme.clone();
        endpoint.authority      = self.endpoint.authority.clone();
        endpoint.path_and_query = Some(path.try_into()?);

        let mut request = Request::new(Body::empty());
        *request.method_mut() = Method::GET;
        *request.uri_mut() = endpoint.try_into()?;

        let header = "Metadata";
        let value  = "true".as_bytes().try_into()?;
        request.headers_mut().insert(header, value);

        let response = self.client.request(request).await?;
        if !response.status().is_success() {
            return Err(response.status().into());
        }

        Ok(to_bytes(response).await?)
    }
}
