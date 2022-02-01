use std::time::Duration;
use anyhow::Result;
use hyper::Client;
use tokio::time::sleep;
use cloud_meta::{*, cloud::*};

#[derive(Debug)]
pub enum Cloud {
    Amazon(amazon::Instance),
    Azure(azure::Instance),
    Google(google::Instance),
    Oracle(oracle::Instance),
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();

    let amazon = Amazon::new(client.clone());
    let azure  = Azure::new(client.clone());
    let google = Google::new(client.clone());
    let oracle = Oracle::new(client.clone());

    let amazon = async {
        let ttl   = Duration::from_secs(60);
        let token = amazon.token(ttl).await.ok();
        amazon.instance(token.as_deref()).await
    };

    let azure   = azure.instance();
    let google  = google.instance();
    let oracle  = oracle.instance();
    let timeout = sleep(Duration::from_secs(2));

    let cloud = tokio::select! {
        Ok(instance) = amazon  => Some(Cloud::Amazon(instance)),
        Ok(instance) = azure   => Some(Cloud::Azure(instance)),
        Ok(instance) = google  => Some(Cloud::Google(instance)),
        Ok(instance) = oracle  => Some(Cloud::Oracle(instance)),
        _            = timeout => None,
    };

    println!("cloud: {:?}", cloud);

    Ok(())
}
