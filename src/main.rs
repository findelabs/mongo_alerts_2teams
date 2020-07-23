use futures_util::TryStreamExt;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use serde_json::json;


/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
async fn echo(req: Request<Body>) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
let card_body = json!({
    "@type": "MessageCard",
    "@context": "https://schema.org/extensions",
    "summary": "Test card",
    "themeColor": "0078D7",
    "title": "--title--",
    "sections": [
        {
            "activityTitle": "--stitle--",
            "activitySubtitle": "--date--",
            "activityImage": "--icon--",
            "facts": [
                {
                    "name": "Custer:",
                    "value": "--clustername--"
                },
                {
                    "name": "Replica set:",
                    "value": "--repset--"
                },
                {
                    "name": "Server:",
                    "value": "--handp--"
                },
                {
                    "name": "Metric:",
                    "value": "--stitle--"
                },
                {
                    "name": "Metric value:",
                    "value": "--value--"
                },
                {
                    "name": "Type:",
                    "value": "--type--"
                },
                {
                    "name": "Event:",
                    "value": "--event--"
                }
    
            ],
            "text": "--MESSAGE--"
        }
    ],
    "potentialAction": [      
        {
            "@type": "OpenUri",
            "name": "Check the stats",
            "targets": [
                {
                    "os": "default",
                    "uri": "--url--",
                    "url": "--url--"
                }
            ]
        }
    ]
});

    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Try POSTing data to /echo such as: `curl localhost:8000/echo -XPOST -d 'hello world'`",
        ))),

        // Return default card
        (&Method::GET, "/default") => Ok(Response::new(Body::from(
            card_body.to_string()
        ))),

        // Simply echo the body back to the client.
        (&Method::POST, "/echo") => Ok(Response::new(req.into_body())),

        // Log to stdout posted body
        (&Method::POST, "/stdout") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            println!("{:?}",whole_body);
            Ok(Response::new(Body::from("Written to stdout")))
        }

        (&Method::POST, "/echo/uppercase") => {
            let chunk_stream = req.into_body().map_ok(|chunk| {
                chunk
                    .iter()
                    .map(|byte| byte.to_ascii_uppercase())
                    .collect::<Vec<u8>>()
            });
            Ok(Response::new(Body::wrap_stream(chunk_stream)))
        }

        // Reverse the entire body before sending back to the client.
        //
        // Since we don't know the end yet, we can't simply stream
        // the chunks as they arrive as we did with the above uppercase endpoint.
        // So here we do `.await` on the future, waiting on concatenating the full body,
        // then afterwards the content can be reversed. Only then can we return a `Response`.
        (&Method::POST, "/echo/reversed") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;

            let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
            Ok(Response::new(Body::from(reversed_body)))
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
    let addr = ([127, 0, 0, 1], 8000).into();

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(echo)) });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
