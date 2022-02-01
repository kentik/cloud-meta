use std::collections::HashMap;
use std::io::BufRead;
use std::time::Duration;
use http::uri::{Uri, Parts};
use hyper::{Body, Method, Request};
use hyper::body::{to_bytes, Bytes};
use hyper::client::{Client, HttpConnector};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::Error;

pub struct Amazon {
    client:   Client<HttpConnector, Body>,
    endpoint: Parts,
    version:  String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    pub instance_id:   String,
    pub image_id:      String,
    pub architecture:  String,
    pub instance_type: String,
    pub region:        String,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl Amazon {
    pub fn new(client: Client<HttpConnector, Body>) -> Self {
        let endpoint = "http://169.254.169.254";
        let version  = "2021-07-15";
        Self {
            client:   client,
            endpoint: Uri::from_static(endpoint).into_parts(),
            version:  version.to_owned(),
        }
    }

    pub async fn instance(&self, token: Option<&[u8]>) -> Result<Instance, Error> {
        let path  = "dynamic/instance-identity/document";
        let bytes = self.get(path, token).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    #[async_recursion::async_recursion]
    pub async fn scan(
        &self,
        path:  &str,
        token: Option<&'async_recursion [u8]>
    ) -> Result<Value, Error> {
        let bytes = self.get(path, token).await?;

        if !path.ends_with('/') {
            let value = String::from_utf8(bytes.to_vec())?;
            return Ok(Value::String(value));
        }

        let mut map = Map::new();
        for line in bytes.lines() {
            let name  = line?;
            let path  = format!("{}{}", path, name);
            let value = self.scan(&path, token).await;
            map.insert(name, value.unwrap_or(Value::Null));
        }
        Ok(Value::Object(map))
    }

    pub async fn get(&self, path: &str, token: Option<&[u8]>) -> Result<Bytes, Error> {
        let version = &self.version;
        let path    = format!("/{version}/{path}");

        let mut request = self.request(Method::GET, &path)?;

        if let Some(token) = token {
            let header = "X-aws-ec2-metadata-token";
            let value  = token.try_into()?;
            request.headers_mut().insert(header, value);
        }

        let response = self.client.request(request).await?;
        if !response.status().is_success() {
            return Err(response.status().into());
        }

        Ok(to_bytes(response).await?)
    }

    pub async fn token(&self, ttl: Duration) -> Result<Vec<u8>, Error> {
        let mut request = self.request(Method::PUT, "/latest/api/token")?;

        let header = "X-aws-ec2-metadata-token-ttl-seconds";
        let value  = ttl.as_secs().to_string().into_bytes().try_into()?;
        request.headers_mut().insert(header, value);

        let response = self.client.request(request).await?;
        if !response.status().is_success() {
            return Err(response.status().into());
        }

        Ok(to_bytes(response).await?.to_vec())
    }

    fn request(&self, method: Method, path: &str) -> Result<Request<Body>, Error> {
        let mut endpoint = Parts::default();
        endpoint.scheme         = self.endpoint.scheme.clone();
        endpoint.authority      = self.endpoint.authority.clone();
        endpoint.path_and_query = Some(path.try_into()?);

        let mut request = Request::new(Body::empty());
        *request.method_mut() = method;
        *request.uri_mut() = endpoint.try_into()?;

        Ok(request)
    }
}
