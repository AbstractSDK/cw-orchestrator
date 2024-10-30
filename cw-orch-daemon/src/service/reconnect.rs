use crate::service::future::ResponseFuture;
use log::trace;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{BoxError, Service};

/// Reconnect to failed services.
pub struct Reconnect<M, Target>
where
    M: Service<Target>,
    M::Error: std::fmt::Debug,

    M: Sync,
    Target: Sync,
{
    mk_service: M,
    #[allow(clippy::type_complexity)]
    state: Arc<Mutex<State<M::Future, M::Response, M::Error>>>,
    target: Target,
}

impl<M, Target> Clone for Reconnect<M, Target>
where
    M: Service<Target>,
    M::Error: std::fmt::Debug,
    M: Clone,
    Target: Clone,
    M: Sync,
    Target: Sync,
{
    fn clone(&self) -> Self {
        Self {
            mk_service: self.mk_service.clone(),
            state: self.state.clone(),
            target: self.target.clone(),
        }
    }
}

#[derive(Debug)]
enum State<F, S, E: std::fmt::Debug> {
    Error(E),
    Idle,
    Connecting(F),
    Connected(S),
}

impl<M, Target> Reconnect<M, Target>
where
    M: Service<Target>,
    M::Error: std::fmt::Debug,
    M: Sync,
    Target: Sync,
{
    /// Lazily connect and reconnect to a [`Service`].
    pub fn new(mk_service: M, target: Target) -> Self {
        Reconnect {
            mk_service,
            state: Arc::new(Mutex::new(State::Idle)),
            target,
        }
    }

    /// Reconnect to a already connected [`Service`].
    pub fn with_connection(init_conn: M::Response, mk_service: M, target: Target) -> Self {
        Reconnect {
            mk_service,
            state: Arc::new(Mutex::new(State::Connected(init_conn))),
            target,
        }
    }
}

impl<M, Target, S, Request> Service<Request> for Reconnect<M, Target>
where
    M: Service<Target, Response = S> + Sync,
    M::Error: std::fmt::Debug,
    M::Future: Unpin,
    BoxError: From<M::Error> + From<S::Error>,

    S: Service<Request>,
    Target: Clone + Sync,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = ResponseFuture<S::Future, M::Error>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        loop {
            let mut state = self.state.lock().unwrap();
            match &mut *state {
                State::Idle | State::Error(_) => {
                    trace!("poll_ready; idle");
                    match self.mk_service.poll_ready(cx) {
                        Poll::Ready(r) => r?,
                        Poll::Pending => {
                            trace!("poll_ready; MakeService not ready");
                            return Poll::Pending;
                        }
                    }

                    let fut = self.mk_service.call(self.target.clone());
                    drop(state);
                    self.state = Arc::new(Mutex::new(State::Connecting(fut)));
                    continue;
                }
                State::Connecting(ref mut f) => {
                    trace!("poll_ready; connecting");
                    match Pin::new(f).poll(cx) {
                        Poll::Ready(Ok(service)) => {
                            drop(state);
                            self.state = Arc::new(Mutex::new(State::Connected(service)));
                        }
                        Poll::Pending => {
                            trace!("poll_ready; not ready");
                            return Poll::Pending;
                        }
                        Poll::Ready(Err(e)) => {
                            trace!("poll_ready; error, retrying in {} seconds", 5);
                            drop(state);
                            self.state = Arc::new(Mutex::new(State::Error(e)));
                            sleep(Duration::from_secs(5));
                        }
                    }
                }
                State::Connected(ref mut inner) => {
                    trace!("poll_ready; connected");
                    match inner.poll_ready(cx) {
                        Poll::Ready(Ok(())) => {
                            trace!("poll_ready; ready");
                            return Poll::Ready(Ok(()));
                        }
                        Poll::Pending => {
                            trace!("poll_ready; not ready");
                            return Poll::Pending;
                        }
                        Poll::Ready(Err(_)) => {
                            trace!("poll_ready; error");

                            drop(state);
                            self.state = Arc::new(Mutex::new(State::Idle));
                        }
                    }
                }
            }
        }
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let mut state = self.state.lock().unwrap();
        let service = match &mut *state {
            State::Connected(ref mut service) => service,
            State::Error(error) => panic!(
                "service not ready; poll_ready must be called first: {:?}",
                error
            ),
            _ => panic!("service not ready; poll_ready must be called first"),
        };

        let fut = service.call(request);
        ResponseFuture::new(fut)
    }
}

impl<M, Target> fmt::Debug for Reconnect<M, Target>
where
    M: Service<Target> + fmt::Debug,
    M::Future: fmt::Debug,
    M::Response: fmt::Debug,
    Target: fmt::Debug,
    M::Error: std::fmt::Debug,
    M: Sync,
    Target: Sync,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Reconnect")
            .field("mk_service", &self.mk_service)
            .field("state", &self.state)
            .field("target", &self.target)
            .finish()
    }
}
