use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::ops::Deref;

#[derive(Debug)]
pub struct StatusCode {
    code: u16,
    reason: &'static str,
}

impl StatusCode {
    pub fn as_u16(&self) -> u16 {
        self.code
    }

    pub fn as_str(&self) -> &'static str {
        self.reason
    }
}

impl Deref for StatusCode {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.code
    }
}

impl From<reqwest::StatusCode> for StatusCode {
    fn from(status: reqwest::StatusCode) -> Self {
        StatusCode::from(status.as_u16())
    }
}

impl From<u16> for StatusCode {
    fn from(code: u16) -> Self {
        (&code).into()
    }
}

impl From<&u16> for StatusCode {
    fn from(code: &u16) -> Self {
        match code {
            100 => StatusCode::CONTINUE,
            101 => StatusCode::SWITCHING_PROTOCOLS,
            102 => StatusCode::PROCESSING,
            103 => StatusCode::EARLY_HINTS,
            200 => StatusCode::OK,
            201 => StatusCode::CREATED,
            202 => StatusCode::ACCEPTED,
            203 => StatusCode::NON_AUTHORITATIVE_INFORMATION,
            204 => StatusCode::NO_CONTENT,
            205 => StatusCode::RESET_CONTENT,
            206 => StatusCode::PARTIAL_CONTENT,
            207 => StatusCode::MULTI_STATUS,
            208 => StatusCode::ALREADY_REPORTED,
            226 => StatusCode::IM_USED_HTTP_DELTA_ENCODING_,
            300 => StatusCode::MULTIPLE_CHOICES,
            301 => StatusCode::MOVED_PERMANENTLY,
            302 => StatusCode::FOUND,
            303 => StatusCode::SEE_OTHER,
            304 => StatusCode::NOT_MODIFIED,
            305 => StatusCode::USE_PROXY,
            306 => StatusCode::UNUSED,
            307 => StatusCode::TEMPORARY_REDIRECT,
            308 => StatusCode::PERMANENT_REDIRECT,
            400 => StatusCode::BAD_REQUEST,
            401 => StatusCode::UNAUTHORIZED,
            402 => StatusCode::PAYMENT_REQUIRED_EXPERIMENTAL,
            403 => StatusCode::FORBIDDEN,
            404 => StatusCode::NOT_FOUND,
            405 => StatusCode::METHOD_NOT_ALLOWED,
            406 => StatusCode::NOT_ACCEPTABLE,
            407 => StatusCode::PROXY_AUTHENTICATION_REQUIRED,
            408 => StatusCode::REQUEST_TIMEOUT,
            409 => StatusCode::CONFLICT,
            410 => StatusCode::GONE,
            411 => StatusCode::LENGTH_REQUIRED,
            412 => StatusCode::PRECONDITION_FAILED,
            413 => StatusCode::PAYLOAD_TOO_LARGE,
            414 => StatusCode::URI_TOO_LONG,
            415 => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            416 => StatusCode::RANGE_NOT_SATISFIABLE,
            417 => StatusCode::EXPECTATION_FAILED,
            418 => StatusCode::IM_A_TEAPOT,
            421 => StatusCode::MISDIRECTED_REQUEST,
            422 => StatusCode::UNPROCESSABLE_CONTENT,
            423 => StatusCode::LOCKED,
            424 => StatusCode::FAILED_DEPENDENCY,
            425 => StatusCode::TOO_EARLY_EXPERIMENTAL,
            _ => StatusCode {
                code: *code,
                reason: "Unknown",
            },
        }
    }
}
impl From<&'static str> for StatusCode {
    fn from(reason: &'static str) -> Self {
        let normalized_reason = reason
            .to_string()
            .to_uppercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
            .collect::<String>()
            .replace(&[' ', '-', '\''][..], "_")
            .split('_')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("_");
        match normalized_reason.as_str() {
            "CONTINUE" => StatusCode::CONTINUE,
            "SWITCHING_PROTOCOLS" => StatusCode::SWITCHING_PROTOCOLS,
            "PROCESSING" => StatusCode::PROCESSING,
            "EARLY_HINTS" => StatusCode::EARLY_HINTS,
            "OK" => StatusCode::OK,
            "CREATED" => StatusCode::CREATED,
            "ACCEPTED" => StatusCode::ACCEPTED,
            "NON_AUTHORITATIVE_INFORMATION" => StatusCode::NON_AUTHORITATIVE_INFORMATION,
            "NO_CONTENT" => StatusCode::NO_CONTENT,
            "RESET_CONTENT" => StatusCode::RESET_CONTENT,
            "PARTIAL_CONTENT" => StatusCode::PARTIAL_CONTENT,
            "MULTI_STATUS" => StatusCode::MULTI_STATUS,
            "ALREADY_REPORTED" => StatusCode::ALREADY_REPORTED,
            "IM_USED_HTTP_DELTA_ENCODING_" => StatusCode::IM_USED_HTTP_DELTA_ENCODING_,
            "MULTIPLE_CHOICES" => StatusCode::MULTIPLE_CHOICES,
            "MOVED_PERMANENTLY" => StatusCode::MOVED_PERMANENTLY,
            "FOUND" => StatusCode::FOUND,
            "SEE_OTHER" => StatusCode::SEE_OTHER,
            "NOT_MODIFIED" => StatusCode::NOT_MODIFIED,
            "USE_PROXY" => StatusCode::USE_PROXY,
            "UNUSED" => StatusCode::UNUSED,
            "TEMPORARY_REDIRECT" => StatusCode::TEMPORARY_REDIRECT,
            "PERMANENT_REDIRECT" => StatusCode::PERMANENT_REDIRECT,
            "BAD_REQUEST" => StatusCode::BAD_REQUEST,
            "UNAUTHORIZED" => StatusCode::UNAUTHORIZED,
            "PAYMENT_REQUIRED_EXPERIMENTAL" => StatusCode::PAYMENT_REQUIRED_EXPERIMENTAL,
            "FORBIDDEN" => StatusCode::FORBIDDEN,
            "NOT_FOUND" => StatusCode::NOT_FOUND,
            "METHOD_NOT_ALLOWED" => StatusCode::METHOD_NOT_ALLOWED,
            "NOT_ACCEPTABLE" => StatusCode::NOT_ACCEPTABLE,
            "PROXY_AUTHENTICATION_REQUIRED" => StatusCode::PROXY_AUTHENTICATION_REQUIRED,
            "REQUEST_TIMEOUT" => StatusCode::REQUEST_TIMEOUT,
            "CONFLICT" => StatusCode::CONFLICT,
            "GONE" => StatusCode::GONE,
            "LENGTH_REQUIRED" => StatusCode::LENGTH_REQUIRED,
            "PRECONDITION_FAILED" => StatusCode::PRECONDITION_FAILED,
            "PAYLOAD_TOO_LARGE" => StatusCode::PAYLOAD_TOO_LARGE,
            "URI_TOO_LONG" => StatusCode::URI_TOO_LONG,
            "UNSUPPORTED_MEDIA_TYPE" => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "RANGE_NOT_SATISFIABLE" => StatusCode::RANGE_NOT_SATISFIABLE,
            "EXPECTATION_FAILED" => StatusCode::EXPECTATION_FAILED,
            "IM_A_TEAPOT" => StatusCode::IM_A_TEAPOT,
            "MISDIRECTED_REQUEST" => StatusCode::MISDIRECTED_REQUEST,
            "UNPROCESSABLE_CONTENT" => StatusCode::UNPROCESSABLE_CONTENT,
            "LOCKED" => StatusCode::LOCKED,
            "FAILED_DEPENDENCY" => StatusCode::FAILED_DEPENDENCY,
            "TOO_EARLY_EXPERIMENTAL" => StatusCode::TOO_EARLY_EXPERIMENTAL,
            "UPGRADE_REQUIRED" => StatusCode::UPGRADE_REQUIRED,
            "PRECONDITION_REQUIRED" => StatusCode::PRECONDITION_REQUIRED,
            "TOO_MANY_REQUESTS" => StatusCode::TOO_MANY_REQUESTS,
            "REQUEST_HEADER_FIELDS_TOO_LARGE" => StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
            "UNAVAILABLE_FOR_LEGAL_REASONS" => StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
            "INTERNAL_SERVER_ERROR" => StatusCode::INTERNAL_SERVER_ERROR,
            "NOT_IMPLEMENTED" => StatusCode::NOT_IMPLEMENTED,
            "BAD_GATEWAY" => StatusCode::BAD_GATEWAY,
            "SERVICE_UNAVAILABLE" => StatusCode::SERVICE_UNAVAILABLE,
            "GATEWAY_TIMEOUT" => StatusCode::GATEWAY_TIMEOUT,
            "HTTP_VERSION_NOT_SUPPORTED" => StatusCode::HTTP_VERSION_NOT_SUPPORTED,
            "VARIANT_ALSO_NEGOTIATES" => StatusCode::VARIANT_ALSO_NEGOTIATES,
            "INSUFFICIENT_STORAGE" => StatusCode::INSUFFICIENT_STORAGE,
            "LOOP_DETECTED" => StatusCode::LOOP_DETECTED,
            "NOT_EXTENDED" => StatusCode::NOT_EXTENDED,
            "NETWORK_AUTHENTICATION_REQUIRED" => StatusCode::NETWORK_AUTHENTICATION_REQUIRED,
            _ => StatusCode {
                code: 0,
                reason: reason,
            },
        }
    }
}

