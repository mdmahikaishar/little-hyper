use crate::headers::Headers;
use crate::path::PathData;
use crate::path::path_regex::Params;
use crate::query::QueryData;

#[derive(Debug)]
pub struct Request {
    /// Method
    ///
    pub method: String,
    /// Path
    ///
    pub path: String,
    /// Pathname
    ///
    pub pathname: String,
    /// Params
    ///
    /// TODO: delete this one, after PathExtractor ready.
    pub params: Params,
    /// Querystring
    ///
    pub querystring: String,
    /// Query
    ///
    pub query: QueryData,
    /// Http Version
    ///
    pub version: String,
    /// Headers
    ///
    pub headers: Headers,
    /// Body
    ///
    pub body: String,
}

impl From<Vec<String>> for Request {
    fn from(value: Vec<String>) -> Self {
        let heading_info = value[0].split(' ').collect::<Vec<&str>>();

        let PathData {
            path,
            pathname,
            querystring,
            query,
        } = PathData::new(heading_info[1]);

        let body = "".to_string();

        Request {
            method: heading_info[0].to_string(),
            version: heading_info[2].to_string(),
            path,
            pathname,
            params: Params::new(),
            query,
            querystring,
            headers: Headers::from(&value[1..]),
            body,
        }
    }
}
