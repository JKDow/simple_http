use std::{future::Future, pin::Pin, sync::Arc};

use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{body::Incoming, service::Service, Method, Request, Response};

use crate::{
    routing::{CallbackFuture, CallbackInput, CallbackResult, Route, RouteHandler},
    utils::empty_body,
};

#[derive(Clone)]
pub struct Router {
    routes: Arc<[Route]>,
}

impl Router {
    fn new(routes: Arc<[Route]>) -> Self {
        Self { routes }
    }

    pub fn builder() -> RouterBuilder {
        RouterBuilder::new()
    }

    #[tracing::instrument(skip_all)]
    pub async fn route(&self, req: Request<hyper::body::Incoming>) -> CallbackResult {
        let method = req.method().clone();
        let path = req.uri().path().to_string();
        tracing::debug!("Request: {} {}", method, path);
        match self
            .routes
            .iter()
            .find(|r| r.matches(&method, &path).is_some())
        {
            Some(route) => route.callback(req).await,
            None => {
                let mut not_found = Response::new(empty_body());
                *not_found.status_mut() = hyper::StatusCode::NOT_FOUND;
                Ok(not_found)
            }
        }
    }
}

impl Service<Request<Incoming>> for Router {
    type Response = Response<BoxBody<Bytes, hyper::Error>>;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
    fn call(&self, req: CallbackInput) -> Self::Future {
        let router = self.clone();
        Box::pin(async move { router.route(req).await })
    }
}

pub struct RouterBuilder {
    routes: Vec<Route>,
}

impl RouterBuilder {
    fn new() -> Self {
        Self { routes: vec![] }
    }

    pub fn add_route(&mut self, method: Method, path: &str, callback: Box<dyn RouteHandler>) {
        self.routes.push(Route::new(method, path, callback));
    }

    pub fn add_basic_route<F>(&mut self, method: Method, path: &str, callback: F)
    where
        F: Fn(Request<hyper::body::Incoming>) -> CallbackFuture + Send + Sync + 'static,
    {
        self.routes
            .push(Route::with_basic_handler(method, path, callback));
    }

    pub fn build(self) -> Router {
        let routes = self.routes.into_iter().collect::<Arc<[_]>>();
        Router::new(routes)
    }
}
