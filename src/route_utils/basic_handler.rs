use super::{CallbackFuture, CallbackInput, RouteHandler};

pub struct BasicHandler {
    callback: Box<dyn Fn(CallbackInput) -> CallbackFuture + Send + Sync>,
}

impl BasicHandler {
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn(CallbackInput) -> CallbackFuture + Send + Sync + 'static,
    {
        BasicHandler {
            callback: Box::new(callback),
        }
    }
}

impl RouteHandler for BasicHandler {
    fn callback(&self, input: CallbackInput) -> CallbackFuture {
        (self.callback)(input)
    }
}

