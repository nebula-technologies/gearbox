pub mod body;
pub mod client;
pub mod error;
pub mod header;
pub mod request_builder;
pub mod response;
pub mod status_code;
pub mod url;
pub mod utils;

pub use {
    body::Body,
    client::Client,
    error::Error,
    header::Header,
    header::HeaderMap,
    request_builder::{Builder, Method},
    response::Response,
    status_code::StatusCode,
    url::Url,
};

#[cfg(test)]
mod tests {

    use crate::net::http::request::Builder;
    use crate::net::http::test::test_server::start_test_server;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_get_request() {
        let (addr, tx) = start_test_server().await;

        // Give the server a moment to start
        sleep(Duration::from_secs(1)).await;

        let url = format!("http://{}", addr);
        let builder = Builder::GET.url(&url);
        let response_raw = unsafe { builder.send().await.unwrap_unchecked() };
        let response = response_raw.body().into_str().await.unwrap();
        assert_eq!(response, "GET response");

        // Shut down the server
        tx.send(()).unwrap();
    }

    #[tokio::test]
    async fn test_post_request() {
        let (addr, tx) = start_test_server().await;

        // Give the server a moment to start
        sleep(Duration::from_secs(1)).await;

        let url = format!("http://{}", addr);
        let builder = Builder::POST.url(&url);
        let response_raw = unsafe { builder.send().await.unwrap_unchecked() };
        let response = response_raw.body().into_str().await.unwrap();
        assert_eq!(response, "POST response");

        // Shut down the server
        tx.send(()).unwrap();
    }

    #[tokio::test]
    async fn test_patch_request() {
        let (addr, tx) = start_test_server().await;

        // Give the server a moment to start
        sleep(Duration::from_secs(1)).await;

        let url = format!("http://{}", addr);
        let builder = Builder::PATCH.url(&url);
        let response_raw = unsafe { builder.send().await.unwrap_unchecked() };
        let response = response_raw.body().into_str().await.unwrap();
        assert_eq!(response, "PATCH response");

        // Shut down the server
        tx.send(()).unwrap();
    }

    #[tokio::test]
    async fn test_delete_request() {
        let (addr, tx) = start_test_server().await;

        // Give the server a moment to start
        sleep(Duration::from_secs(1)).await;

        let url = format!("http://{}", addr);
        let builder = Builder::DELETE.url(&url);
        let response_raw = unsafe { builder.send().await.unwrap_unchecked() };
        let response = response_raw.body().into_str().await.unwrap();
        assert_eq!(response, "DELETE response");

        // Shut down the server
        tx.send(()).unwrap();
    }
}
