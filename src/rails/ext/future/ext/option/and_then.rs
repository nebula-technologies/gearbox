use core::future::Future;
use core::mem;
use core::pin::Pin;
use core::task::{Context, Poll};

pub struct AndThen<Fut, FutFunc, FuncOutput> {
    state: State<Fut, FutFunc, FuncOutput>,
}

enum State<Fut, FutFunc, FuncOutput> {
    Waiting { future: Fut, func: FutFunc },
    Processing { future: FuncOutput },
    Done,
}

impl<Fut, FutFunc, FuncOutput, T, U> AndThen<Fut, FutFunc, FuncOutput>
where
    Fut: Future<Output = Option<T>>,
    FutFunc: FnOnce(T) -> FuncOutput,
    FuncOutput: Future<Output = Option<U>>,
{
    pub fn new(future: Fut, func: FutFunc) -> Self {
        Self {
            state: State::Waiting { future, func },
        }
    }
}

impl<Fut, FutFunc, FuncOutput, T, U> Future for AndThen<Fut, FutFunc, FuncOutput>
where
    Fut: Future<Output = Option<T>>,
    FutFunc: FnOnce(T) -> FuncOutput,
    FuncOutput: Future<Output = Option<U>>,
{
    type Output = Option<U>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = unsafe { self.get_unchecked_mut() };
        match this.state {
            State::Waiting { ref mut future, .. } => {
                // Poll the initial future
                match unsafe { Pin::new_unchecked(future) }.poll(cx) {
                    Poll::Ready(opt_value) => {
                        match opt_value {
                            Some(t) => {
                                // Move the function out of the state
                                let func = match mem::replace(&mut this.state, State::Done) {
                                    State::Waiting { func, .. } => func,
                                    _ => unreachable!(),
                                };
                                // Create the next future
                                let new_future = func(t);
                                // Transition to Processing state
                                this.state = State::Processing { future: new_future };
                                cx.waker().wake_by_ref();
                                Poll::Pending
                            }
                            None => {
                                // Transition to Done state
                                this.state = State::Done;
                                Poll::Ready(None)
                            }
                        }
                    }
                    Poll::Pending => Poll::Pending,
                }
            }
            State::Processing { ref mut future } => {
                // Poll the future returned by the function
                unsafe { Pin::new_unchecked(future) }.poll(cx).map(|t| t)
            }
            State::Done => panic!("Future polled after completion"),
        }
    }
}
