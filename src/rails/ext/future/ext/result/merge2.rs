use crate::rails::ext::future::ext::MultiStateX;
use core::future::Future;
use core::mem;
use core::pin::Pin;
use core::task::{Context, Poll};

pub struct Merge2<Fut, T1, T2, E, FutFunc, FutOut> {
    state: MultiStateX<Fut, (Result<T1, E>, Result<T2, E>), FutFunc, FutOut>,
}

impl<Fut, T1, T2, FutFunc, FutOut, U, T, E> Merge2<Fut, T1, T2, E, FutFunc, FutOut>
where
    Fut: Future<Output = Result<T, E>>,
    FutFunc: FnOnce(T, T1, T2) -> FutOut,
    FutOut: Future<Output = Result<U, E>>,
{
    pub fn new(future: Fut, func: FutFunc, in_t1: Result<T1, E>, in_t2: Result<T2, E>) -> Self {
        Self {
            state: MultiStateX::Waiting {
                future,
                func,
                input: (in_t1, in_t2),
            },
        }
    }
}

impl<Fut, T1, T2, FutFunc, FutOut, U, T, E> Future for Merge2<Fut, T1, T2, E, FutFunc, FutOut>
where
    Fut: Future<Output = Result<T, E>>,
    FutFunc: FnOnce(T, T1, T2) -> FutOut,
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
                            let (func, (in_t1, in_t2)) =
                                match mem::replace(&mut this.state, MultiStateX::Done) {
                                    MultiStateX::Waiting { func, input, .. } => (func, input),
                                    _ => unreachable!(),
                                };
                            match (in_t1, in_t2) {
                                (Ok(t1), Ok(t2)) => {
                                    let new_future = func(t, t1, t2);
                                    this.state = MultiStateX::Processing { future: new_future };
                                    cx.waker().wake_by_ref();
                                    Poll::Pending
                                }
                                (Err(e), _) => Poll::Ready(Err(e)),
                                (_, Err(e)) => Poll::Ready(Err(e)),
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
