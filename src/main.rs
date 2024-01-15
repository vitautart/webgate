use std::net::{TcpListener, TcpStream};
use std::io::*;
use std::fs;

// IMPORTANT: to kill server you need to press Ctrl-C not Ctrl-Z
fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();
    let mut pool = Vec::<std::thread::JoinHandle<()>>::new();
    for stream in listener.incoming()
    {
        std::thread::sleep(std::time::Duration::from_millis(500));
        if pool.len() > 3
        {
            if let Some(e) = pool.pop() 
            {
                e.join().unwrap();
            }
        }
        pool.push(std::thread::spawn(||
        {
            let stream = stream.unwrap();
            handle_conn(stream);
        }));
    }
}

fn handle_conn(mut stream: TcpStream)
{
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request = std::str::from_utf8(&buffer).unwrap();
    println!("REQUEST: {request}");
    
    let response = if buffer.starts_with(b"POST /add HTTP/1.1\r\n") 
    {
        let body = String::from("<p>Entry</p>");
        let body_len = body.len();
        format!("HTTP/1.1 201 CTEATED\r\nCache-Control: no-cache\r\nContent-Type: text/html;charset=utf-8\r\nContent-Length: {body_len}\r\n\r\n{body}")
    }
    else if buffer.starts_with(b"GET / HTTP/1.1\r\n") 
    {
        let body = fs::read_to_string("index.html").unwrap();
        let body_len = body.len();
        format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {body_len}\r\n\r\n{body}")
    } 
    else 
    {
        let body = fs::read_to_string("404.html").unwrap();
        let body_len = body.len();
        format!("HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\nContent-Length: {body_len}\r\n\r\n{body}")
    };
    println!("RESPONSE: {response}");
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    //println!("Connection established");
}
