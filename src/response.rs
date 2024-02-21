use crate::Headers;
use std::io::prelude::*;
use std::net::TcpStream;

/// Response
///
///
#[derive(Debug)]
pub struct Response {
    stream: TcpStream,
    pub headers: Headers,
    contents: String,
    status_code: usize,
}

impl Response {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            headers: Headers::default(),
            contents: String::new(),
            status_code: 200,
        }
    }

    // /// Get Headers
    // ///
    // pub fn get_headers(&self) -> &Headers {
    //     &self.headers
    // }
    // /// Get Header
    // ///
    // /// Get header value.
    // pub fn get_header(&self, key: &str) -> Option<&String> {
    //     self.headers.get(key)
    // }
    // /// Set Header
    // ///
    // /// Set header value.
    // pub fn set_header(&mut self, key: &str, value: &str) {
    //     self.headers.set(key, value);
    // }
    /// Set Content Type
    ///
    /// Set content type in header.
    pub fn set_content_type(&mut self, content_type: &str) {
        self.headers.set_content_type(content_type);
    }

    /// Status
    ///
    /// Get status code.
    pub fn status(&self) -> usize {
        self.status_code
    }

    /// Set Status
    ///
    /// Set status code.
    pub fn set_status(&mut self, status_code: usize) {
        self.status_code = status_code;
    }
}

impl Response {
    /// Raw Response
    ///
    /// That will send back to client.
    fn raw_response(&self) -> String {
        format!(
            "{version} {status_code} {status_phrase}\r\n{headers}\r\n\r\n{contents}",
            version = "HTTP/1.1",
            status_code = self.status_code,
            status_phrase = "OK",
            headers = self.headers.raw_headers(),
            contents = self.contents
        )
    }

    /// Write
    ///
    /// Write data to client and flush out.
    fn write(&mut self, contents: &str) {
        // Set content and content len.
        self.contents.push_str(contents);
        self.headers.set_content_len(&self.contents.len().to_string());

        // Set content type, If there in None.
        if self.headers.content_type().is_none() {
            self.headers.set_content_type("plain/text");
        }

        self.stream
            .write_all(self.raw_response().as_bytes())
            .expect("ERROR: WriteResponse.");
        self.stream.flush().expect("ERROR: FlushResponse.");
    }

    /// Send
    ///
    /// Send data to client.
    pub fn send(&mut self, data: &str) {
        self.write(data);
    }

    /// Json
    ///
    /// Send JSON file.
    pub fn json(&mut self, json: &str) {
        self.set_content_type("application/json");

        self.write(json);
    }

    /// Html
    ///
    /// Send HTML file.
    pub fn html(&mut self, html: &str) {
        self.set_content_type("text/html");

        self.write(html);
    }

    /// File
    ///
    /// Send File contents.
    ///
    /// - `Content-Type` file extension or text/html.
    pub fn file(&mut self, path: &str) {
        self.set_content_type("plain/text");

        let contents = std::fs::read_to_string(path).expect("ERROR: ReadFile.");

        self.write(&contents)
    }
}
