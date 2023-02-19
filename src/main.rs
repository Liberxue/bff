extern crate serde_json;
extern crate hyper;
pub mod cars;
pub mod handler;
use std::net::SocketAddr;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use log::{info, warn,error, debug,trace};
use tokio::net::TcpListener;
extern crate pretty_env_logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let json_str = r#" { "name": "Test BFF", "age": 30, "liber": "000", "sex":18, "address": { "street": "123 Main St", "city": "Shanghai", "data1":{ "Liber":"1234", "Liber123":"1234" } } } "#;
    let config_str = r#" { "update": { "age": 80, "liber": "111", "data3":{ "key1":"value1" } }, "add": { "address": { "zipcode": "94107", "liber": "111", "liber222222": "111", "data":[ { "math_1":"1111", "englist_1":"5555" }, { "math_2":"123", "englist_2":"456" } ] } }, "delete":[ "address.data1.Liber123", "address.data1.Liber", "address.data1" ] } "#;

    pretty_env_logger::init();

    let result = handler::action(json_str,config_str);
    println!("result {}",result);
    eprintln!("error: test");
    info!("this's info log");
    warn!("this's warn log");
    error!("this's error log");
    debug!("this's debug log");
    trace!("this's trace log");
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::task::spawn(async move {
            if let Err(err) = Http::new().serve_connection(stream, service_fn(cars::cars_handler)).await {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}