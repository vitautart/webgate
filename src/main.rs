use std::net::{TcpListener, TcpStream};
use std::io::*;
use std::fs;

// IMPORTANT: to kill server you need to press Ctrl-C not Ctrl-Z
fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_conn(stream);
    }
}

fn handle_conn(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    
    let body = fs::read_to_string("index.html").unwrap();
    let body_len = body.len();

    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContext-Length: {body_len}\r\n\r\n{body}");
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Connection established");
}
