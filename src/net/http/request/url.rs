use crate::net::http::request::error::Error;
use core::str::FromStr;
use url::Host;

pub enum Url {
    Simple(String),
    Strict(UrlStrict),
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
            Url::Strict(url) => reqwest::Url::from_str(&url.scheme).unwrap(),
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

pub struct UrlStrict {
    scheme: String,
    host: Option<Host>,
    port: Option<u16>,
    path: String,
    query: Option<String>,
    fragment: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

impl From<url::Url> for UrlStrict {
    fn from(url: url::Url) -> Self {
        let scheme = url.scheme().to_string();
        let host = url.host().map(|t| t.to_owned());
        let port = url.port();
        let path = url.path().to_string();
        let query = url.query().map(String::from);
        let fragment = url.fragment().map(String::from);
        let username = Option::from(url.username().to_string());
        let password = url.password().map(String::from);

        UrlStrict {
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

        UrlStrict {
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
