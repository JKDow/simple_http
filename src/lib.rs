pub(crate) mod router;
pub(crate) mod server;
pub(crate) mod route_utils;

pub use server::WebServer;

pub mod routing {
    pub use crate::router::Router;
    pub use crate::route_utils::*;
}

pub mod hyper {
    pub use hyper::{Method, Request, Response};
}

pub mod utils;
