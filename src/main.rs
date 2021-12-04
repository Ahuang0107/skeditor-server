#![deny(warnings)]

use tokio::fs::File;

use tokio_util::codec::{BytesCodec, FramedRead};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Result, Server, StatusCode};

static TEST1: &str = "files/test1.sketch";
static TEST2: &str = "files/test2.sketch";
static TEST3: &str = "files/test3.sketch";
static NOTFOUND: &[u8] = b"Not Found";

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let address = "127.0.0.1:30000".parse().unwrap();

    let make_service =
        make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(response_examples)) });

    let server = Server::bind(&address).serve(make_service);

    println!("Listening on http://{}", address);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn response_examples(req: Request<Body>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/docs/test1") => simple_file_send(TEST1).await,
        (&Method::GET, "/docs/test2") => simple_file_send(TEST2).await,
        (&Method::GET, "/docs/test3") => simple_file_send(TEST3).await,
        (&Method::GET, "/docs") => send_json().await,
        _ => Ok(not_found()),
    }
}

/// HTTP status code 404
fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(NOTFOUND.into())
        .unwrap()
}

async fn send_json() -> Result<Response<Body>> {
    let body = Body::from("['test1','test2','test3']");
    Ok(Response::new(body))
}

async fn simple_file_send(filename: &str) -> Result<Response<Body>> {
    if let Ok(file) = File::open(filename).await {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        return Ok(Response::new(body));
    }

    Ok(not_found())
}