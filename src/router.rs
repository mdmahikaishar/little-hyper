use crate::{
    path::{self, path_regex},
    Request, Response,
};
use std::collections::HashMap;

pub type RouteHandler = dyn Fn(&mut Request, &mut Response) + 'static;

/// Router
///
///
#[derive(Default)]
pub struct Router {
    /// Routes
    ///
    /// All get method routes are stored here.
    routes: HashMap<String, Box<RouteHandler>>,
}

impl Router {
    /// Router
    ///
    /// New instance of router.
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    /// Get
    ///
    /// Add get route.
    pub fn get<CB>(&mut self, path: &str, callback: CB)
    where
        CB: Fn(&mut Request, &mut Response) + 'static,
    {
        self.route("GET", path, callback);
    }

    /// Route
    ///
    /// Add route to router
    pub fn route<CB>(&mut self, _method: &str, path: &str, callback: CB)
    where
        CB: Fn(&mut Request, &mut Response) + 'static,
    {
        let sanitized_path = path::sanitize_path(path);
        let regex_path = path_regex::path_to_regex(&sanitized_path);

        self.routes.insert(regex_path, Box::new(callback));
    }

    /// Merge Router
    ///
    /// 
    pub fn merge_router(&mut self, target_router: Router) {
        target_router.routes.into_iter().for_each(|(key, value)| {
            self.routes.insert(key, value);
        });
    }

    /// Get Handler
    ///
    /// - `pathname` must be got from request. Or, you will get a unknown handler.
    pub fn get_handler(&self, request: &mut Request) -> Option<&Box<RouteHandler>> {
        // 1st dynamic routes
        // 2nd normal routes
        let key = self.routes.keys().find(|regex_path| {
            if let Some(params) = path_regex::path_regex_matcher(regex_path, &request.pathname) {
                request.params = params;

                return true;
            }
            false
        });

        if let Some(key) = key {
            return self.routes.get(key);
        }

        None
    }
}
