use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub enum Connection {
    Http1(http1::Connection<TokioIo<TcpStream>, TowerToHyperService<Router>>),
    Http2(http2::Connection<TokioIo<TcpStream>, TowerToHyperService<Router>, TokioExecutor>),
    H2C(http2::Connection<TokioIo<TcpStream>, TowerToHyperService<Router>, TokioExecutor>),
}

impl Future for Connection {
    type Output = Result<(), hyper::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.get_mut() {
            Connection::Http1(ref mut fut) => Pin::new(fut).poll(cx),
            Connection::Http2(ref mut fut) => Pin::new(fut).poll(cx),
            Connection::H2C(ref mut fut) => Pin::new(fut).poll(cx),
        }
    }
}
