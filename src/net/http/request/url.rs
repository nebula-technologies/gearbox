use crate::net::http::request::error::Error;
use crate::rails::ext::blocking::IntoOptional;
use alloc::string::{String, ToString};
use core::fmt;
use core::fmt::Display;
use core::str::FromStr;
use serde_derive::{Deserialize, Serialize};
use url::Host;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Url {
    Simple(String),
    Strict(UrlStrict),
}

impl Url {}
impl Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Url::Simple(url) => write!(f, "{}", url),
            Url::Strict(url) => write!(f, "{}", url.to_string()),
        }
    }
}

impl UrlStrict {
    fn from_str(url: &str) -> Result<UrlStrict, Error> {
        reqwest::Url::from_str(url)
            .map(|url| url.into())
            .map_err(|e| Error::UrlParser(e))
    }
}

impl From<Url> for url::Url {
    fn from(url: Url) -> Self {
        (&url).into()
    }
}

impl From<&Url> for url::Url {
    fn from(url: &Url) -> Self {
        match url {
            Url::Simple(url) => reqwest::Url::from_str(&url).unwrap(),
            Url::Strict(url) => reqwest::Url::from_str(&url.to_string()).unwrap(),
        }
    }
}

impl From<url::Url> for Url {
    fn from(url: url::Url) -> Self {
        Url::from(&url)
    }
}

impl From<&url::Url> for Url {
    fn from(url: &url::Url) -> Self {
        Url::Strict(UrlStrict::from(url))
    }
}

