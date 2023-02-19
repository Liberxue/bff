use hyper::{header, Body, Method, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::error::Error;
use hyper::body::Buf;
use rand::Rng;
const INTERNAL_SERVER_ERROR: &str = "Internal Server Error";


#[derive(Serialize, Deserialize)]
pub struct Car {
    id: String,
    brand: String,
    model: String,
    year: u16,
}

// #[macro_use] 
pub async fn cars_handler(req: Request<Body>) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
    // match (req.method(), req.uri().path()) {
    //     (&Method::GET, "/cars") => Ok(Response::new(Body::from("GET cars"))),
    //     (&Method::GET, "/cars/:id") => Ok(Response::new(Body::from("GET cars"))),
    //     (&Method::POST, "/cars") => Ok(Response::new(Body::from("POST cars"))),
    //     // Return the 404 Not Found for other routes.
    //     _ => {
    //         let mut not_found = Response::default();
    //         *not_found.status_mut() = StatusCode::NOT_FOUND;
    //         Ok(not_found)
    //     }
    // }
    let path = req.uri().path().to_owned();
    let path_segments = path.split("/").collect::<Vec<&str>>();
    let base_path = path_segments[1];

    match (req.method(), base_path) {
        (&Method::GET, "cars") => {
            if path_segments.len() <= 2 {
                let res = get_car_list();
                return Ok(res);
            }

            let car_id = path_segments[2];

            if car_id.trim().is_empty() {
                let res = get_car_list();
                return Ok(res);
            } else {
                let res = get_car_by_id(&car_id.to_string());
                Ok(res)
            }
        }

        (&Method::POST, "cars") => create_car(req).await,

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

fn get_car_list() -> Response<Body> {
    let cars: [Car; 3] = [
        Car {
            id: "1".to_owned(),
            brand: "Ford".to_owned(),
            model: "Bronco".to_owned(),
            year: 2022,
        },
        Car {
            id: "2".to_owned(),
            brand: "Hyundai".to_owned(),
            model: "Santa Fe".to_owned(),
            year: 2010,
        },
        Car {
            id: "3".to_owned(),
            brand: "Dodge".to_owned(),
            model: "Challenger".to_owned(),
            year: 2015,
        },
    ];

    match serde_json::to_string(&cars) {
        Ok(json) => Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(INTERNAL_SERVER_ERROR.into())
            .unwrap(),
    }
}

 fn get_car_by_id(car_id: &String) -> Response<Body> {
    let cars: [Car; 3] = [
        Car {
            id: "1".to_owned(),
            brand: "Ford".to_owned(),
            model: "Bronco".to_owned(),
            year: 2022,
        },
        Car {
            id: "2".to_owned(),
            brand: "Hyundai".to_owned(),
            model: "Santa Fe".to_owned(),
            year: 2010,
        },
        Car {
            id: "3".to_owned(),
            brand: "Dodge".to_owned(),
            model: "Challenger".to_owned(),
            year: 2015,
        },
    ];

    let car_index_option = cars.iter().position(|x| &x.id == car_id);

    if car_index_option.is_none() {
        return Response::new(Body::from("Car not found"));
    }

    let car = &cars[car_index_option.unwrap()];

    match serde_json::to_string(car) {
        Ok(json) => Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(INTERNAL_SERVER_ERROR.into())
            .unwrap(),
    }
}

async fn create_car(req: Request<Body>) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
    // get the buffer from the request body
    let buffer = hyper::body::aggregate(req).await?;

    // add an id to the new_car
    let mut new_car: serde_json::Value = serde_json::from_reader(buffer.reader())?;

    let mut random = rand::thread_rng();

    let car_id: u8 = random.gen();
    new_car["id"] = serde_json::Value::from(car_id.to_string());
    
    let res = match serde_json::to_string(&new_car) {
        Ok(json) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(INTERNAL_SERVER_ERROR.into())
            .unwrap(),
    };

    Ok(res)
}