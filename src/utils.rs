use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};

/// Create a response with a full body.
/// # Example
/// ```rust
/// let mut response = Response::new(full_body("Hello, World!"));
/// *rsp.status_mut() = hyper::StatusCode::OK;
/// Ok(response)
/// ```
pub fn full_body<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

/// # Example
/// ```rust
/// let mut not_found = Response::new(empty_body());
/// *not_found.status_mut() = hyper::StatusCode::NOT_FOUND;
/// Ok(not_found)
/// ```
pub fn empty_body() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}
