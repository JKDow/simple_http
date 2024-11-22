use hyper::{Method, Request, Response};
use crate::{callback::{CallbackFuture, CallbackResult, RouteHandler}, route::Route, utils::empty_body};

pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: vec![] }
    }

    pub fn add_route(&mut self, method: Method, path: &str, callback: Box<dyn RouteHandler>) {
        self.routes.push(Route::new(method, path, callback));
    }

    pub fn add_basic_route<F>(&mut self, method: Method, path: &str, callback: F)
    where
        F: Fn(Request<hyper::body::Incoming>) -> CallbackFuture + Send + Sync + 'static,
    {
        self.routes.push(Route::with_basic_handler(method, path, callback));
    }

    #[tracing::instrument(skip_all)]
    pub async fn route(
        &self,
        req: Request<hyper::body::Incoming>,
    ) -> CallbackResult {
        let method = req.method().clone();
        let path = req.uri().path().to_string();
        tracing::debug!("Request: {} {}", method, path);
        match self.routes.iter().find(|r| r.matches(&method, &path)) {
            Some(route) => route.callback(req).await,
            None => {
                let mut not_found = Response::new(empty_body());
                *not_found.status_mut() = hyper::StatusCode::NOT_FOUND;
                Ok(not_found)
            }
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}
