
use hyper::{Client, Request, Uri, body::HttpBody};
use hyper::client::HttpConnector;
use hyper_proxy::{Proxy, ProxyConnector, Intercept};
use headers::Authorization;
use std::error::Error;
use tokio::io::{stdout, AsyncWriteExt as _};

pub async fn source(source:&str,target:&str) -> Result<(), Box<dyn Error>> {
    let proxy = {
        let proxy_uri = source.parse().unwrap();
        println!("source {}",source);
        let mut proxy = Proxy::new(Intercept::All, proxy_uri);
        proxy.set_authorization(Authorization::basic("John Doe", "Agent1234"));
        let connector = HttpConnector::new();
        let proxy_connector = ProxyConnector::from_proxy(connector, proxy).unwrap();
        proxy_connector
    };

    let uri: Uri = target.parse().unwrap();
    println!("target {}",target);

    let mut req = Request::get(uri.clone()).body(hyper::Body::empty()).unwrap();

    if let Some(headers) = proxy.http_headers(&uri) {
        req.headers_mut().extend(headers.clone().into_iter());
    }

    let client = Client::builder().build(proxy);
    let mut resp = client.request(req).await?;
    println!("Response: {}", resp.status());
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    // // Connecting to an https uri is straightforward (uses 'CONNECT' method underneath)
    // let uri = target.parse().unwrap();
    // let mut resp = client.get(uri).await?;
    // println!("Response: {}", resp.status());
    // while let Some(chunk) = resp.body_mut().data().await {
    //     stdout().write_all(&chunk?).await?;
    // }
    Ok(())
}

