use crate::rails::ext::future::ext::MultiState;
use core::future::Future;
use core::mem;
use core::pin::Pin;
use core::task::{Context, Poll};

pub struct MapOr<Fut, U, FutFunc, FuncOutput> {
    state: MultiState<Fut, (U, FutFunc), FuncOutput>,
}

impl<Fut, FutFunc, FuncOutput, T, U, E> MapOr<Fut, U, FutFunc, FuncOutput>
where
    Fut: Future<Output = Result<T, E>>,
    FutFunc: FnOnce(T) -> FuncOutput,
    FuncOutput: Future<Output = U>,
{
    pub fn new(future: Fut, default: U, func: FutFunc) -> Self {
        Self {
            state: MultiState::Waiting {
                future,
                func: (default, func),
            },
        }
    }
}

impl<Fut, FutFunc, FuncOutput, T, U, E> Future for MapOr<Fut, U, FutFunc, FuncOutput>
where
    Fut: Future<Output = Result<T, E>>,
    FutFunc: FnOnce(T) -> FuncOutput,
    FuncOutput: Future<Output = U>,
{
    type Output = U;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        match this.state {
            MultiState::Waiting { ref mut future, .. } => {
                match unsafe { Pin::new_unchecked(future) }.poll(cx) {
                    Poll::Ready(res) => match res {
                        Ok(t) => {
                            let (_, func) = match mem::replace(&mut this.state, MultiState::Done) {
                                MultiState::Waiting { func, .. } => func,
                                _ => unreachable!(),
                            };
                            let new_future = func(t);
                            this.state = MultiState::Processing { future: new_future };
                            cx.waker().wake_by_ref();
                            Poll::Pending
                        }
                        Err(_) => {
                            let (default, _) = match mem::replace(&mut this.state, MultiState::Done)
                            {
                                MultiState::Waiting { func, .. } => func,
                                _ => unreachable!(),
                            };
                            Poll::Ready(default)
                        }
                    },
                    Poll::Pending => Poll::Pending,
                }
            }
            MultiState::Processing { ref mut future } => {
                unsafe { Pin::new_unchecked(future) }.poll(cx)
            }
            MultiState::Done => panic!("Future polled after completion"),
        }
    }
}
