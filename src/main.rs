use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

use http_rust::{request::Request, response::Response, thread_pool::ThreadPool};

// fn build_manaul_response_200(content: &str) -> String {
//     let status_line = "HTTP/1.1 200 OK";
//     let contents = content;
//     let length = contents.len();
//     let response = format!(
//         "{status_line}\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{contents}"
//     );

//     return response;
// }

fn handle_connection(mut stream: TcpStream) {
    let req = Request::new(&stream);
    let res = match req {
        Ok(req) => {
            let res = match req.path.as_str() {
                "/hello" => {
                    let default_name = "World".to_string();
                    let name: &String = req.query.get("name").unwrap_or_else(|| &default_name);
                    let payload = format!("{{\"message\": \"Hello, {}!\"}}", name);
                    Response::json(200, &payload, None)
                }
                &_ => {
                    let payload = "{{\"message\": \"Invalid Path\"}}";
                    Response::json(400, &payload, None)
                }
            };
            Response::resolve(&res)
        }
        Err(s) => {
            let payload = format!("{{\"message\": \"{}\"}}", s);
            let res = Response::json(500, &payload, None);
            Response::resolve(&res)
        }
    };
    match stream.write(res.as_bytes()) {
        Ok(_) => {}
        Err(_) => {
            println!("FAILED DISPATCHED RESPONSE")
        }
    }
}
// fn handle_connection(stream: &mut TcpStream) {
//     let read_buffer = BufReader::new(stream);

//     for line in read_buffer.lines() {
//         let line = line.unwrap();

//         if line.is_empty() {
//             break;
//             //end of the headers
//         }

//         println!("Recived: {}", line);
//     }
// }

fn main() {
    let addr: &str = "127.0.0.1";
    let port: u16 = 5000;
    // let contents = "{'hello': 'World'}";

    //TCP listerner and .unwrap() any errors maybe not do it
    let listener = TcpListener::bind((addr, port)).expect("failed to bind TCP listener");

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