impl From<&str> for Url {
    fn from(url: &str) -> Self {
        if let Ok(url) = UrlStrict::from_str(url) {
            Url::Strict(url)
        } else {
            Url::Simple(url.to_string())
        }
    }
}
impl From<String> for Url {
    fn from(url: String) -> Self {
        Url::from(url.as_str())
    }
}
impl From<&String> for Url {
    fn from(url: &String) -> Self {
        Url::from(url.as_str())
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct UrlStrict {
    #[serde(skip_serializing_if = "Option::is_none")]
    serialized: Option<String>,
    #[allow(unused)]
    scheme: String,
    #[allow(unused)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "self::host_serde")]
    host: Option<Host>,
    #[allow(unused)]
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<u16>,
    #[allow(unused)]
    path: String,
    #[allow(unused)]
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<String>,
    #[allow(unused)]
    #[serde(skip_serializing_if = "Option::is_none")]
    fragment: Option<String>,
    #[allow(unused)]
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    #[allow(unused)]
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<String>,
}

impl UrlStrict {
    /// Return the scheme of this URL, lower-cased, as an ASCII string without the ':' delimiter.
    ///
    /// # Examples
    ///
    /// ```
    /// use url::Url;
    /// # use url::ParseError;
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = Url::parse("file:///tmp/foo")?;
    /// assert_eq!(url.scheme(), "file");
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    #[inline]
    pub fn scheme(&self) -> &str {
        self.scheme.as_str()
    }

    /// Return the username for this URL (typically the empty string)
    /// as a percent-encoded ASCII string.
    ///
    /// # Examples
    ///
    /// ```
    /// use url::Url;
    /// # use url::ParseError;
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = Url::parse("ftp://rms@example.com")?;
    /// assert_eq!(url.username(), "rms");
    ///
    /// let url = Url::parse("ftp://:secret123@example.com")?;
    /// assert_eq!(url.username(), "");
    ///
    /// let url = Url::parse("https://example.com")?;
    /// assert_eq!(url.username(), "");
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn username(&self) -> Option<&str> {
        self.username.as_ref().map(|t| t.as_str())
    }

    /// Return the password for this URL, if any, as a percent-encoded ASCII string.
    ///
    /// # Examples
    ///
    /// ```
    /// use url::Url;
    /// # use url::ParseError;
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = Url::parse("ftp://rms:secret123@example.com")?;
    /// assert_eq!(url.password(), Some("secret123"));
    ///
    /// let url = Url::parse("ftp://:secret123@example.com")?;
    /// assert_eq!(url.password(), Some("secret123"));
    ///
    /// let url = Url::parse("ftp://rms@example.com")?;
    /// assert_eq!(url.password(), None);
    ///
    /// let url = Url::parse("https://example.com")?;
    /// assert_eq!(url.password(), None);
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn password(&self) -> Option<&str> {
        self.password.as_ref().map(|password| password.as_str())
    }

    /// Return the parsed representation of the host for this URL.
    /// Non-ASCII domain labels are punycode-encoded per IDNA if this is the host
    /// of a special URL, or percent encoded for non-special URLs.
    ///
    /// Cannot-be-a-base URLs (typical of `data:` and `mailto:`) and some `file:` URLs
    /// don’t have a host.
    ///
    /// See also the `host_str` method.
    ///
    /// # Examples
    ///
    /// ```
    /// use url::Url;
    /// # use url::ParseError;
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = Url::parse("https://127.0.0.1/index.html")?;
    /// assert!(url.host().is_some());
    ///
    /// let url = Url::parse("ftp://rms@example.com")?;
    /// assert!(url.host().is_some());
    ///
    /// let url = Url::parse("unix:/run/foo.socket")?;
    /// assert!(url.host().is_none());
    ///
    /// let url = Url::parse("data:text/plain,Stuff")?;
    /// assert!(url.host().is_none());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn host(&self) -> Option<String> {
        match &self.host {
            Some(Host::Domain(d)) => Some(d.to_string()),
            Some(Host::Ipv4(address)) => Some(address.to_string()),
            Some(Host::Ipv6(address)) => Some(address.to_string()),
            _ => None,
        }
    }

    /// Return the port number for this URL, if any.
    ///
    /// Note that default port numbers are never reflected by the serialization,
    /// use the `port_or_known_default()` method if you want a default port number returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use url::Url;
    /// # use url::ParseError;
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = Url::parse("https://example.com")?;
    /// assert_eq!(url.port(), None);
    ///
    /// let url = Url::parse("https://example.com:443/")?;
    /// assert_eq!(url.port(), None);
    ///
    /// let url = Url::parse("ssh://example.com:22")?;
    /// assert_eq!(url.port(), Some(22));
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    #[inline]
    pub fn port(&self) -> Option<u16> {
        self.port
            .or_else(|| super::utils::default_port(&self.to_string()))
    }

    /// Return the path for this URL, as a percent-encoded ASCII string.
    /// For cannot-be-a-base URLs, this is an arbitrary string that doesn’t start with '/'.
    /// For other URLs, this starts with a '/' slash
    /// and continues with slash-separated path segments.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use url::{Url, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = Url::parse("https://example.com/api/versions?page=2")?;
    /// assert_eq!(url.path(), "/api/versions");
    ///
    /// let url = Url::parse("https://example.com")?;
    /// assert_eq!(url.path(), "/");
    ///
    /// let url = Url::parse("https://example.com/countries/việt nam")?;
    /// assert_eq!(url.path(), "/countries/vi%E1%BB%87t%20nam");
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn path(&self) -> &str {
        self.path.as_ref()
    }

    /// Return this URL’s query string, if any, as a percent-encoded ASCII string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use url::Url;
    /// # use url::ParseError;
    ///
    /// fn run() -> Result<(), ParseError> {
    /// let url = Url::parse("https://example.com/products?page=2")?;
    /// let query = url.query();
    /// assert_eq!(query, Some("page=2"));
    ///
    /// let url = Url::parse("https://example.com/products")?;
    /// let query = url.query();
    /// assert!(query.is_none());
    ///
    /// let url = Url::parse("https://example.com/?country=español")?;
    /// let query = url.query();
    /// assert_eq!(query, Some("country=espa%C3%B1ol"));
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn query(&self) -> Option<&str> {
        self.query.as_ref().map(|s| s.as_str())
    }

    /// Return this URL’s fragment identifier, if any.
    ///
    /// A fragment is the part of the URL after the `#` symbol.
    /// The fragment is optional and, if present, contains a fragment identifier
    /// that identifies a secondary resource, such as a section heading
    /// of a document.
    ///
    /// In HTML, the fragment identifier is usually the id attribute of a an element
    /// that is scrolled to on load. Browsers typically will not send the fragment portion
    /// of a URL to the server.
    ///
    /// **Note:** the parser did *not* percent-encode this component,
    /// but the input may have been percent-encoded already.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use url::Url;
    /// # use url::ParseError;
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = Url::parse("https://example.com/data.csv#row=4")?;
    ///
    /// assert_eq!(url.fragment(), Some("row=4"));
    ///
    /// let url = Url::parse("https://example.com/data.csv#cell=4,1-6,2")?;
    ///
    /// assert_eq!(url.fragment(), Some("cell=4,1-6,2"));
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn fragment(&self) -> Option<&str> {
        self.fragment.as_ref().map(|s| s.as_str())
    }
}

impl fmt::Debug for UrlStrict {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .debug_struct("Url")
            .field("scheme", &self.scheme())
            .field("username", &self.username())
            .field("password", &self.password())
            .field("host", &self.host())
            .field("port", &self.port())
            .field("path", &self.path())
            .field("query", &self.query())
            .field("fragment", &self.fragment())
            .finish()
    }
}

