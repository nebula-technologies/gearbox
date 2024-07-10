use crate::rails::ext::future::ext::State;
use core::future::Future;
use core::mem;
use core::pin::Pin;
use core::task::{Context, Poll};

pub struct Or<Fut, T> {
    state: State<Fut, Option<T>>,
}

impl<Fut, T> Or<Fut, T>
where
    Fut: Future<Output = Option<T>>,
{
    pub fn new(future: Fut, func: Option<T>) -> Self {
        Self {
            state: State::Waiting { future, func },
        }
    }
}

impl<Fut, T> Future for Or<Fut, T>
where
    Fut: Future<Output = Option<T>>,
{
    type Output = Option<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };

        match this.state {
            State::Waiting { ref mut future, .. } => {
                // Poll the initial future
                match unsafe { Pin::new_unchecked(future) }.poll(cx) {
                    Poll::Ready(opt_value) => match opt_value {
                        Some(t) => {
                            // Transition to Done state
                            this.state = State::Done;
                            Poll::Ready(Some(t))
                        }
                        None => {
                            // Move the function out of the state
                            let func = match mem::replace(&mut this.state, State::Done) {
                                State::Waiting { func, .. } => func,
                                _ => unreachable!(),
                            };
                            // Create the next future
                            Poll::Ready(func)
                        }
                    },
                    Poll::Pending => Poll::Pending,
                }
            }
            State::Done => panic!("Future polled after completion"),
        }
    }
}
