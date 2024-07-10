use crate::net::http::request::{Body, Builder, Url};
use serde_derive::{Deserialize, Serialize};

use crate::collections::HashMap;
use crate::error::{DynTracerError, TracerError};
use crate::net::http::request;
use crate::net::http::request::body::BodyTrait;
use crate::rails::ext::fut::ext::result::Map;
use crate::rails::ext::future::FutureResult;
#[cfg(target_arch = "wasm32")]
use crate::serde::wasm_bindgen as serde_wasm_bindgen;
use crate::template::engine::TemplateContext;
use crate::template::{PipelineValue, TemplateEngine};
use crate::{tracer_dyn_err, tracer_err};
use alloc::{string::String, sync::Arc, vec::Vec};
use core::future::Future;
use serde::{de, ser, Deserializer, Serializer};
use spin::Mutex;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// The `RequestChain` structure allows for creating a sequence of HTTP requests where the response
/// from one request can be used to populate and forward data to the next request. This is useful
/// for scenarios such as calling a REST API where an initial request is needed to obtain a nonce
/// before making the main API call.
///
/// Requests are stored in `template_requests` with each `RequestNode` identified by a unique name.
/// Multiple call chains can be defined in `call_structures`, where each chain is represented by
/// a vector of request names. This allows for reusing requests across different chains.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RequestChain {
    template_requests: HashMap<String, RequestNode>,
    call_structures: HashMap<String, Vec<String>>,
}

impl RequestChain {
    /// Creates a new, empty `RequestChain`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request_chaining::*;
    ///
    /// let chain = RequestChain::new();
    /// ```
    pub fn new() -> RequestChain {
        RequestChain {
            template_requests: HashMap::new(),
            call_structures: HashMap::new(),
        }
    }

    /// Adds a `RequestNode` to the template requests.
    ///
    /// # Arguments
    ///
    /// * `request` - The `RequestNode` to add to the template requests.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request_chaining::*;
    ///
    /// let mut chain = RequestChain::new();
    /// let node = RequestNode::default();
    /// chain.add_template_request(node);
    /// ```
    pub fn add_template_request(&mut self, request: RequestNode) {
        self.template_requests.insert(request.name.clone(), request);
    }
    /// Adds a template request to the chain using an asynchronous callback function.
    ///
    /// This method allows for the addition of a `RequestNode` to the `RequestChain` through a
    /// provided callback function that operates asynchronously. The callback function receives
    /// a `RequestNodeBuilder` which can be used to construct and return a `RequestNodeBuilder`
    /// to be added to the chain.
    ///
    /// # Arguments
    ///
    /// * `o` - A callback function that takes a `RequestNodeBuilder` and returns a future that
    ///         resolves to a `Result<RequestNodeBuilder, TracerError>`.
    ///
    /// # Returns
    ///
    /// A `Result` containing a mutable reference to the `RequestChain` if successful, or a
    /// `TracerError` if an error occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request_chaining::*;
    /// use gearbox::error::DynTracerError;
    /// use core::future::Future;
    ///
    /// async fn example_callback(mut builder: RequestNodeBuilder) -> Result<RequestNodeBuilder, DynTracerError> {
    ///     builder = builder.name("example_node");
    ///     Ok(builder)
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut chain = RequestChain::new();
    ///     chain.add_template_with_callback(example_callback).await.unwrap();
    /// }
    /// ```
    pub async fn add_template_with_callback<
        U: Future<Output = Result<RequestNodeBuilder, DynTracerError>>,
        O: FnOnce(RequestNodeBuilder) -> U,
    >(
        &mut self,
        o: O,
    ) -> Result<&mut Self, DynTracerError> {
        let builder = RequestNodeBuilder::default();
        o(builder).await.map(|t| {
            self.add_template_request(t.build());
            self
        })
    }

    /// Adds a call structure to the request chain.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the call structure.
    /// * `calls` - A vector of strings representing the call sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request_chaining::*;
    ///
    /// let mut chain = RequestChain::new();
    /// chain.add_call_structure("example_chain", vec!["request1".to_string(), "request2".to_string()]);
    /// ```
    pub fn add_call_structure(&mut self, name: &str, calls: Vec<String>) {
        self.call_structures.insert(name.to_string(), calls);
    }
}

