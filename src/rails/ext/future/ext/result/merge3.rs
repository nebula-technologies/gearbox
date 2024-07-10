use crate::rails::ext::future::ext::MultiStateX;
use core::future::Future;
use core::mem;
use core::pin::Pin;
use core::task::{Context, Poll};

pub struct Merge3<Fut, T1, T2, T3, E, FutFunc, FutOut> {
    state: MultiStateX<Fut, (Result<T1, E>, Result<T2, E>, Result<T3, E>), FutFunc, FutOut>,
}

impl<Fut, T1, T2, T3, FutFunc, FutOut, U, T, E> Merge3<Fut, T1, T2, T3, E, FutFunc, FutOut>
where
    Fut: Future<Output = Result<T, E>>,
    FutFunc: FnOnce(T, T1, T2, T3) -> FutOut,
    FutOut: Future<Output = Result<U, E>>,
{
    pub fn new(
        future: Fut,
        func: FutFunc,
        in_t1: Result<T1, E>,
        in_t2: Result<T2, E>,
        in_t3: Result<T3, E>,
    ) -> Self {
        Self {
            state: MultiStateX::Waiting {
                future,
                func,
                input: (in_t1, in_t2, in_t3),
            },
        }
    }
}

impl<Fut, T1, T2, T3, FutFunc, FutOut, U, T, E> Future
    for Merge3<Fut, T1, T2, T3, E, FutFunc, FutOut>
where
    Fut: Future<Output = Result<T, E>>,
    FutFunc: FnOnce(T, T1, T2, T3) -> FutOut,
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
                            let (func, (in_t1, in_t2, in_t3)) =
                                match mem::replace(&mut this.state, MultiStateX::Done) {
                                    MultiStateX::Waiting { func, input, .. } => (func, input),
                                    _ => unreachable!(),
                                };
                            match (in_t1, in_t2, in_t3) {
                                (Ok(t1), Ok(t2), Ok(t3)) => {
                                    let new_future = func(t, t1, t2, t3);
                                    this.state = MultiStateX::Processing { future: new_future };
                                    cx.waker().wake_by_ref();
                                    Poll::Pending
                                }
                                (Err(e), _, _) => Poll::Ready(Err(e)),
                                (_, Err(e), _) => Poll::Ready(Err(e)),
                                (_, _, Err(e)) => Poll::Ready(Err(e)),
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
