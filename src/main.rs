#![deny(warnings)]

extern crate mime_guess;
use std::path::Path;

// use futures::{Future, Stream};
// use futures_fs::{FsPool, FsReadStream};
// use futures_util::TryStreamExt;
// use futures_util::TryFutureExt;

// use bytes::BytesMut;
// use tokio::fs::File;
// use tokio_util::codec::{BytesCodec, FramedRead};

use tokio::fs::File;
use tokio::io::AsyncReadExt;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, 
  //Method, 
  Request, Response, Result, Server, StatusCode, 
  header
};

static INDEX: &str = "static/index.html";
static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";
static NOTFOUND: &[u8] = b"Not Found";

#[tokio::main]
async fn main() {
  pretty_env_logger::init();

  let addr = "0.0.0.0:8080".parse().unwrap();

  let make_service =
    make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(handle_request)) });

  let server = Server::bind(&addr).serve(make_service);

  println!("Listening on http://127.0.0.1:8080/");

  if let Err(e) = server.await {
    eprintln!("server error: {}", e);
  }
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>> {
  let mut file_name: String = "./static".to_owned();
  file_name.push_str(req.uri().path());

  if Path::new(&file_name).exists() {
    simple_file_send(&file_name).await
  } else {
    simple_file_send(INDEX).await
  }
}

/// HTTP status code 404
fn not_found() -> Response<Body> {
  Response::builder()
    .status(StatusCode::NOT_FOUND)
    .body(NOTFOUND.into())
    .unwrap()
}

/// HTTP status code 500
fn internal_server_error() -> Response<Body> {
  Response::builder()
    .status(StatusCode::INTERNAL_SERVER_ERROR)
    .body(INTERNAL_SERVER_ERROR.into())
    .unwrap()
}

// async fn simple_file_send_async(filename: &str) -> Result<Response<Body>> {
//   // get mime-type for the requested content
//   let mime_type = mime_guess::from_path(Path::new(filename)).first_or_octet_stream();

//   // let stream: dyn futures::Stream = File::open("C:\\Source\\Backup_Ignore.txt")
//   //   .map_ok(|file| FramedRead::new(file, BytesCodec::new()).map_ok(BytesMut::freeze));
//   let fs = FsPool::default();
//   let stream = fs.read(filename, Default::default()).await;

//   Ok(Response::builder()
//     .header(header::CONTENT_TYPE, mime_type.to_string())
//     .body(stream)
//     .unwrap())
// }

async fn simple_file_send(filename: &str) -> Result<Response<Body>> {
  // Serve a file by asynchronously reading it entirely into memory.
  // Uses tokio_fs to open file asynchronously, then tokio::io::AsyncReadExt
  // to read into memory asynchronously.

  let mime_type = mime_guess::from_path(Path::new(filename)).first_or_octet_stream();

  if let Ok(mut file) = File::open(filename).await {
      let mut buf = Vec::new();
      if let Ok(_) = file.read_to_end(&mut buf).await {
          return Ok(Response::builder()
            .header(header::CONTENT_TYPE, mime_type.to_string())
            .body(buf.into())
            .unwrap());
      }

      return Ok(internal_server_error());
  }

  Ok(not_found())
}
