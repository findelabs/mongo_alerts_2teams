use hyper::{Body, Method, Request, Response, StatusCode};
use std::str::from_utf8;

use crate::config;
use crate::post;
use crate::transform;

// This is our service handler. It receives a Request, routes on its
// path, and returns a Future of a Response.
pub async fn echo(
    req: Request<Body>,
    config: config::ConfigHash,
) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Paths:\n\t/echo: Returns json back\n\t/stdout: Write posted json to stdout\n\t/alert: Send alert to teams\n\t/testalert: Returns body of post to teams",
        ))),

        // Return posted body
        (&Method::POST, "/echo") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            let whole_body_vec = whole_body.iter().cloned().collect::<Vec<u8>>();
            let value = from_utf8(&whole_body_vec).to_owned()?;
            let value: serde_json::Value = serde_json::from_str(value)?;
            Ok(Response::new(Body::from(value.to_string())))
        }

        // Log to stdout posted body
        (&Method::POST, "/stdout") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            let whole_body_vec = whole_body.iter().cloned().collect::<Vec<u8>>();
            let value = from_utf8(&whole_body_vec).to_owned()?;
            let value: serde_json::Value = serde_json::from_str(value)?;
            log::info!("printing json posted to /stdout: {}",&value.to_string());
            Ok(Response::new(Body::from("json accepted and written to stdout")))
        }

        // echo transformed card with received variables
        (&Method::POST, "/testalert") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            let whole_body_vec = whole_body.iter().cloned().collect::<Vec<u8>>();
            let value = from_utf8(&whole_body_vec).to_owned()?;
            let value_json: serde_json::Value = serde_json::from_str(value)?;
            let card_body = transform::create_card(value_json)?;
            Ok(Response::new(Body::from(card_body.to_string())))
        }

        // Alert transformed card with received variables
        (&Method::POST, "/alert") => {
            let (parts,body) = req.into_parts();
            let whole_body = hyper::body::to_bytes(body).await?;
            let whole_body_vec = whole_body.iter().cloned().collect::<Vec<u8>>();
            let value = from_utf8(&whole_body_vec).to_owned()?;
            let value_json: serde_json::Value = serde_json::from_str(value)?;
            let card_body = transform::create_card(value_json.clone())?;

            log::info!("Received message: {}", value_json);

            match config::match_channel(&parts, config) {
                Some(url) => {
                    match post::post_retry(&card_body, url).await {
                        Some(true) => {
                            let mut response = Response::default();
                            *response.status_mut() = StatusCode::OK;
                            log::info!("Successfully posted: \"{}\"", card_body);
                            Ok(response)
                        },
                        Some(false) => {
                            let mut response = Response::default();
                            *response.status_mut() = StatusCode::TOO_MANY_REQUESTS;
                            log::error!("Posting to teams failed for id: {}, bulk post failure", value_json["id"]);
                            Ok(response)
                        },
                        None => {
                            let mut response = Response::default();
                            *response.status_mut() = StatusCode::BAD_REQUEST;
                            log::error!("Post failed for id: {}", value_json["id"]);
                            Ok(response)
                        }
                    }
                },
                None => {
                    let mut response = Response::default();
                    *response.status_mut() = StatusCode::BAD_REQUEST;
                    log::error!("Bad channel specified for id: {}", value_json["id"]);
                    Ok(response)
                }
            }
        }

        // echo transformed card with received variables
        (&Method::GET, "/health") => {
            Ok(Response::new(Body::from("ok".to_string())))
        }

        // echo transformed card with received variables
        (&Method::GET, "/config") => {
            let config = config.lock().expect("Unable to lock config");
            Ok(Response::new(Body::from(format!("{:#?}",config))))
        }

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
