
use std::{error::Error, future::Future, pin::Pin};

use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{Request, Response};

pub type CallbackInput = Request<hyper::body::Incoming>;
pub type CallbackResult = Result<Response<BoxBody<Bytes, hyper::Error>>, Box<dyn Error + Send + Sync>>;
pub type CallbackFuture = Pin<Box<dyn Future<Output = CallbackResult> + Send>>;

pub trait RouteHandler: Send + Sync {
    fn callback(&self, input: CallbackInput) -> CallbackFuture;
}
