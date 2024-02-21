pub mod path_regex;
use crate::query::{QueryData, QUERY_START_WITH};

/// Path
///
///
/// ```fn my_path(Path(path): Path<MyStruct>) { }```
// #[derive(Debug)]
// pub struct Path<T>(T);

// impl<T> Extractor<T> for Path<T> {
//     fn inner(&self) -> &T {
//         &self.0
//     }
// }

#[derive(Debug)]
pub struct PathData {
    pub path: String,
    pub pathname: String,
    pub querystring: String,
    pub query: QueryData,
}

impl PathData {
    pub fn new(path: &str) -> Self {
        let path = sanitize_path(path);

        let (pathname, querystring) = path.split_once(QUERY_START_WITH).unwrap_or((&path, &""));

        Self {
            path: path.to_string(),
            pathname: pathname.to_string(),
            querystring: querystring.to_string(),
            query: QueryData::from(querystring),
        }
    }
}

// remove noise of path
pub fn sanitize_path(path: &str) -> String {
    let path = path.replace('\\', "/");
    let path = path.replace('/', "/");
    let mut path = path.replace("/?", "?");

    if path.len() > 1 && path.ends_with('/') {
        path.pop();
    }

    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_path() {
        assert_eq!(sanitize_path("/abc/xyz/"), "/abc/xyz");
        assert_eq!(sanitize_path("/abc/1/"), "/abc/1");
        assert_eq!(sanitize_path("/abc/?a=1"), "/abc?a=1");
    }

    #[test]
    fn test_path_data() {
        let path_one = PathData::new("/users/?userId=10");

        assert!(path_one.path == "/users?userId=10");
        assert!(path_one.pathname == "/users");
        assert!(path_one.querystring == "userId=10");
        assert!(path_one.query.len() == 1);
    }
}
