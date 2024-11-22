use std::collections::HashMap;

use hyper::Method;
use regex::Regex;

use super::{BasicHandler, CallbackFuture, CallbackInput, CallbackResult, RouteHandler};

pub struct Route {
    method: Method,
    path_regex: Regex,
    params_names: Vec<String>,
    handler: Box<dyn RouteHandler>,
}

impl Route {
    pub fn new(method: Method, path: &str, handler: Box<dyn RouteHandler>) -> Self {
        let (reg, params) = Self::compile_path(path);
        Self {
            method,
            path_regex: reg,
            params_names: params,
            handler,
        }
    }

    pub fn with_basic_handler<F>(method: Method, path: &str, callback: F) -> Self
    where
        F: Fn(CallbackInput) -> CallbackFuture + Send + Sync + 'static,
    {
        Self::new(method, path, Box::new(BasicHandler::new(callback)))
    }

    pub fn matches(&self, method: &Method, path: &str) -> Option<HashMap<String, String>> {
        if self.method != *method {
            return None;
        }
        self.path_regex.captures(path).map(|caps| {
            self.params_names
                .iter()
                .enumerate()
                .filter_map(|(i, name)| {
                    caps.get(i + 1)
                        .map(|val| (name.clone(), val.as_str().to_string()))
                })
                .collect::<HashMap<String, String>>()
        })
    }

    pub async fn callback(&self, input: CallbackInput) -> CallbackResult {
        self.handler.callback(input).await
    }

    fn compile_path(path: &str) -> (Regex, Vec<String>) {
        let mut params_names = Vec::new();
        let regex_str = path
            .split('/')
            .map(|seg| {
                if seg.starts_with(":") {
                    params_names.push(seg[1..].to_string());
                    r"([^/]+)".to_string()
                } else {
                    regex::escape(seg)
                }
            })
            .collect::<Vec<_>>()
            .join("/");
        let regex_pattern = format!(r"^{}$", regex_str);
        let regex = Regex::new(&regex_pattern).unwrap();
        (regex, params_names)
    }
}