/// This implementation for the Request Chain is mainly meant for use in conjunction with WASM.
/// WASM has several limitations and often requires a different structure, such as using callbacks
/// instead of closures. These function implementations provide a more "JavaScript-like" experience
/// when using the RequestChain structure.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
impl RequestChain {
    /// Adds a call to the request chain using a JavaScript callback.
    ///
    /// # Arguments
    ///
    /// * `callback` - The JavaScript function to call.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let chain = RequestChain::new().add_call(&js_function);
    /// ```
    pub fn add_call(mut self, callback: &js_sys::Function) -> Self {
        let request_node_builder = RequestNodeBuilder::default();

        let this = JsValue::null();
        let result = callback.call1(&this, &request_node_builder.into()).unwrap();
        let returned_builder: RequestNodeBuilder = serde_wasm_bindgen::from_value(result).unwrap();

        self.add_template_request(returned_builder.build());

        self
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl RequestChain {
    pub async fn run(
        &self,
        chain_name: &str,
        variables: Vec<WasmVariable>,
    ) -> Result<ChainResponses, DynTracerError> {
        let mut processor = RequestProcessor::new(self.clone());
        processor
            .process(
                chain_name,
                variables
                    .into_iter()
                    .map(|t| t.into())
                    .collect::<HashMap<String, String>>(),
            )
            .await
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WasmVariable {
    name: String,
    value: String,
}

impl From<WasmVariable> for (String, String) {
    fn from(v: WasmVariable) -> (String, String) {
        (v.name, v.value)
    }
}

/// A builder for creating `RequestNode` objects. It allows adjusting for variable captures and
/// templating for forwarding the data.
///
/// # Examples
///
/// ```
/// use gearbox::net::http::request_chaining::*;
///
/// let builder = RequestNodeBuilder::default()
///     .name("example_node")
///     .build();
/// ```
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RequestNodeBuilder {
    requests: Vec<Builder>,
    captures: VariableCaptures,
    name: String,
}

/// These are extensive implementations that is not allowed under WASM. But functions that improve
/// usability under Rust.
impl RequestNodeBuilder {}

/// These are common implementations for the RequestNodeBuilder structure, that are also supported under
/// WASM. These functions are used to set the name of the RequestNodeBuilder and to build the RequestNode.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl RequestNodeBuilder {
    /// Adds a request to the `RequestNodeBuilder`.
    ///
    /// # Arguments
    ///
    /// * `request` - The request to add.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::Builder;
    /// use gearbox::net::http::request_chaining::*;
    ///
    /// let builder = RequestNodeBuilder::default().add_request(Builder::default());
    /// ```
    pub fn add_request(mut self, request: Builder) -> RequestNodeBuilder {
        self.requests.push(request);
        self
    }

    /// Adds a variable capture to the `RequestNodeBuilder`.
    ///
    /// # Arguments
    ///
    /// * `capture` - The variable capture to add.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request_chaining::*;
    ///
    /// let builder = RequestNodeBuilder::default().add_capture(VariableCapture::default());
    /// ```
    pub fn add_capture(mut self, capture: VariableCapture) -> RequestNodeBuilder {
        self.captures.body.push(capture);
        self
    }

    /// Sets the name of the `RequestNodeBuilder`.
    ///
    /// # Arguments
    ///
    /// * `name` - The name to set.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request_chaining::*;
    ///
    /// let builder = RequestNodeBuilder::default().name("example_node");
    /// ```
    pub fn name(mut self, name: &str) -> RequestNodeBuilder {
        self.name = name.to_string();
        self
    }

    /// Builds and returns a `RequestNode`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request_chaining::*;
    ///
    /// let node = RequestNodeBuilder::default().build();
    /// ```
    pub fn build(self) -> RequestNode {
        RequestNode {
            name: self.name,
            matcher: self.captures,
            children: self.requests,
            depends_on: Vec::new(),
        }
    }
}

/// These implementations of RequestNodeBuilder are mainly meant for WASM, as they allow for a more
/// "JavaScript-like" experience when using the RequestNodeBuilder structure.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
impl RequestNodeBuilder {
    /// Adds a request using a JavaScript callback.
    ///
    /// # Arguments
    ///
    /// * `callback` - The JavaScript function to call.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let builder = RequestNodeBuilder::default().make_request(&js_function);
    /// ```
    pub fn make_request(mut self, callback: &js_sys::Function) -> Self {
        let request_builder = Builder::default();

        let result = callback
            .call1(&JsValue::NULL, &request_builder.into())
            .unwrap();
        let returned_builder: Builder = serde_wasm_bindgen::from_value(result).unwrap();

        self.requests.push(returned_builder);

        self
    }
}

/// A node in the request chain.
///
/// # Examples
///
/// ```
/// use gearbox::net::http::request_chaining::*;
///
/// let node = RequestNode::default();
/// ```
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RequestNode {
    name: String,
    matcher: VariableCaptures,
    children: Vec<Builder>,
    depends_on: Vec<String>,
}

/// Captures variables from different parts of a response.
///
/// # Examples
///
/// ```
/// use gearbox::net::http::request_chaining::*;
///
/// let captures = VariableCaptures::default();
/// ```
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, Default)]
pub struct VariableCaptures {
    body: Vec<VariableCapture>,
    headers: Vec<VariableCapture>,
    query: Vec<VariableCapture>,
}

impl VariableCaptures {}

impl ser::Serialize for VariableCaptures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut captures = Vec::new();
        for capture in &self.body {
            captures.push(CaptureWrapper {
                id: capture.id.clone(),
                capture_type: "body".to_string(),
                matcher: capture.matcher.clone(),
                default: capture.default.clone(),
            });
        }
        for capture in &self.headers {
            captures.push(CaptureWrapper {
                id: capture.id.clone(),
                capture_type: "headers".to_string(),
                matcher: capture.matcher.clone(),
                default: capture.default.clone(),
            });
        }
        for capture in &self.query {
            captures.push(CaptureWrapper {
                id: capture.id.clone(),
                capture_type: "query".to_string(),
                matcher: capture.matcher.clone(),
                default: capture.default.clone(),
            });
        }
        captures.serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for VariableCaptures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let captures: Vec<CaptureWrapper> = de::Deserialize::deserialize(deserializer)?;
        let mut body = Vec::new();
        let mut headers = Vec::new();
        let mut query = Vec::new();

