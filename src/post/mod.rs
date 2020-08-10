use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;
use std::{thread, time};

pub async fn post_retry(card_body: serde_json::Value, url: String) -> Option<bool> {
    for i in 1..4u16 {
        if i == 4 {
            return Some(false);
        } else {
            let https = HttpsConnector::new();
            let client = Client::builder().build::<_, hyper::Body>(https);
            let req = Request::builder()
                .method("POST")
                .uri(url.clone())
                .header("Content-Type", "application/json")
                .body(Body::from(card_body.to_string()))
                .expect("request builder");

            match client.request(req).await {
                Ok(m) => {
                    if m.status().as_u16() == 429u16 {
                        if i == 1 {
                            log::info!("Detected rate limiting, retrying in 10 seconds");
                            thread::sleep(time::Duration::from_millis(10000));
                            continue;
                        } else if i > 1 && i <= 3 {
                            log::info!("Detected rate limiting, retrying in 30 seconds");
                            thread::sleep(time::Duration::from_millis(30000));
                            continue;
                        }
                    } else if m.status().as_u16() == 200u16 {
                        return Some(true);
                    } else {
                        log::info!(
                            "Failed to post to teams, got status code {}",
                            m.status().as_u16()
                        );
                        log::info!(
                            "Message from Teams endpoint: {:#?}",
                            m
                        );
                        return None;
                    }
                }
                Err(e) => {
                    log::error!("Caught error posting: {}", e);
                    return None;
                }
            };
        }
    }
    None
}
