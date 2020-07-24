use hyper::service::{make_service_fn, service_fn};
use hyper::{Client, Body, Method, Request, Response, Server, StatusCode};
use serde_json::json;
use std::str::from_utf8;
use hyper_tls::HttpsConnector;

const URL: &str = "";

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
async fn echo(req: Request<Body>) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
    let mut card_body = json!({
        "@type": "MessageCard",
        "@context": "https://schema.org/extensions",
        "summary": "Test card",
        "themeColor": "0078D7",
        "title": "",
        "sections": [
            {
                "activityTitle": "",
                "activitySubtitle": "",
                "activityImage": "",
                "facts": [
                    {
                        "name": "Replica set:",
                        "value": ""
                    },
                    {
                        "name": "Server:",
                        "value": ""
                    },
                    {
                        "name": "Type:",
                        "value": ""
                    },
        
                ]
            }
        ]
    });

    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Paths:\n\t/echo: Returns json back\n\t/card: Displays teams card\n\t/stdout: Write posted json to stdout",
        ))),

        // Return default card
        (&Method::GET, "/card") => Ok(Response::new(Body::from(
            card_body.to_string()
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
            println!("Received json: {}",&value.to_string());
            Ok(Response::new(Body::from("json accepted and written to stdout")))
        }

        // echo transformed card with received variables
        (&Method::POST, "/echo_alert") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            let whole_body_vec = whole_body.iter().cloned().collect::<Vec<u8>>();
            let value = from_utf8(&whole_body_vec).to_owned()?;
            let value: serde_json::Value = serde_json::from_str(value)?;
            println!("Received json: {}",&value.to_string());
            
            // Transform card
            card_body["title"] = value["eventTypeName"].clone();
            card_body["sections"][0]["activitySubtitle"] = value["created"].clone();
            card_body["sections"][0]["activityTitle"] = value["eventTypeName"].clone();
            card_body["sections"][0]["facts"][0]["value"] = value["replicaSetName"].clone();
            card_body["sections"][0]["facts"][1]["value"] = value["hostnameAndPort"].clone();
            card_body["sections"][0]["facts"][2]["value"] = value["status"].clone();

            Ok(Response::new(Body::from(card_body.to_string())))
        }

        // Alert transformed card with received variables
        (&Method::POST, "/alert") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            let whole_body_vec = whole_body.iter().cloned().collect::<Vec<u8>>();
            let value = from_utf8(&whole_body_vec).to_owned()?;
            let value: serde_json::Value = serde_json::from_str(value)?;

            println!("Received json post to /alert: {}", value.to_string());
            
            // Set status
            if value["status"].is_string() {
                if value["status"] == "OPEN" {
                    card_body["title"] = serde_json::to_value("New Alert Triggered")?;
                    card_body["sections"][0]["activityImage"] = serde_json::to_value("https://upload.wikimedia.org/wikipedia/commons/9/92/Error_%2889607%29_-_The_Noun_Project.svg")?;
                } else if value["status"] == "CLOSED" {
                    card_body["title"] = serde_json::to_value("Alert Closed")?;
                    card_body["sections"][0]["activityImage"] = serde_json::to_value("https://upload.wikimedia.org/wikipedia/commons/1/15/Mood-very-good_%28CoreUI_Icons_v1.0.0%29.svg")?;
                } else if value["status"] == "INFORMATIONAL" {
                    card_body["title"] = serde_json::to_value("Informational Alert")?;
                    card_body["sections"][0]["activityImage"] = serde_json::to_value("https://upload.wikimedia.org/wikipedia/commons/9/9b/Good_Article_%28Black%29.svg")?;
                }
            };

            // Set title based on eventTypeName
            if value["eventTypeName"].is_string() { 
                if value["eventTypeName"] == "PRIMARY_ELECTED" {
                    card_body["sections"][0]["activityTitle"] = serde_json::to_value("Replica set elected a new primary")?;
                } else {
                    card_body["sections"][0]["activityTitle"] = value["eventTypeName"].clone();
                }
            };

            // Transform card facts
            if value["created"].is_string() { card_body["sections"][0]["activitySubtitle"] = value["created"].clone() };
            if value["replicaSetName"].is_string() { card_body["sections"][0]["facts"][0]["value"] = value["replicaSetName"].clone() };
            if value["hostnameAndPort"].is_string() { card_body["sections"][0]["facts"][1]["value"] = value["hostnameAndPort"].clone() };
            if value["typeName"].is_string() { card_body["sections"][0]["facts"][2]["value"] = value["typeName"].clone() };

            let https = HttpsConnector::new();
            let client = Client::builder().build::<_, hyper::Body>(https);
            let req = Request::builder()
                .method("POST")
                .uri(URL)
                .header("Content-Type","application/json")
                .body(Body::from(card_body.to_string()))
                .expect("request builder");

            let future = match client.request(req).await {
                Ok(_) => {
                    println!("Successful post to {}", URL);
                    "success"
                },
                Err(e) => {
                    println!("Caught error posting: {}", e);
                    "error"
                }
            };
            Ok(Response::new(Body::from(future)))
        }

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([0, 0, 0, 0], 8000).into();

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(echo)) });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
