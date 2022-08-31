use super::*;
use futures_util::ready;
use http::{Request, Response};
use pin_project_lite::pin_project;
use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{
    util:: Oneshot,
    ServiceExt,
};
use tower_service::Service;

opaque_future! {
    /// Response future for [`EmptyRouter`](super::EmptyRouter).
    pub type EmptyRouterFuture<E> =
        std::future::Ready<Result<Response<BoxBody>, E>>;
}

opaque_future! {
    /// Response future from [`MakeRouteService`] services.
    pub type MakeRouteServiceFuture<S> =
        std::future::Ready<Result<S, Infallible>>;
}

pin_project! {
    #[derive(Debug)]
    pub struct RouteFuture<S, F, B>
    where
        S: Service<Request<B>>,
        F: Service<Request<B>>,
    {
        #[pin]
        state: RouteFutureInner<S, F, B>,
    }
}

impl<S, F, B> RouteFuture<S, F, B>
where
    S: Service<Request<B>>,
    F: Service<Request<B>>,
{
    pub(crate) fn a(a: Oneshot<S, Request<B>>, fallback: F) -> Self {
        RouteFuture {
            state: RouteFutureInner::A {
                a,
                fallback: Some(fallback),
            },
        }
    }

    pub(crate) fn b(b: Oneshot<F, Request<B>>) -> Self {
        RouteFuture {
            state: RouteFutureInner::B { b },
        }
    }
}

pin_project! {
    #[project = RouteFutureInnerProj]
    #[derive(Debug)]
    enum RouteFutureInner<S, F, B>
    where
        S: Service<Request<B>>,
        F: Service<Request<B>>,
    {
        // 路由匹配的情况
        A {
            #[pin]
            a: Oneshot<S, Request<B>>,
            fallback: Option<F>
        },
        // 需要 fallback 的情况
        B {
            #[pin]
            b: Oneshot<F, Request<B>>
        },
    }
}

impl<S, F, B> Future for RouteFuture<S, F, B>
where
    S: Service<Request<B>, Response = Response<BoxBody>>,
    F: Service<Request<B>, Response = Response<BoxBody>, Error = S::Error>,
    B: Send + Sync + 'static,
{
    type Output = Result<Response<BoxBody>, S::Error>;

    #[allow(warnings)]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            let mut this = self.as_mut().project();

            let new_state = match this.state.as_mut().project() {
                RouteFutureInnerProj::A { a, fallback } => {
                    let mut response = ready!(a.poll(cx))?;

                    let req = if let Some(ext) =
                        response.extensions_mut().remove::<FromEmptyRouter<B>>()
                    {
                        ext.request
                    } else {
                        return Poll::Ready(Ok(response))
                    };

                    RouteFutureInner::B {
                        b: fallback
                            .take()
                            .expect("future polled after completion")
                            .oneshot(req),
                    }
                }
                RouteFutureInnerProj::B { b } => return b.poll(cx),
            };

            this.state.set(new_state);
        }
    }
}