use std::{collections::HashMap, io::Read, net::TcpStream};

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub content: String,
}

impl Request {
    pub fn new(mut stream: &TcpStream) -> Result<Self, String> {
        const MESSAGE_SIZE: usize = 1024;
        let mut received: Vec<u8> = vec![]; // this will store full request
        let mut rx_bytes = [0u8; MESSAGE_SIZE]; // temporary buffer

        loop {
            let byte_read = stream.read(&mut rx_bytes);
            match byte_read {
                Ok(bytes) => {
                    received.extend_from_slice(&rx_bytes[..bytes]);
                    if bytes < MESSAGE_SIZE {
                        break;
                    }
                }
                Err(err) => {
                    return Err(err.to_string());
                    //we call this log and pray. We hope some one is watching console and smart enough to figure out what went wrong.
                }
            }

            // Store headers and query parameters
            //please dont do this in prod code , when calling .unwrap() we are basically expecting the incomming HTTP request to be perfetcly formatted
            let request_text = String::from_utf8_lossy(&received).into_owned();
            let mut request_lines: Vec<&str> = request_text.split_inclusive('\n').collect();
            let mut header_map: HashMap<String, String> = HashMap::new();
            let mut query_params: HashMap<String, String> = HashMap::new();

            // First line of HTTP request, e.g., "GET /path?x=1 HTTP/1.1"
            let request_line = request_lines[0];
            let mut parts = request_line.split_ascii_whitespace();

            // Extract HTTP method and full path
            let _http_method = parts.next().unwrap();
            let full_path = parts.next().unwrap();

            // Separate path and query string (e.g., "/search?q=rust")
            let path_and_query: Vec<&str> = full_path.split('?').collect();
            let _path = path_and_query[0];

            if path_and_query.len() > 1 {
                let query_string = path_and_query[1..].join("");
                let query_pairs: Vec<&str> = query_string.split("&").collect();
                for pair in query_pairs {
                    if let Some((key, value)) = pair.split_once('=') {
                        query_params.insert(key.to_string(), value.to_string());
                    }
                }
            }

            // Remove the request line (already processed)
            request_lines.remove(0);

            let blank_line_index = request_lines
                .iter()
                .position(|&line| line == "\r\n")
                .unwrap();

            let body_lines = &mut request_lines.split_off(blank_line_index);
            body_lines.remove(0); // Remove the blank line itself
            let _body_content = body_lines.join(""); // Reconstruct body as a single string

            //Finally the fkn header parsing
            for header_line in &request_lines {
                if header_line.trim().is_empty() {
                    continue;
                }
                if let Some((key, value)) = header_line.split_once(": ") {
                    header_map.insert(key.to_string(), value.trim().to_string());
                }
            }
        }

        Ok(Request {
            method: String::new(),
            path: String::new(),
            headers: HashMap::new(),
            query: HashMap::new(),
            content: String::from_utf8_lossy(&received).into_owned(),
        })
    }
}