impl From<(&u16, &'static str)> for StatusCode {
    fn from((code, reason): (&u16, &'static str)) -> Self {
        if reason.is_empty() {
            (code).into()
        } else {
            StatusCode {
                code: *code,
                reason: reason,
            }
        }
    }
}

impl From<(&u16, Option<&'static str>)> for StatusCode {
    fn from((code, reason): (&u16, Option<&'static str>)) -> Self {
        match reason {
            Some(reason) => StatusCode {
                code: *code,
                reason: reason,
            },
            None => code.into(),
        }
    }
}

macro_rules! status_codes
{
    (
        $(
            $(#[$docs:meta])*
            ($name:ident, $code:literal, $reason:literal);
        )+
    ) => {
        impl StatusCode {
            $(
                $(#[$docs])*
                pub const $name: StatusCode = StatusCode { code: $code, reason: $reason };
            )+
        }
    }
}

status_codes! {
    /// This interim response indicates that the client should continue the request or ignore the response if the request is already finished.
    (CONTINUE, 100, "Continue");
    /// This code is sent in response to an Upgrade request header from the client and indicates the protocol the server is switching to.
    (SWITCHING_PROTOCOLS, 101, "Switching Protocols");
    /// This code indicates that the server has received and is task the request, but no response is available yet.
    (PROCESSING, 102, "Processing (WebDAV)");
    /// This status code is primarily intended to be used with the Link header, letting the user agent start preloading resources while the server prepares a response or preconnect to an origin from which the page will need resources.Successful responses
    (EARLY_HINTS, 103, "Early Hints");
    /// The request succeeded. The result meaning of "success" depends on the HTTP method:\nGET: The resource has been fetched and transmitted in the message body.\nHEAD: The representation headers are included in the response without any message body.\nPUT or POST: The resource describing the result of the action is transmitted in the message body.\nTRACE: The message body contains the request message as received by the server.
    (OK, 200, "OK");
    /// The request succeeded, and a new resource was created as a result. This is typically the response sent after POST requests, or some PUT requests.
    (CREATED, 201, "Created");
    /// The request has been received but not yet acted upon. It is noncommittal, since there is no way in HTTP to later send an asynchronous response indicating the outcome of the request. It is intended for cases where another process or server handles the request, or for batch task.
    (ACCEPTED, 202, "Accepted");
    /// This response code means the returned metadata is not exactly the same as is available from the origin server, but is collected from a local or a third-party copy. This is mostly used for mirrors or backups of another resource. Except for that specific case, the 200 OK response is preferred to this status.
    (NON_AUTHORITATIVE_INFORMATION, 203, "Non-Authoritative Information");
    /// There is no content to send for this request, but the headers may be useful. The user agent may update its cached headers for this resource with the new ones.
    (NO_CONTENT, 204, "No Content");
    /// Tells the user agent to reset the document which sent this request.
    (RESET_CONTENT, 205, "Reset Content");
    /// This response code is used when the Range header is sent from the client to request only part of a resource.
    (PARTIAL_CONTENT, 206, "Partial Content");
    /// Conveys information about multiple resources, for situations where multiple status codes might be appropriate.
    (MULTI_STATUS, 207, "Multi-Status (WebDAV)");
    /// Used inside a <dav:propstat> response element to avoid repeatedly enumerating the internal members of multiple bindings to the same collection.
    (ALREADY_REPORTED, 208, "Already Reported (WebDAV)");
    /// The server has fulfilled a GET request for the resource, and the response is a representation of the result of one or more instance-manipulations applied to the current instance.
    (IM_USED_HTTP_DELTA_ENCODING_, 226, "IM Used (HTTP Delta encoding)");
    /// The request has more than one possible response. The user agent or user should choose one of them. (There is no standardized way of choosing one of the responses, but HTML links to the possibilities are recommended so the user can pick.)
    (MULTIPLE_CHOICES, 300, "Multiple Choices");
    /// The URL of the requested resource has been changed permanently. The new URL is given in the response.
    (MOVED_PERMANENTLY, 301, "Moved Permanently");
    /// This response code means that the URI of requested resource has been changed temporarily. Further changes in the URI might be made in the future. Therefore, this same URI should be used by the client in future requests.
    (FOUND, 302, "Found");
    /// The server sent this response to direct the client to get the requested resource at another URI with a GET request.
    (SEE_OTHER, 303, "See Other");
    /// This is used for caching purposes. It tells the client that the response has not been modified, so the client can continue to use the same cached version of the response.
    (NOT_MODIFIED, 304, "Not Modified");
    /// Defined in a previous version of the HTTP specification to indicate that a requested response must be accessed by a proxy. It has been deprecated due to security concerns regarding in-band configuration of a proxy.
    (USE_PROXY, 305, "Use Proxy (Deprecated)");
    /// This response code is no longer used; it is just reserved. It was used in a previous version of the HTTP/1.1 specification.
    (UNUSED, 306, "unused");
    /// The server sends this response to direct the client to get the requested resource at another URI with the same method that was used in the prior request. This has the same semantics as the 302 Found HTTP response code, with the exception that the user agent must not change the HTTP method used: if a POST was used in the first request, a POST must be used in the second request.
    (TEMPORARY_REDIRECT, 307, "Temporary Redirect");
    /// This means that the resource is now permanently located at another URI, specified by the Location: HTTP Response header. This has the same semantics as the 301 Moved Permanently HTTP response code, with the exception that the user agent must not change the HTTP method used: if a POST was used in the first request, a POST must be used in the second request.
    (PERMANENT_REDIRECT, 308, "Permanent Redirect");
    /// The server cannot or will not process the request due to something that is perceived to be a client error (e.g., malformed request syntax, invalid request message framing, or deceptive request routing).
    (BAD_REQUEST, 400, "Bad Request");
    /// Although the HTTP standard specifies "unauthorized", semantically this response means "unauthenticated". That is, the client must authenticate itself to get the requested response.
    (UNAUTHORIZED, 401, "Unauthorized");
    /// This response code is reserved for future use. The initial aim for creating this code was using it for digital payment systems, however this status code is used very rarely and no standard convention exists.
    (PAYMENT_REQUIRED_EXPERIMENTAL, 402, "Payment Required Experimental");
    /// The client does not have access rights to the content; that is, it is unauthorized, so the server is refusing to give the requested resource. Unlike 401 Unauthorized, the client's identity is known to the server.
    (FORBIDDEN, 403, "Forbidden");
    /// The server cannot find the requested resource. In the browser, this means the URL is not recognized. In an API, this can also mean that the endpoint is valid but the resource itself does not exist. Servers may also send this response instead of 403 Forbidden to hide the existence of a resource from an unauthorized client. This response code is probably the most well known due to its frequent occurrence on the web.
    (NOT_FOUND, 404, "Not Found");
    /// The request method is known by the server but is not supported by the target resource. For example, an API may not allow calling DELETE to remove a resource.
    (METHOD_NOT_ALLOWED, 405, "Method Not Allowed");
    /// This response is sent when the web server, after performing server-driven content negotiation, doesn't find any content that conforms to the criteria given by the user agent.
    (NOT_ACCEPTABLE, 406, "Not Acceptable");
    /// This is similar to 401 Unauthorized but authentication is needed to be done by a proxy.
    (PROXY_AUTHENTICATION_REQUIRED, 407, "Proxy Authentication Required");
    /// This response is sent on an idle connection by some servers, even without any previous request by the client. It means that the server would like to shut down this unused connection. This response is used much more since some browsers, like Chrome, Firefox 27+, or IE9, use HTTP pre-connection mechanisms to speed up surfing. Also note that some servers merely shut down the connection without sending this message.
    (REQUEST_TIMEOUT, 408, "Request Timeout");
    /// This response is sent when a request conflicts with the current state of the server.
    (CONFLICT, 409, "Conflict");
    /// This response is sent when the requested content has been permanently deleted from server, with no forwarding address. Clients are expected to remove their caches and links to the resource. The HTTP specification intends this status code to be used for "limited-time, promotional services". APIs should not feel compelled to indicate resources that have been deleted with this status code.
    (GONE, 410, "Gone");
    /// Server rejected the request because the Content-Length header field is not defined and the server requires it.
    (LENGTH_REQUIRED, 411, "Length Required");
    /// The client has indicated preconditions in its headers which the server does not meet.
    (PRECONDITION_FAILED, 412, "Precondition Failed");
    /// Request entity is larger than limits defined by server. The server might close the connection or return an Retry-After header field.
    (PAYLOAD_TOO_LARGE, 413, "Payload Too Large");
    /// The URI requested by the client is longer than the server is willing to interpret.
    (URI_TOO_LONG, 414, "URI Too Long");
    /// The media format of the requested data is not supported by the server, so the server is rejecting the request.
    (UNSUPPORTED_MEDIA_TYPE, 415, "Unsupported Media Type");
    /// The range specified by the Range header field in the request cannot be fulfilled. It's possible that the range is outside the size of the target URI's data.
    (RANGE_NOT_SATISFIABLE, 416, "Range Not Satisfiable");
    /// This response code means the expectation indicated by the Expect request header field cannot be met by the server.
    (EXPECTATION_FAILED, 417, "Expectation Failed");
    /// The server refuses the attempt to brew coffee with a teapot.
    (IM_A_TEAPOT, 418, "I'm a teapot");
    /// The request was directed at a server that is not able to produce a response. This can be sent by a server that is not configured to produce responses for the combination of scheme and authority that are included in the request URI.
    (MISDIRECTED_REQUEST, 421, "Misdirected Request");
    /// The request was well-formed but was unable to be followed due to semantic errors.
    (UNPROCESSABLE_CONTENT, 422, "Unprocessable Content (WebDAV)");
    /// The resource that is being accessed is locked.
    (LOCKED, 423, "Locked (WebDAV)");
    /// The request failed due to failure of a previous request.
    (FAILED_DEPENDENCY, 424, "Failed Dependency (WebDAV)");
    /// Indicates that the server is unwilling to risk task a request that might be replayed.
    (TOO_EARLY_EXPERIMENTAL, 425, "Too Early Experimental");
    /// The server refuses to perform the request using the current protocol but might be willing to do so after the client upgrades to a different protocol. The server sends an Upgrade header in a 426 response to indicate the required protocol(s).
    (UPGRADE_REQUIRED, 426, "Upgrade Required");
    /// The origin server requires the request to be conditional. This response is intended to prevent the 'lost update' problem, where a client GETs a resource's state, modifies it and PUTs it back to the server, when meanwhile a third party has modified the state on the server, leading to a conflict.
    (PRECONDITION_REQUIRED, 428, "Precondition Required");
    /// The user has sent too many requests in a given amount of time ("rate limiting").
    (TOO_MANY_REQUESTS, 429, "Too Many Requests");
    /// The server is unwilling to process the request because its header fields are too large. The request may be resubmitted after reducing the size of the request header fields.
    (REQUEST_HEADER_FIELDS_TOO_LARGE, 431, "Request Header Fields Too Large");
    /// The user agent requested a resource that cannot legally be provided, such as a web page censored by a government.
    (UNAVAILABLE_FOR_LEGAL_REASONS, 451, "Unavailable For Legal Reasons");
    /// The server has encountered a situation it does not know how to handle.
    (INTERNAL_SERVER_ERROR, 500, "Internal Server Error");
    /// The request method is not supported by the server and cannot be handled. The only methods that servers are required to support (and therefore that must not return this code) are GET and HEAD.
    (NOT_IMPLEMENTED, 501, "Not Implemented");
    /// This error response means that the server, while working as a gateway to get a response needed to handle the request, got an invalid response.
    (BAD_GATEWAY, 502, "Bad Gateway");
    /// The server is not ready to handle the request. Common causes are a server that is down for maintenance or that is overloaded. Note that together with this response, a user-friendly page explaining the problem should be sent. This response should be used for temporary conditions and the Retry-After HTTP header should, if possible, contain the estimated time before the recovery of the service. The webmaster must also take care about the caching-related headers that are sent along with this response, as these temporary condition responses should usually not be cached.
    (SERVICE_UNAVAILABLE, 503, "Service Unavailable");
    /// This error response is given when the server is acting as a gateway and cannot get a response in time.
    (GATEWAY_TIMEOUT, 504, "Gateway Timeout");
    /// The HTTP version used in the request is not supported by the server.
    (HTTP_VERSION_NOT_SUPPORTED, 505, "HTTP Version Not Supported");
    /// The server has an internal configuration error: the chosen variant resource is configured to engage in transparent content negotiation itself, and is therefore not a proper end point in the negotiation process.
    (VARIANT_ALSO_NEGOTIATES, 506, "Variant Also Negotiates");
    /// The method could not be performed on the resource because the server is unable to store the representation needed to successfully complete the request.
    (INSUFFICIENT_STORAGE, 507, "Insufficient Storage (WebDAV)");
    /// The server detected an infinite loop while task the request.
    (LOOP_DETECTED, 508, "Loop Detected (WebDAV)");
    /// Further extensions to the request are required for the server to fulfill it.
    (NOT_EXTENDED, 510, "Not Extended");
    /// Indicates that the client needs to authenticate to gain network access.
    (NETWORK_AUTHENTICATION_REQUIRED, 511, "Network Authentication Required");

}