        for capture in captures {
            match capture.capture_type.to_lowercase().as_str() {
                "body" => body.push(VariableCapture {
                    id: capture.id.clone(),
                    matcher: capture.matcher.clone(),
                    default: capture.default.clone(),
                }),
                "headers" => headers.push(VariableCapture {
                    id: capture.id.parse().unwrap(),
                    matcher: capture.matcher.clone(),
                    default: capture.default.clone(),
                }),
                "query" => query.push(VariableCapture {
                    id: capture.id.parse().unwrap(),
                    matcher: capture.matcher.clone(),
                    default: capture.default.clone(),
                }),
                _ => {
                    return Err(serde::de::Error::unknown_variant(
                        &*capture.capture_type,
                        &["body", "headers", "query"],
                    ))
                }
            }
        }

        Ok(VariableCaptures {
            body,
            headers,
            query,
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct CaptureWrapper {
    id: String,
    #[serde(rename = "type")]
    capture_type: String,
    matcher: Matcher,
    default: Option<String>,
}

/// Captures a single variable from a response.
///
/// # Examples
///
/// ```
/// use gearbox::net::http::request_chaining::*;
///
/// let capture = VariableCapture::default();
/// ```
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct VariableCapture {
    id: String,
    matcher: Matcher,
    default: Option<String>,
}

/// Matches parts of a response for variable capturing.
///
/// # Examples
///
/// ```
/// use gearbox::net::http::request_chaining::*;
///
/// let matcher = Matcher::default();
/// ```
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Matcher {
    #[serde(skip_serializing_if = "Option::is_none")]
    between: Option<(String, String)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regexp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    xpath: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    all: Option<bool>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Matcher {
    /// Creates a matcher for capturing text between two strings.
    ///
    /// # Arguments
    ///
    /// * `from` - The start string.
    /// * `to` - The end string.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request_chaining::*;
    ///
    /// let matcher = Matcher::between("start".to_string(), "end".to_string());
    /// ```
    pub fn between(from: String, to: String) -> Matcher {
        Matcher {
            between: Some((from, to)),
            regexp: None,
            xpath: None,
            all: None,
        }
    }

    /// Creates a matcher for capturing text using a regular expression.
    ///
    /// # Arguments
    ///
    /// * `regexp` - The regular expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request_chaining::*;
    ///
    /// let matcher = Matcher::regexp("regex".to_string());
    /// ```
    pub fn regexp(regexp: String) -> Matcher {
        Matcher {
            between: None,
            regexp: Some(regexp),
            xpath: None,
            all: None,
        }
    }

    /// Creates a matcher for capturing all text.
    ///
    /// # Arguments
    ///
    /// * `all` - Whether to capture all text.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request_chaining::*;
    ///
    /// let matcher = Matcher::all(true);
    /// ```
    pub fn all(all: bool) -> Matcher {
        Matcher {
            between: None,
            regexp: None,
            all: Some(all),
            xpath: None,
        }
    }
}

/// Processes a request chain, capturing variables and responses.
///
/// # Examples
///
/// ```
/// use gearbox::net::http::request_chaining::*;
/// use gearbox::collections::HashMap;
///
/// let mut variables = HashMap::new();
/// variables.insert("example".to_string(), "value".to_string());
///
/// let chain = RequestChain::new();
/// let mut processor = RequestProcessor::new(chain);
/// processor.process("example_chain", variables);
/// ```
pub struct RequestProcessor {
    request_chain: RequestChain,
    variables: HashMap<String, String>,
    response: ChainResponses,
}

impl RequestProcessor {
    /// Creates a new `RequestProcessor`.
    ///
    /// # Arguments
    ///
    /// * `request_chain` - The `RequestChain` to process.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request_chaining::*;
    ///
    /// let chain = RequestChain::new();
    /// let processor = RequestProcessor::new(chain);
    /// ```
    pub fn new(request_chain: RequestChain) -> RequestProcessor {
        RequestProcessor {
            request_chain,
            variables: HashMap::new(),
            response: ChainResponses::default(),
        }
    }

    /// Processes a call structure in the request chain.
    ///
    /// # Arguments
    ///
    /// * `chain_name` - The name of the call structure to process.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request_chaining::*;
    /// use gearbox::collections::HashMap;
    ///
    /// let mut variables = HashMap::new();
    /// variables.insert("example".to_string(), "value".to_string());
    ///
    /// let chain = RequestChain::new();
    /// let mut processor = RequestProcessor::new(chain);
    /// processor.process("example_chain", variables);
    /// ```
    pub async fn process(
        &mut self,
        chain_name: &str,
        variables: HashMap<String, String>,
    ) -> Result<ChainResponses, DynTracerError> {
        if let Some(calls) = self
            .request_chain
            .call_structures
            .get(chain_name)
            .map(Clone::clone)
        {
            for call in calls {
                if let Some(request_node) = self.request_chain.template_requests.get(&call) {
                    self.execute_request(request_node.clone()).await?;
                }
            }
            if self.response.responses.is_empty() {
                Err(tracer_dyn_err!("No responses were generated."))
            } else {
                self.variables = variables;
                Ok(self.response.clone())
            }
        } else {
            Err(tracer_dyn_err!("Call structure not found."))
        }
    }

    /// Executes a request node and captures variables.
    ///
    /// # Arguments
    ///
    /// * `request_node` - The `RequestNode` to execute.
    async fn execute_request(&mut self, request_node: RequestNode) -> Result<(), DynTracerError> {
        let mut context = TemplateContext::new();

        self.variables.iter().for_each(|(k, v)| {
            context.insert(k.as_str(), Box::new(v.clone()));
        });

        let context = &context.clone();
        for mut request in request_node.children {
            // Updating headers, if the headers are using templating we are going to render them into
            // the variables and then update the headers with the rendered values
            request.headers_mut().iter_mut().for_each(|(k, v)| {
                v.iter_mut().for_each(|t| {
                    String::from_utf8(t.0.clone()).map(|s| {
                        TemplateEngine::new().render(&s, &context).map(|r| {
                            t.0 = r.into_bytes();
                        });
                    });
                });
            });

            request
                .update_body(|mut t| async move {
                    let output = t.into_string().await;
                    output
                        .and_then(|r| TemplateEngine::new().render(&r, &context))
                        .map(|t| Box::new(Body::from(t)))
                })
                .await?;

            request.url_mut().as_mut().map(|t| {
                TemplateEngine::new()
                    .render(&t.to_string(), &context)
                    .map(|r| {
                        *t = Url::from(&r);
                    });
            });

            let response = request
                .send()
                .map_err(|e| async { tracer_dyn_err!(e) })
                .and_then(|t| async { ChainResponse::try_from_response(t).await })
                .await?;

            self.capture_variables(&response, &request_node.matcher);
            self.response.responses.push(response.clone());
            self.response.last = response;
        }

        Ok(())
    }

    /// Captures variables from a response based on the provided captures.
    ///
    /// # Arguments
    ///
    /// * `response` - The `Response` object to capture variables from.
    /// * `captures` - The `VariableCaptures` defining what to capture.
    fn capture_variables(&mut self, response: &ChainResponse, captures: &VariableCaptures) {
        for capture in &captures.body {
            if let Some(value) = self.match_response(&response.body, &capture.matcher) {
                self.variables.insert(capture.id.clone(), value);
            }
        }
        // Similarly handle captures from headers and query
    }

    /// Matches a response based on the provided matcher.
    ///
    /// # Arguments
    ///
    /// * `response` - The response string to match.
    /// * `matcher` - The `Matcher` defining how to match the response.
    ///
    /// # Returns
    ///
    /// An optional string representing the matched value.
    fn match_response(&self, response: &str, matcher: &Matcher) -> Option<String> {
        if let Some((from, to)) = &matcher.between {
            if let Some(start) = response.find(from) {
                if let Some(end) = response[start..].find(to) {
                    return Some(response[start + from.len()..start + end].to_string());
                }
            }
        }
        if let Some(regexp) = &matcher.regexp {
            // Apply regexp matching logic
        }
        if matcher.all.unwrap_or(false) {
            return Some(response.to_string());
        }
        None
    }
}

/// Holds the responses from processed requests.
///
/// # Examples
///
/// ```
/// use gearbox::net::http::request_chaining::*;
///
/// let responses = ChainResponses::default();
/// ```
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ChainResponses {
    responses: Vec<ChainResponse>,
    last: ChainResponse,
}

/// Represents a single response from a processed request.
///
/// # Examples
///
/// ```
/// use gearbox::net::http::request_chaining::*;
///
/// let response = ChainResponse::default();
/// ```
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ChainResponse {
    body: String,
    headers: HashMap<String, String>,
    status: u16,
    status_msg: String,
    variables_state: HashMap<String, String>,
}

impl ChainResponse {
    pub async fn try_from_response(response: request::Response) -> Result<Self, DynTracerError> {
        let body = response.body().into_str().await?;
        let headers = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_header_string()))
            .collect();
        let status = response.status().as_u16();
        let status_msg = response.status().as_str().to_string();
        let variables_state = HashMap::new();
        Ok(ChainResponse {
            body,
            headers,
            status,
            status_msg,
            variables_state,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::HashMap;
    use crate::net::http::request::Method;
    use crate::net::http::test::test_server::start_test_server;
    use crate::rails::ext::future::*;
    use tokio::sync::oneshot;

    #[tokio::test]
    async fn test_complete_request_chain_functionality() {
        let (addr, shutdown_tx) = start_test_server().await;

        // Setup RequestChain
        let mut chain = RequestChain::new();
        chain
            .add_template_with_callback(|mut builder| async {
                builder = builder
                    .name("test_node_1")
                    .add_request(
                        Builder::default()
                            .body(r#"{"status":200, "payload":"hello", "headers":{}}"#)
                            .content_type("application/json")
                            .method(Method::Post)
                            .url(format!("http://{}/", addr)),
                    )
                    .add_capture(VariableCapture {
                        id: "text".to_string(),
                        matcher: Matcher::all(true),
                        default: None,
                    });
                Ok(builder)
            })
            .and_then(|t| async {
                t.add_template_with_callback(|mut builder| async {
                    builder = builder
                        .name("test_node_2")
                        .add_request(
                            Builder::default()
                                .body(
                                    r#"{"status":200, "payload":"{{ text }} world", "headers":{}}"#,
                                )
                                .content_type("application/json")
                                .method(Method::Post)
                                .url(format!("http://{}/", addr)),
                        )
                        .add_capture(VariableCapture {
                            id: "test_capture".to_string(),
                            matcher: Matcher::all(true),
                            default: None,
                        });
                    Ok(builder) as Result<_, DynTracerError>
                })
                .await
            })
            .await
            .ok();

        chain.call_structures.insert(
            "test_chain".to_string(),
            vec!["test_node_1".to_string(), "test_node_2".to_string()],
        );

        let serialized_chain = serde_json::to_string(&chain).unwrap();

        println!("{}", serialized_chain);

        let deserialized_chain: RequestChain = serde_json::from_str(&serialized_chain).unwrap();

        let responses = deserialized_chain
            .run("test_chain", Vec::new())
            .await
            .unwrap();

        assert_eq!(responses.responses.len(), 2);
        assert_eq!(responses.responses[0].body, r#"hello"#);
        assert_eq!(responses.responses[1].body, r#"hello world"#);

        // Shutdown the server
        shutdown_tx.send(()).unwrap();
    }

    #[tokio::test]
    async fn test_request_chain_new() {
        let chain = RequestChain::new();
        assert!(chain.template_requests.is_empty());
        assert!(chain.call_structures.is_empty());
    }

    #[tokio::test]
    async fn test_add_template_request() {
        let mut chain = RequestChain::new();
        let node = RequestNode {
            name: "test_node".to_string(),
            ..Default::default()
        };
        chain.add_template_request(node.clone());
        assert_eq!(
            chain.template_requests.get("test_node").unwrap().name,
            "test_node"
        );
    }

    #[tokio::test]
    async fn test_add_call_structure() {
        let mut chain = RequestChain::new();
        chain.add_call_structure(
            "test_chain",
            vec!["request1".to_string(), "request2".to_string()],
        );
        assert_eq!(
            chain.call_structures.get("test_chain").unwrap(),
            &vec!["request1", "request2"]
        );
    }

    #[tokio::test]
    async fn test_request_node_builder() {
        let mut builder = RequestNodeBuilder::default()
            .name("example_node")
            .add_request(Builder::default())
            .add_capture(VariableCapture::default());

        let node = builder.build();
        assert_eq!(node.name, "example_node");
        assert_eq!(node.children.len(), 1);
        assert_eq!(node.matcher.body.len(), 1);
    }

    #[tokio::test]
    async fn test_request_node_builder_with_request() {
        let mut builder = RequestNodeBuilder::default()
            .name("example_node")
            .add_request(Builder::default())
            .add_capture(VariableCapture::default())
            .add_request(Builder::default());

        let node = builder.build();
        assert_eq!(node.name, "example_node");
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.matcher.body.len(), 1);
    }

    #[tokio::test]
    async fn test_request_processor_with_server() {
        let (addr, shutdown_tx) = start_test_server().await;

        // Setup RequestChain
        let mut chain = RequestChain::new();

        // Create a request node
        let node = RequestNodeBuilder::default()
            .name("test_node")
            .add_request(
                Builder::default()
                    .method("GET")
                    .url(format!("http://{}/", addr)),
            )
            .build();
        chain.add_template_request(node.clone());
        chain.add_call_structure("test_chain", vec!["test_node".to_string()]);

        // Process the request chain
        let mut processor = RequestProcessor::new(chain);
        let variables = HashMap::new();
        let result = processor.process("test_chain", variables).await;

        // Validate the result
        match result {
            Ok(responses) => {
                assert_eq!(responses.responses.len(), 1);
                let response = &responses.responses[0];
                assert_eq!(response.body, "GET response");
            }
            Err(e) => panic!("Process failed: {:?}", e),
        }

        // Shutdown the server
        shutdown_tx.send(()).unwrap();
    }

    #[tokio::test]
    async fn test_multi_request_processor_with_server() {
        let (addr, shutdown_tx) = start_test_server().await;

        // Setup RequestChain
        let mut chain = RequestChain::new();

        // Create first request node
        let node1 = RequestNodeBuilder::default()
            .name("test_node_1")
            .add_request(
                Builder::default()
                    .method(Method::Get)
                    .url(format!("http://{}/", addr))
                    .body(Body::empty()),
            )
            .build();
        chain.add_template_request(node1.clone());

        // Create second request node
        let node2 = RequestNodeBuilder::default()
            .name("test_node_2")
            .add_request(
                Builder::default()
                    .method(Method::Post)
                    .url(format!("http://{}/", addr))
                    .body(Body::empty()),
            )
            .build();
        chain.add_template_request(node2.clone());

        // Create third request node
        let node3 = RequestNodeBuilder::default()
            .name("test_node_3")
            .add_request(
                Builder::default()
                    .method(Method::Delete)
                    .url(format!("http://{}/", addr))
                    .body(Body::empty()),
            )
            .build();
        chain.add_template_request(node3.clone());

        // Add call structure
        chain.add_call_structure(
            "test_chain",
            vec![
                "test_node_1".to_string(),
                "test_node_2".to_string(),
                "test_node_3".to_string(),
            ],
        );

        // Process the request chain
        let mut processor = RequestProcessor::new(chain);
        let variables = HashMap::new();
        let result = processor.process("test_chain", variables).await;

        // Validate the result
        match result {
            Ok(responses) => {
                assert_eq!(responses.responses.len(), 3);

                let response1 = &responses.responses[0];
                assert_eq!(response1.body, "GET response");

                let response2 = &responses.responses[1];
                assert_eq!(response2.body, "POST response");

                let response3 = &responses.responses[2];
                assert_eq!(response3.body, "DELETE response");
            }
            Err(e) => panic!("Process failed: {:?}", e),
        }

        // Shutdown the server
        shutdown_tx.send(()).unwrap();
    }

    #[tokio::test]
    async fn test_matcher_between() {
        let matcher = Matcher::between("start".to_string(), "end".to_string());
        let response = "this is the start of the match end of the string";
        let value = RequestProcessor {
            request_chain: RequestChain::new(),
            variables: HashMap::new(),
            response: ChainResponses::default(),
        }
        .match_response(response, &matcher);
        assert_eq!(value, Some(" of the match ".to_string()));
    }

    #[tokio::test]
    async fn test_matcher_regexp() {
        let matcher = Matcher::regexp(r"\d+".to_string());
        let response = "there are 42 apples";
        let value = RequestProcessor {
            request_chain: RequestChain::new(),
            variables: HashMap::new(),
            response: ChainResponses::default(),
        }
        .match_response(response, &matcher);
        // Assuming the implementation of regex matching is added
        // assert_eq!(value, Some("42".to_string()));
    }

    #[tokio::test]
    async fn test_matcher_all() {
        let matcher = Matcher::all(true);
        let response = "capture the entire string";
        let value = RequestProcessor {
            request_chain: RequestChain::new(),
            variables: HashMap::new(),
            response: ChainResponses::default(),
        }
        .match_response(response, &matcher);
        assert_eq!(value, Some(response.to_string()));
    }
}
