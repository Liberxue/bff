
use hyper::{Client, Request, Uri};
use hyper::client::HttpConnector;
use futures::{TryFutureExt, TryStreamExt};
use hyper_proxy::{Proxy, ProxyConnector, Intercept};
use headers::Authorization;
use std::error::Error;

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
    let fut_http = client.request(req)
        .and_then(|res| res.into_body().map_ok(|x|x.to_vec()).try_concat())
        .map_ok(move |body| ::std::str::from_utf8(&body).unwrap().to_string());

    // Connecting to an https uri is straightforward (uses 'CONNECT' method underneath)
    let uri = target.parse().unwrap();
    let fut_https = client.get(uri)
        .and_then(|res| res.into_body().map_ok(|x|x.to_vec()).try_concat())
        .map_ok(move |body| ::std::str::from_utf8(&body).unwrap().to_string());

    let (http_res, https_res) = futures::future::join(fut_http, fut_https).await;
    let (_, _) = (http_res?, https_res?);
    Ok(())
}

