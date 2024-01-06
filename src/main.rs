use std::net::SocketAddr;

use http_body_util::{combinators::BoxBody, BodyExt, Full, Empty};
use hyper::body::{Bytes, Frame};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Body, Request, Response, Method, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use std::fs;

// async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
//     println!("test");
//     Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
// }

async fn echo(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => Ok(Response::new(full(
            "Try POSTing data to /echo such as: `curl localhost:3000/echo -XPOST -d \"hello world\"`",
        ))),
        (&Method::GET, "/test") => {
            let html = fs::read_to_string("index/test.html").expect("Unable to read file");
            Ok(Response::new(full(html)))
        },
        (&Method::POST, "/traiter_mot") => {
            Ok(Response::new(req.into_body().boxed()))
        },
        // Simply echo the body back to the client.
        (&Method::POST, "/echo") => Ok(Response::new(req.into_body().boxed())),
        (&Method::POST, "/echo/uppercase") => {
    // Map this body's frame to a different type
    let frame_stream = req.into_body().map_frame(|frame| {
        let frame = if let Ok(data) = frame.into_data() {
            // Convert every byte in every Data frame to uppercase
            data.iter()
                .map(|byte| byte.to_ascii_uppercase())
                .collect::<Bytes>()
        } else {
            Bytes::new()
        };

        Frame::data(frame)
    });

    Ok(Response::new(frame_stream.boxed()))
},
            (&Method::POST, "/echo/reversed") => {
    // Protect our server from massive bodies.
    let upper = req.body().size_hint().upper().unwrap_or(u64::MAX);
    if upper > 1024 * 64 {
        let mut resp = Response::new(full("Body too big"));
        *resp.status_mut() = hyper::StatusCode::PAYLOAD_TOO_LARGE;
        return Ok(resp);
    }

    // Await the whole body to be collected into a single `Bytes`...
    let whole_body = req.collect().await?.to_bytes();

    // Iterate the whole body in reverse order and collect into a new Vec.
    let reversed_body = whole_body.iter()
        .rev()
        .cloned()
        .collect::<Vec<u8>>();

    Ok(Response::new(full(reversed_body)))
},
        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(io, service_fn(echo))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
