use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

pub struct FutureContainer<O>(Option<O>);

impl<T> FutureContainer<Option<T>> {
    pub fn new_from_opt(value: Option<T>) -> Self {
        Self(Some(value))
    }
}

impl<T, E> FutureContainer<Result<T, E>> {
    pub fn new_from_res(value: Result<T, E>) -> Self {
        Self(Some(value))
    }
}

impl<T> Future for FutureContainer<Option<T>> {
    type Output = Option<T>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        Poll::Ready(this.0.take().flatten())
    }
}

impl<T, E> Future for FutureContainer<Result<T, E>> {
    type Output = Result<T, E>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        Poll::Ready(this.0.take().unwrap())
    }
}
