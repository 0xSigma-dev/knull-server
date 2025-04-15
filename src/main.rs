use std::{borrow::Cow, io::{Read, Write}, net::TcpListener};

#[derive(Debug)]
struct Request {
    method: String,
    path: String,
    version: String,
    user_agent: String,
    headers: Vec<(String, String)>,
    body: Option<String>
}

fn main() {
    match TcpListener::bind("127.0.0.1:8080") {
        Ok(listener) => {
            println!("Listener: {:?}", listener);
            println!("{:?}", listener.incoming());
            println!("Server listening on 127.0.0.1:8080");

            for stream in listener.incoming() {
                println!("stream: {:?}", stream);
                match stream {
                    Ok(mut stream) => {
                        println!("New connection established");
                        let mut buffer = [0; 1024];
                        println!("Buffer: {:?}", buffer);

                        if let Ok(bytes_read) = stream.read(&mut buffer) {
                            println!("Bytes_read: {}", bytes_read);
                            let request_data: Cow<'_, str> = String::from_utf8_lossy(&buffer[..bytes_read]);
                            println!("Buffer: {:?}", buffer);
                            println!("Received request:\n{}", request_data);
                            if let Some(request) = parse_request(&request_data) {
                                println!("Parsed request: {:?}", request);

                                let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";
                                stream.write_all(response.as_bytes()).expect("Failed to write response");
                            } else {
                                eprintln!("Failed to parse request");
                            }

                        }
                    },
                    Err(e) => eprintln!("Failed to establish connection: {}", e)
                }
            }

            

        },
        Err(e) => {
            eprintln!("Failed to bind: {}", e);

        }
    }
    
}


fn parse_request(request_data: &str) -> Option<Request> {
    let mut lines = request_data.lines();
    println!("Lines: {:?}", lines);

    let request_line = lines.next()?;
    println!("Request_line: {}", request_line);
    let mut parts = request_line.split_whitespace();
    println!("Parts: {:?}", parts);
    let method = parts.next()?.to_string();
    let path = parts.next()?.to_string();
    let version = parts.next()?.to_string();

    let mut headers = Vec::new();
    let mut user_agent = String::new();

    for line in request_data.lines() {
        if line.is_empty() {
            break;
        }

        if let Some((key, value)) = line.split_once(": ") {
            headers.push((key.to_string(), value.to_string()));

            if key.eq_ignore_ascii_case("User-Agent") {
                user_agent = value.to_string();
            }
        }
    }

    let body = request_data.lines().skip(headers.len() + 1).collect::<Vec<&str>>().join("\n");
    println!("body: {}", body);
    let body = if body.is_empty() { None } else {
        Some(body)
    };
    println!("body: {:?}", body);

    Some(
        Request { 
             method,
             path, 
             version,
             user_agent,
             headers,
             body
             }
        )
}
