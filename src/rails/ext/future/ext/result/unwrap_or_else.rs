use crate::rails::ext::future::ext::MultiState;
use core::future::Future;
use core::mem;
use core::pin::Pin;
use core::task::{Context, Poll};

pub struct UnwrapOrElse<Fut, FutFunc, FuncOutput> {
    state: MultiState<Fut, FutFunc, FuncOutput>,
}

impl<Fut, FutFunc, FuncOutput, T, E> UnwrapOrElse<Fut, FutFunc, FuncOutput>
where
    Fut: Future<Output = Result<T, E>>,
    FutFunc: FnOnce(E) -> FuncOutput,
    FuncOutput: Future<Output = T>,
{
    pub fn new(future: Fut, func: FutFunc) -> Self {
        Self {
            state: MultiState::Waiting { future, func },
        }
    }
}

impl<Fut, FutFunc, FuncOutput, T, E> Future for UnwrapOrElse<Fut, FutFunc, FuncOutput>
where
    Fut: Future<Output = Result<T, E>>,
    FutFunc: FnOnce(E) -> FuncOutput,
    FuncOutput: Future<Output = T>,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };

        match this.state {
            MultiState::Waiting { ref mut future, .. } => {
                match unsafe { Pin::new_unchecked(future) }.poll(cx) {
                    Poll::Ready(res) => match res {
                        Ok(t) => {
                            this.state = MultiState::Done;
                            Poll::Ready(t)
                        }
                        Err(e) => {
                            let func = match mem::replace(&mut this.state, MultiState::Done) {
                                MultiState::Waiting { func, .. } => func,
                                _ => unreachable!(),
                            };
                            let new_future = func(e);
                            this.state = MultiState::Processing { future: new_future };
                            cx.waker().wake_by_ref();
                            Poll::Pending
                        }
                    },
                    Poll::Pending => Poll::Pending,
                }
            }
            MultiState::Processing { ref mut future } => {
                unsafe { Pin::new_unchecked(future) }.poll(cx).map(|t| t)
            }
            MultiState::Done => panic!("Future polled after completion"),
        }
    }
}
