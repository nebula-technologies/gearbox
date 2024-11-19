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

impl<Fut, FutFunc, FuncOutput, T, U, E> AndThen<Fut, FutFunc, FuncOutput>
where
    Fut: Future<Output = Result<T, E>>,
    FutFunc: FnOnce(T) -> FuncOutput,
    FuncOutput: Future<Output = Result<U, E>>,
{
    pub fn new(future: Fut, func: FutFunc) -> Self {
        Self {
            state: State::Waiting { future, func },
        }
    }
}

impl<Fut, FutFunc, FuncOutput, T, U, E> Future for AndThen<Fut, FutFunc, FuncOutput>
where
    Fut: Future<Output = Result<T, E>>,
    FutFunc: FnOnce(T) -> FuncOutput,
    FuncOutput: Future<Output = Result<U, E>>,
{
    type Output = Result<U, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        match this.state {
            State::Waiting { ref mut future, .. } => {
                match unsafe { Pin::new_unchecked(future) }.poll(cx) {
                    Poll::Ready(res) => match res {
                        Ok(t) => {
                            let func = match mem::replace(&mut this.state, State::Done) {
                                State::Waiting { func, .. } => func,
                                _ => unreachable!(),
                            };
                            let new_future = func(t);
                            this.state = State::Processing { future: new_future };
                            cx.waker().wake_by_ref();
                            Poll::Pending
                        }
                        Err(e) => {
                            this.state = State::Done;
                            Poll::Ready(Err(e))
                        }
                    },
                    Poll::Pending => Poll::Pending,
                }
            }
            State::Processing { ref mut future } => {
                unsafe { Pin::new_unchecked(future) }.poll(cx).map(|t| t)
            }
            State::Done => panic!("Future polled after completion"),
        }
    }
}
