use crate::rails::ext::fut::ext::MultiStateX;
use core::future::Future;
use core::mem;
use core::pin::Pin;
use core::task::{Context, Poll};

pub struct Merge<Fut, T1, FutFunc, FutOut> {
    state: MultiStateX<Fut, Option<T1>, FutFunc, FutOut>,
}

impl<Fut, FutFunc, FutOut, U, T, T1> Merge<Fut, T1, FutFunc, FutOut>
where
    Fut: Future<Output = Option<T>>,
    FutFunc: FnOnce(T, T1) -> FutOut,
    FutOut: Future<Output = Option<U>>,
{
    pub fn new(future: Fut, func: FutFunc, t1: Option<T1>) -> Self {
        Self {
            state: MultiStateX::Waiting {
                future,
                func,
                input: t1,
            },
        }
    }
}

impl<Fut, FutFunc, FutOut, U, T, T1> Future for Merge<Fut, T1, FutFunc, FutOut>
where
    Fut: Future<Output = Option<T>>,
    FutFunc: FnOnce(T, T1) -> FutOut,
    FutOut: Future<Output = Option<U>>,
{
    type Output = Option<U>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        match this.state {
            MultiStateX::Waiting { ref mut future, .. } => {
                match unsafe { Pin::new_unchecked(future) }.poll(cx) {
                    Poll::Ready(opt_value) => match opt_value {
                        Some(t) => {
                            let (func, in_t1) =
                                match mem::replace(&mut this.state, MultiStateX::Done) {
                                    MultiStateX::Waiting { func, input, .. } => (func, input),
                                    _ => unreachable!(),
                                };
                            match in_t1 {
                                Some(t1) => {
                                    let new_future = func(t, t1);
                                    this.state = MultiStateX::Processing { future: new_future };
                                    cx.waker().wake_by_ref();
                                    Poll::Pending
                                }
                                _ => Poll::Ready(None),
                            }
                        }
                        None => {
                            this.state = MultiStateX::Done;
                            Poll::Ready(None)
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

pub struct Merge2<Fut, T1, T2, FutFunc, FutOut> {
    state: MultiStateX<Fut, (Option<T1>, Option<T2>), FutFunc, FutOut>,
}

impl<Fut, T1, T2, FutFunc, FutOut, U, T> Merge2<Fut, T1, T2, FutFunc, FutOut>
where
    Fut: Future<Output = Option<T>>,
    FutFunc: FnOnce(T, T1, T2) -> FutOut,
    FutOut: Future<Output = Option<U>>,
{
    pub fn new(future: Fut, func: FutFunc, in_t1: Option<T1>, in_t2: Option<T2>) -> Self {
        Self {
            state: MultiStateX::Waiting {
                future,
                func,
                input: (in_t1, in_t2),
            },
        }
    }
}

impl<Fut, T1, T2, FutFunc, FutOut, U, T> Future for Merge2<Fut, T1, T2, FutFunc, FutOut>
where
    Fut: Future<Output = Option<T>>,
    FutFunc: FnOnce(T, T1, T2) -> FutOut,
    FutOut: Future<Output = Option<U>>,
{
    type Output = Option<U>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        match this.state {
            MultiStateX::Waiting { ref mut future, .. } => {
                match unsafe { Pin::new_unchecked(future) }.poll(cx) {
                    Poll::Ready(opt_value) => match opt_value {
                        Some(t) => {
                            let (func, (in_t1, in_t2)) =
                                match mem::replace(&mut this.state, MultiStateX::Done) {
                                    MultiStateX::Waiting { func, input, .. } => (func, input),
                                    _ => unreachable!(),
                                };
                            match (in_t1, in_t2) {
                                (Some(t1), Some(t2)) => {
                                    let new_future = func(t, t1, t2);
                                    this.state = MultiStateX::Processing { future: new_future };
                                    cx.waker().wake_by_ref();
                                    Poll::Pending
                                }
                                _ => Poll::Ready(None),
                            }
                        }
                        None => {
                            this.state = MultiStateX::Done;
                            Poll::Ready(None)
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

pub struct Merge3<Fut, T1, T2, T3, FutFunc, FutOut> {
    state: MultiStateX<Fut, (Option<T1>, Option<T2>, Option<T3>), FutFunc, FutOut>,
}

impl<Fut, T1, T2, T3, FutFunc, FutOut, U, T> Merge3<Fut, T1, T2, T3, FutFunc, FutOut>
where
    Fut: Future<Output = Option<T>>,
    FutFunc: FnOnce(T, T1, T2, T3) -> FutOut,
    FutOut: Future<Output = Option<U>>,
{
    pub fn new(
        future: Fut,
        func: FutFunc,
        in_t1: Option<T1>,
        in_t2: Option<T2>,
        in_t3: Option<T3>,
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

impl<Fut, T1, T2, T3, FutFunc, FutOut, U, T> Future for Merge3<Fut, T1, T2, T3, FutFunc, FutOut>
where
    Fut: Future<Output = Option<T>>,
    FutFunc: FnOnce(T, T1, T2, T3) -> FutOut,
    FutOut: Future<Output = Option<U>>,
{
    type Output = Option<U>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        match this.state {
            MultiStateX::Waiting { ref mut future, .. } => {
                // Poll the initial future
                match unsafe { Pin::new_unchecked(future) }.poll(cx) {
                    Poll::Ready(opt_value) => match opt_value {
                        Some(t) => {
                            // Move the function out of the state
                            let (func, (in_t1, in_t2, in_t3)) =
                                match mem::replace(&mut this.state, MultiStateX::Done) {
                                    MultiStateX::Waiting { func, input, .. } => (func, input),
                                    _ => unreachable!(),
                                };
                            // Create the next future
                            match (in_t1, in_t2, in_t3) {
                                (Some(t1), Some(t2), Some(t3)) => {
                                    let new_future = func(t, t1, t2, t3);
                                    this.state = MultiStateX::Processing { future: new_future };
                                    cx.waker().wake_by_ref();
                                    Poll::Pending
                                }
                                _ => Poll::Ready(None),
                            }
                        }
                        None => {
                            // Transition to Done state
                            this.state = MultiStateX::Done;
                            Poll::Ready(None)
                        }
                    },
                    Poll::Pending => Poll::Pending,
                }
            }
            MultiStateX::Processing { ref mut future } => {
                // Poll the future returned by the function
                unsafe { Pin::new_unchecked(future) }.poll(cx).map(|t| t)
            }
            MultiStateX::Done => panic!("Future polled after completion"),
        }
    }
}

pub struct Merge4<Fut, T1, T2, T3, T4, FutFunc, FutOut> {
    state: MultiStateX<Fut, (Option<T1>, Option<T2>, Option<T3>, Option<T4>), FutFunc, FutOut>,
}

impl<Fut, T1, T2, T3, T4, FutFunc, FutOut, U, T> Merge4<Fut, T1, T2, T3, T4, FutFunc, FutOut>
where
    Fut: Future<Output = Option<T>>,
    FutFunc: FnOnce(T, T1, T2, T3, T4) -> FutOut,
    FutOut: Future<Output = Option<U>>,
{
    pub fn new(
        future: Fut,
        func: FutFunc,
        in_t1: Option<T1>,
        in_t2: Option<T2>,
        in_t3: Option<T3>,
        in_t4: Option<T4>,
    ) -> Self {
        Self {
            state: MultiStateX::Waiting {
                future,
                func,
                input: (in_t1, in_t2, in_t3, in_t4),
            },
        }
    }
}

impl<Fut, T1, T2, T3, T4, FutFunc, FutOut, U, T> Future
    for Merge4<Fut, T1, T2, T3, T4, FutFunc, FutOut>
where
    Fut: Future<Output = Option<T>>,
    FutFunc: FnOnce(T, T1, T2, T3, T4) -> FutOut,
    FutOut: Future<Output = Option<U>>,
{
    type Output = Option<U>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        match this.state {
            MultiStateX::Waiting { ref mut future, .. } => {
                match unsafe { Pin::new_unchecked(future) }.poll(cx) {
                    Poll::Ready(opt_value) => match opt_value {
                        Some(t) => {
                            let (func, (in_t1, in_t2, in_t3, in_t4)) =
                                match mem::replace(&mut this.state, MultiStateX::Done) {
                                    MultiStateX::Waiting { func, input, .. } => (func, input),
                                    _ => unreachable!(),
                                };
                            match (in_t1, in_t2, in_t3, in_t4) {
                                (Some(t1), Some(t2), Some(t3), Some(t4)) => {
                                    let new_future = func(t, t1, t2, t3, t4);
                                    this.state = MultiStateX::Processing { future: new_future };
                                    cx.waker().wake_by_ref();
                                    Poll::Pending
                                }
                                _ => Poll::Ready(None),
                            }
                        }
                        None => {
                            this.state = MultiStateX::Done;
                            Poll::Ready(None)
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
