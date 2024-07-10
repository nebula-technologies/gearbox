use crate::rails::ext::future::ext::MultiStateX;
use core::future::Future;
use core::mem;
use core::pin::Pin;
use core::task::{Context, Poll};

pub struct Merge<Fut, T1, E, FutFunc, FutOut> {
    state: MultiStateX<Fut, Result<T1, E>, FutFunc, FutOut>,
}

impl<Fut, FutFunc, FutOut, U, T, T1, E> Merge<Fut, T1, E, FutFunc, FutOut>
where
    Fut: Future<Output = Result<T, E>>,
    FutFunc: FnOnce(T, T1) -> FutOut,
    FutOut: Future<Output = Result<U, E>>,
{
    pub fn new(future: Fut, func: FutFunc, t1: Result<T1, E>) -> Self {
        Self {
            state: MultiStateX::Waiting {
                future,
                func,
                input: t1,
            },
        }
    }
}

impl<Fut, FutFunc, FutOut, U, T, T1, E> Future for Merge<Fut, T1, E, FutFunc, FutOut>
where
    Fut: Future<Output = Result<T, E>>,
    FutFunc: FnOnce(T, T1) -> FutOut,
    FutOut: Future<Output = Result<U, E>>,
{
    type Output = Result<U, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        match this.state {
            MultiStateX::Waiting { ref mut future, .. } => {
                match unsafe { Pin::new_unchecked(future) }.poll(cx) {
                    Poll::Ready(res) => match res {
                        Ok(t) => {
                            let (func, in_t1) =
                                match mem::replace(&mut this.state, MultiStateX::Done) {
                                    MultiStateX::Waiting { func, input, .. } => (func, input),
                                    _ => unreachable!(),
                                };
                            match in_t1 {
                                Ok(t1) => {
                                    let new_future = func(t, t1);
                                    this.state = MultiStateX::Processing { future: new_future };
                                    cx.waker().wake_by_ref();
                                    Poll::Pending
                                }
                                Err(e) => Poll::Ready(Err(e)),
                            }
                        }
                        Err(e) => {
                            this.state = MultiStateX::Done;
                            Poll::Ready(Err(e))
                        }
                    },
                    Poll::Pending => Poll::Pending,
                }
            }
            MultiStateX::Processing { ref mut future } => {
                unsafe { Pin::new_unchecked(future) }.poll(cx).map(|t| t)
            }
            MultiStateX::Done => panic!("Future polled after completion"),
        }
    }
}
