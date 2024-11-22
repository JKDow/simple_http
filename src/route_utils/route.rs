use hyper::Method;

use super::{BasicHandler, CallbackFuture, CallbackInput, CallbackResult, RouteHandler};

pub struct Route {
    method: Method,
    path: String,
    handler: Box<dyn RouteHandler>,
}

impl Route {
    pub fn new(method: Method, path: &str, handler: Box<dyn RouteHandler>) -> Self {
        Self {
            method,
            path: path.to_string(),
            handler,
        }
    }

    pub fn with_basic_handler<F>(method: Method, path: &str, callback: F) -> Self
    where
        F: Fn(CallbackInput) -> CallbackFuture + Send + Sync + 'static,
    {
        Self::new(method, path, Box::new(BasicHandler::new(callback)))
    }

    pub fn matches(&self, method: &Method, path: &str) -> bool {
        self.method == *method && self.path == path
    }

    pub async fn callback(&self, input: CallbackInput) -> CallbackResult {
        self.handler.callback(input).await
    }
}
