use futures::{StreamExt, TryStreamExt};
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Body, header::HeaderName, header::HeaderValue, Request, Response};
use hyper_util::rt::TokioIo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::{TcpListener as TokioTcpListener, TcpListener};
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;

#[derive(Debug, Deserialize, Serialize)]
struct ReturnToMe {
    status: u16,
    payload: String,
    headers: HashMap<String, String>,
}

async fn handle_request(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let method = req.method().clone();
    let whole_body = req.into_body().collect().await?.to_bytes().to_vec();

    if !whole_body.is_empty() {
        if let Ok(return_to_me) = serde_json::from_slice::<ReturnToMe>(&whole_body)
            .map_err(|e| panic!("Error deserializing JSON: {:?}", e))
        {
            let mut response = Response::builder()
                .status(return_to_me.status)
                .body(Full::new(Bytes::from(return_to_me.payload)))
                .unwrap();

            for (key, value) in return_to_me.headers {
                response.headers_mut().insert(
                    HeaderName::from_bytes(key.as_bytes()).unwrap(),
                    HeaderValue::from_str(&value).unwrap(),
                );
            }

            return Ok(response);
        }
    } else {
        println!("We did not find any body in the request.");
    }

    Ok(match method {
        hyper::Method::GET => Response::new(Full::new(Bytes::from("GET response"))),
        hyper::Method::POST => Response::new(Full::new(Bytes::from("POST response"))),
        hyper::Method::PATCH => Response::new(Full::new(Bytes::from("PATCH response"))),
        hyper::Method::DELETE => Response::new(Full::new(Bytes::from("DELETE response"))),
        _ => Response::new(Full::new(Bytes::from("Unsupported method"))),
    })
}

pub async fn test_server(listener: TcpListener, mut rx: Receiver<()>) {
    loop {
        tokio::select! {
            _ = &mut rx => {
                println!("Shutdown signal received.");
                break;
            }
            Ok((stream, _)) = listener.accept() => {
                let io = TokioIo::new(stream);

                tokio::task::spawn(async move {
                    if let Err(err) = http1::Builder::new()
                        .serve_connection(io, service_fn(handle_request))
                        .await
                    {
                        eprintln!("Error serving connection: {:?}", err);
                    }
                });
            }
        }
    }
}

pub async fn start_test_server() -> (SocketAddr, oneshot::Sender<()>) {
    let (tx, rx) = oneshot::channel();

    for port in 3000..=3100 {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        if let Ok(t) = TokioTcpListener::bind(addr).await {
            tokio::spawn(test_server(t, rx));
            return (addr, tx);
        }
    }
    panic!("No available ports in the range 3000-3100");
}
