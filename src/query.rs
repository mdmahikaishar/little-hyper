use std::collections::HashMap;

/// Query
///
///
/// ```fn my_path(Query(path): Query<MyStruct>) { }```
// #[derive(Debug)]
// pub struct Query<T>(T);

// #[allow(unused)]
// impl<T> Extractor<T> for Query<T> {
//     fn inner(&self) -> &T {
//         &self.0
//     }
// }

pub const QUERY_START_WITH: &str = "?";
pub const QUERY_SEP: &str = "&";
pub const QUERY_VALUE_SEP: &str = "=";

#[derive(Default, Debug)]
pub struct QueryData(HashMap<String, String>);

impl QueryData {
    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }
    pub fn set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string());
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<&str> for QueryData {
    fn from(value: &str) -> Self {
        value.split(QUERY_SEP).filter(|item| !item.is_empty()).fold(
            QueryData::default(),
            |mut query_data, item| {
                let (key, value) = item.split_once(QUERY_VALUE_SEP).unwrap_or((&item, &""));

                query_data.set(key, value);

                query_data
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_query_data() {
        let query_one = QueryData::from("type=admin");

        assert!(query_one
            .get("type")
            .and_then(|i| Some(i == "admin"))
            .is_some());

        let query_two = QueryData::from("a=1&b=2");

        assert!(query_two.get("a").and_then(|i| Some(i == "1")).is_some());
        assert!(query_two.get("b").and_then(|i| Some(i == "2")).is_some());

        let query_three = QueryData::from("a&b&c=3");

        assert!(query_three.get("a").and_then(|i| Some(i == "")).is_some());
        assert!(query_three.get("b").and_then(|i| Some(i == "")).is_some());
        assert!(query_three.get("b").and_then(|i| Some(i == "2")).is_some());
    }
}
