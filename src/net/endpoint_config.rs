extern crate async_trait;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;

use async_trait::async_trait;
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    headers: Vec<(String, String)>,
    method: Method,
    tls_cert_path: Option<String>, // Optional TLS certificate path for HTTPS
    tls_key_path: Option<String>,  // Optional TLS key path for HTTPS
}

impl HttpConfig {
    // Constructor
    pub fn new(
        headers: Vec<(String, String)>,
        method: Method,
        tls_cert_path: Option<String>,
        tls_key_path: Option<String>,
    ) -> Self {
        HttpConfig {
            headers,
            method,
            tls_cert_path,
            tls_key_path,
        }
    }

    // Primary Getters
    pub fn headers(&self) -> &Vec<(String, String)> {
        &self.headers
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn tls_cert_path(&self) -> Option<&String> {
        self.tls_cert_path.as_ref()
    }

    pub fn tls_key_path(&self) -> Option<&String> {
        self.tls_key_path.as_ref()
    }

    // Setters
    pub fn set_headers(&mut self, headers: Vec<(String, String)>) -> &mut Self {
        self.headers = headers;
        self
    }

    pub fn set_method(&mut self, method: Method) -> &mut Self {
        self.method = method;
        self
    }

    pub fn set_tls_cert_path(&mut self, tls_cert_path: Option<String>) -> &mut Self {
        self.tls_cert_path = tls_cert_path;
        self
    }

    pub fn set_tls_key_path(&mut self, tls_key_path: Option<String>) -> &mut Self {
        self.tls_key_path = tls_key_path;
        self
    }

    // `with_` Methods for Construction
    pub fn with_headers(mut self, headers: Vec<(String, String)>) -> Self {
        self.headers = headers;
        self
    }

    pub fn with_method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    pub fn with_tls_cert_path(mut self, tls_cert_path: Option<String>) -> Self {
        self.tls_cert_path = tls_cert_path;
        self
    }

    pub fn with_tls_key_path(mut self, tls_key_path: Option<String>) -> Self {
        self.tls_key_path = tls_key_path;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

impl From<Method> for reqwest::Method {
    fn from(method: Method) -> Self {
        match method {
            Method::GET => reqwest::Method::GET,
            Method::POST => reqwest::Method::POST,
            Method::PUT => reqwest::Method::PUT,
            Method::DELETE => reqwest::Method::DELETE,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Protocol {
    Http,
    Https,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsAddr {
    pub host: String,
    pub port: u16,
}

// Implementation for DnsAddr
impl DnsAddr {
    // Constructor
    pub fn new(host: String, port: u16) -> Self {
        DnsAddr { host, port }
    }

    // Primary Getters
    pub fn host(&self) -> &String {
        &self.host
    }

    pub fn host_owned(&self) -> String {
        self.host.clone()
    }

    pub fn host_mut(&mut self) -> &mut String {
        &mut self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    // Setters
    pub fn set_host(&mut self, host: String) -> &mut Self {
        self.host = host;
        self
    }

    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }

    // `with_` Methods for Construction
    pub fn with_host(mut self, host: String) -> Self {
        self.host = host;
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HostEndpoint {
    Socket(SocketAddr),
    Dns(DnsAddr),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointConfig {
    address: HostEndpoint,
    protocol: Protocol,
    tls_cert_path: Option<String>,
    tls_key_path: Option<String>,
}

// Primary Getters
impl EndpointConfig {
    pub fn address(&self) -> &HostEndpoint {
        &self.address
    }

    pub fn address_owned(&self) -> HostEndpoint {
        self.address.clone()
    }

    pub fn address_mut(&mut self) -> &mut HostEndpoint {
        &mut self.address
    }

    pub fn protocol(&self) -> &Protocol {
        &self.protocol
    }

    pub fn protocol_owned(&self) -> Protocol {
        self.protocol.clone()
    }

    pub fn protocol_mut(&mut self) -> &mut Protocol {
        &mut self.protocol
    }

    pub fn tls_cert_path(&self) -> Option<&String> {
        self.tls_cert_path.as_ref()
    }

    pub fn tls_cert_path_owned(&self) -> Option<String> {
        self.tls_cert_path.clone()
    }

    pub fn tls_key_path(&self) -> Option<&String> {
        self.tls_key_path.as_ref()
    }

    pub fn tls_key_path_owned(&self) -> Option<String> {
        self.tls_key_path.clone()
    }

    // Setters
    pub fn set_address(&mut self, address: HostEndpoint) -> &mut Self {
        self.address = address;
        self
    }

    pub fn set_protocol(&mut self, protocol: Protocol) -> &mut Self {
        self.protocol = protocol;
        self
    }

    pub fn set_tls_cert_path(&mut self, tls_cert_path: String) -> &mut Self {
        self.tls_cert_path = Some(tls_cert_path);
        self
    }

    pub fn set_tls_key_path(&mut self, tls_key_path: String) -> &mut Self {
        self.tls_key_path = Some(tls_key_path);
        self
    }

    // `with_` Methods for Construction
    pub fn with_address(mut self, address: HostEndpoint) -> Self {
        self.address = address;
        self
    }

    pub fn with_protocol(mut self, protocol: Protocol) -> Self {
        self.protocol = protocol;
        self
    }

    pub fn with_tls_cert_path(mut self, tls_cert_path: Option<String>) -> Self {
        self.tls_cert_path = tls_cert_path;
        self
    }

    pub fn with_tls_key_path(mut self, tls_key_path: Option<String>) -> Self {
        self.tls_key_path = tls_key_path;
        self
    }
}
pub trait Emitter {
    fn send<'a, 'b, 'async_trait>(
        &'a self,
        data: &'b str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'async_trait>>
    where
        'a: 'async_trait,
        'b: 'async_trait,
        Self: 'async_trait;
}
impl Emitter for EndpointConfig {
    fn send<'a, 'b, 'async_trait>(
        &'a self,
        data: &'b str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'async_trait>>
    where
        'a: 'async_trait,
        'b: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(send_request(self.clone(), data))
    }
}

async fn send_request(endpoint: EndpointConfig, data: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = match &endpoint.address {
        HostEndpoint::Socket(socket_addr) => {
            format!(
                "{0}://{1}",
                match endpoint.protocol {
                    Protocol::Http => "http",
                    Protocol::Https => "https",
                },
                socket_addr,
            )
        }
        HostEndpoint::Dns(dns_addr) => {
            format!(
                "{0}://{1}:{2}",
                match endpoint.protocol {
                    Protocol::Http => "http",
                    Protocol::Https => "https",
                },
                dns_addr.host,
                dns_addr.port,
            )
        }
    };
    let response = client
        .request(Method::POST.into(), &url)
        .body(data.to_string())
        .send()
        .await?;
    println!("HTTP/HTTPS response: {0:?}\n", response.status());
    Ok(())
}