impl ToString for UrlStrict {
    fn to_string(&self) -> String {
        if let Some(serialized) = &self.serialized {
            serialized.clone()
        } else {
            let mut url = String::new();
            url.push_str(self.scheme());
            url.push_str("://");
            if let Some(username) = self.username() {
                url.push_str(username);
                if let Some(password) = self.password() {
                    url.push(':');
                    url.push_str(password);
                }
                url.push('@');
            }
            if let Some(host) = self.host() {
                url.push_str(&host);
            }
            if let Some(port) = self.port() {
                url.push(':');
                url.push_str(&port.to_string());
            }
            url.push_str(self.path());
            if let Some(query) = self.query() {
                url.push('?');
                url.push_str(query);
            }
            if let Some(fragment) = self.fragment() {
                url.push('#');
                url.push_str(fragment);
            }
            url
        }
    }
}

impl From<url::Url> for UrlStrict {
    fn from(url: url::Url) -> Self {
        let scheme = url.scheme().to_string();
        let host = url.host().map(|t| t.to_owned());
        let port = url.port();
        let path = url.path().to_string();
        let query = url.query().map(String::from);
        let fragment = url.fragment().map(String::from);
        let username = url.username().to_string().into_opt();
        let password = url.password().map(String::from);
        let serialized = Some(url.to_string());

        UrlStrict {
            serialized,
            scheme,
            host,
            port,
            path,
            query,
            fragment,
            username,
            password,
        }
    }
}

impl From<&url::Url> for UrlStrict {
    fn from(url: &url::Url) -> Self {
        let scheme = url.scheme().to_string();
        let host = url.host().map(|t| t.to_owned());
        let port = url.port();
        let path = url.path().to_string();
        let query = url.query().map(String::from);
        let fragment = url.fragment().map(String::from);
        let username = Option::from(url.username().to_string());
        let password = url.password().map(String::from);
        let serialized = Some(url.to_string());

        UrlStrict {
            serialized,
            scheme,
            host,
            port,
            path,
            query,
            fragment,
            username,
            password,
        }
    }
}

mod host_serde {
    use super::*;
    use crate_serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(host: &Option<Host>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match host {
            Some(h) => serializer.serialize_some(&h.to_string()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Host>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<String> = Option::deserialize(deserializer)?;
        if let Some(host) = opt {
            Host::parse(&host)
                .map_err(serde::de::Error::custom)
                .map(Some)
        } else {
            Ok(None)
        }
    }
}
