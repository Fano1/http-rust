pub struct Response {
    status_text: String,
    headers: Vec<(String, String)>,
    body: String,
}

impl Response {
    pub fn json(status: u16, body: &str, headers: Option<Vec<(String, String)>>) -> Self {
        // conversion to response code
        let content_length = body.len();
        let pre_dermined_headers = vec![
            ("Content-Type".to_string(), "application/json".to_string()),
            ("Content-Length".to_string(), content_length.to_string()),
        ];
        let headers = headers.unwrap_or_else(|| vec![]);

        let status_text = match status {
            200 => "200 OK".to_string(),
            400 => "400 Bad Request".to_string(),
            500 => "500 Internal Server Error".to_string(),
            _ => format!("{} Unknown", status),
        };
        Self {
            status_text,
            headers: [pre_dermined_headers, headers].concat(),
            body: body.to_string(),
        }
    }

    pub fn resolve(response: &Response) -> String {
        // response to string code
        let mut response_str = format!("HTTP/1.1 {}\r\n", response.status_text);
        for (key, value) in &response.headers {
            response_str.push_str(&format!("{}: {}\r\n", key, value));
        }
        response_str.push_str("\r\n");
        response_str.push_str(&response.body);
        response_str
    }
}
