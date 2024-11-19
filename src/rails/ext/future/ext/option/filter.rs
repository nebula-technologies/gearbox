use crate::rails::ext::future::ext::MultiState;
use core::future::Future;
use core::mem;
use core::pin::Pin;
use core::task::{Context, Poll};
pub struct Filter<Fut, FutFunc> {
    state: MultiState<Fut, FutFunc, Fut>,
}

impl<Fut, FutFunc, T> Filter<Fut, FutFunc>
where
    Fut: Future<Output = Option<T>>,
    FutFunc: FnOnce(&T) -> bool,
{
    pub fn new(future: Fut, func: FutFunc) -> Self {
        Self {
            state: MultiState::Waiting { future, func },
        }
    }
}

impl<Fut, FutFunc, T> Future for Filter<Fut, FutFunc>
where
    Fut: Future<Output = Option<T>>,
    FutFunc: FnOnce(&T) -> bool,
{
    type Output = Option<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };

        match this.state {
            MultiState::Waiting { ref mut future, .. } => {
                // Poll the initial future
                match unsafe { Pin::new_unchecked(future) }.poll(cx) {
                    Poll::Ready(opt_value) => {
                        let func = match mem::replace(&mut this.state, MultiState::Done) {
                            MultiState::Waiting { func, .. } => func,
                            _ => unreachable!(),
                        };
                        Poll::Ready(opt_value.filter(|t| func(t)))
                    }
                    Poll::Pending => Poll::Pending,
                }
            }
            MultiState::Processing { .. } => {
                unreachable!("Filter future polled after completion")
            }
            MultiState::Done => panic!("Future polled after completion"),
        }
    }
}
