use crate::rails::ext::future::ext::MultiState;
use core::future::Future;
use core::mem;
use core::pin::Pin;
use core::task::{Context, Poll};

pub struct UnwrapOrElse<Fut, FutFunc, FuncOutput> {
    state: MultiState<Fut, FutFunc, FuncOutput>,
}

impl<Fut, FutFunc, FuncOutput, T, U> UnwrapOrElse<Fut, FutFunc, FuncOutput>
where
    Fut: Future<Output = Option<T>>,
    FutFunc: FnOnce() -> FuncOutput,
    FuncOutput: Future<Output = U>,
{
    pub fn new(future: Fut, func: FutFunc) -> Self {
        Self {
            state: MultiState::Waiting { future, func },
        }
    }
}

impl<Fut, FutFunc, FuncOutput, T> Future for UnwrapOrElse<Fut, FutFunc, FuncOutput>
where
    Fut: Future<Output = Option<T>>,
    FutFunc: FnOnce() -> FuncOutput,
    FuncOutput: Future<Output = T>,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        match this.state {
            MultiState::Waiting { ref mut future, .. } => {
                // Poll the initial future
                match unsafe { Pin::new_unchecked(future) }.poll(cx) {
                    Poll::Ready(opt_value) => match opt_value {
                        Some(t) => {
                            // Transition to Done state
                            this.state = MultiState::Done;
                            Poll::Ready(t)
                        }
                        None => {
                            // Move the function out of the state
                            let func = match mem::replace(&mut this.state, MultiState::Done) {
                                MultiState::Waiting { func, .. } => func,
                                _ => unreachable!(),
                            };
                            // Create the next future
                            let new_future = func();
                            // Transition to Processing state
                            this.state = MultiState::Processing { future: new_future };
                            cx.waker().wake_by_ref();
                            Poll::Pending
                        }
                    },
                    Poll::Pending => Poll::Pending,
                }
            }
            MultiState::Processing { ref mut future } => {
                // Poll the future returned by the function
                unsafe { Pin::new_unchecked(future) }.poll(cx).map(|t| t)
            }
            MultiState::Done => panic!("Future polled after completion"),
        }
    }
}
