#![deny(warnings)]
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::time::Duration;
use http::HeaderValue;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server};
use std::collections::HashMap;

type HttpClient = Client<HttpConnector>;
type HttpsClient = Client<HttpsConnector<HttpConnector>>;

lazy_static::lazy_static! {
    static ref FORWARDING_MAP: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("/v1/action/test", "http://127.0.0.1:8000");
        // map.insert("/app", "http://app.example.com");
        map
    };
}


fn determine_base_url(req: &Request<Body>) -> Option<String> {
    // Look up the base URL in the map based on the request path
    println!("{}",req.uri().path());
    FORWARDING_MAP.get(req.uri().path()).map(|&url| url.to_string())
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8100));
    let https_connector = HttpsConnector::new();
   let https_client = Client::builder()
       .pool_idle_timeout(Duration::from_secs(15))
       .build::<_, hyper::Body>(https_connector);

   let http_client = Client::builder()
       .pool_idle_timeout(Duration::from_secs(15))
       .http2_only(false)
       .http1_title_case_headers(true)
       .http1_preserve_header_case(true)
       .build_http();

    let make_service = make_service_fn(move |_| {
        let http_client = http_client.clone();
        let https_client = https_client.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| proxy(http_client.clone(), https_client.clone(), req)))
        }
    });

    let server = Server::bind(&addr)
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(make_service);

    println!("Listening on http://{}", addr);
    
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn proxy(http_client: HttpClient, https_client: HttpsClient, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let headers = req.headers().clone();
    println!("headers: {:?} \n", headers);
    let target_url = req.uri().to_string();
    println!("target_url:{:?} \n", target_url);
    let base_url = determine_base_url(&req).unwrap_or_else(|| "https://xxxx.com".to_string());
    // let base_url = determine_base_url(&req).unwrap();

    return Ok(match req.uri().scheme_str() {
        Some("http") => get_response(http_client, req, &base_url, &target_url).await?,
        Some("https") => get_response(https_client, req, &base_url, &target_url).await?,
        _ => get_response(https_client, req, &base_url, &target_url).await?,
    });
}

async fn get_response<C: hyper::client::connect::Connect + Clone + Send + Sync + 'static>(
    client: Client<C>,
    req: Request<Body>,
    upstream_url: &str,
    target_url: &str,
) -> Result<Response<Body>, hyper::Error> {
    // ... rest of your code ...
    let target_url = format!("{}{}",upstream_url,target_url);
    println!("{}",target_url);
    
    let mut headers = req.headers().clone();
    let mut request_builder = Request::builder()
        .method(req.method())
        .uri(target_url)
        .body(req.into_body())
        .unwrap();
    headers.insert("BFF_WORLFLOW_TRACE", HeaderValue::from_static("liber_test"));
    *request_builder.headers_mut() = headers;
    
    let response = client.request(request_builder).await?;
    let body = hyper::body::to_bytes(response.into_body()).await?;
    let body = String::from_utf8(body.to_vec()).unwrap();
    println!("{:?}",body);
    let mut resp: Response<Body> = Response::new(Body::from(body));
    *resp.status_mut() = http::StatusCode::OK;
    Ok(resp)
}
