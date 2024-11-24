use super::Connection;

pub enum ConnectionBuilder {
    Http1(http1::Builder),
    Http2(http2::Builder<TokioExecutor>),
    H2C(http2::Builder<TokioExecutor>),
}

impl ConnectionBuilder {
    pub fn serve_connection(
        &self,
        stream: TokioIo<TcpStream>,
        app: TowerToHyperService<Router>,
    ) -> Connection {
        match self {
            Self::Http1(t) => Connection::Http1(t.serve_connection(stream, app)),
            Self::Http2(t) => Connection::Http2(t.serve_connection(stream, app)),
            Self::H2C(t) => Connection::Http2(t.serve_connection(stream, app)),
        }
    }
}

impl From<http1::Builder> for ConnectionBuilder {
    fn from(t: http1::Builder) -> Self {
        Self::Http1(t)
    }
}

impl From<http2::Builder<TokioExecutor>> for ConnectionBuilder {
    fn from(t: http2::Builder<TokioExecutor>) -> Self {
        Self::Http2(t)
    }
}
