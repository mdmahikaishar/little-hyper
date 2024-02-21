use std::collections::HashMap;

const HEADER_SEP: &str = ": ";
const HEADER_CONTENT_TYPE: &str = "Content-Type";
const HEADER_CONTENT_LEN: &str = "Content-Length";

#[derive(Debug, Default)]
pub struct Headers(HashMap<String, String>);

impl Headers {
    /// Get
    ///
    /// Get header value.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }

    /// Set
    ///
    /// Set header value.
    pub fn set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string());
    }

    /// Raw Header
    ///
    /// Used for writing response.
    pub fn raw_headers(&self) -> String {
        self.0
            .iter()
            .map(|(key, value)| format!("{}{}{}", key, HEADER_SEP, value))
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn content_type(&mut self) -> Option<&String> {
        self.get(HEADER_CONTENT_TYPE)
    }
    pub fn set_content_type(&mut self, content_type: &str) {
        self.set(HEADER_CONTENT_TYPE, content_type)
    }

    pub fn content_len(&mut self) -> Option<&String> {
        self.get(HEADER_CONTENT_LEN)
    }
    pub fn set_content_len(&mut self, content_len: &str) {
        self.set(HEADER_CONTENT_LEN, content_len)
    }
}

impl From<&[String]> for Headers {
    fn from(value: &[String]) -> Self {
        let mut headers = Headers::default();

        for item in value {
            let (key, value) = item.split_once(HEADER_SEP).expect("ERROR: ReadHeader");

            headers.set(key, value);
        }

        headers
    }
}
