use std::fs::read_to_string;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use http_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        // thread::spawn(|| {
        //     handle_connection(stream);
        // });
        pool.execute(|| {
            handle_connection(stream);
        })
    }
    
    println!("Shutting down");
}

/// HTTP-Version Status-Code Reason-Phrase CRLF
/// headers CRLF
/// message-body
///
/// # Example
/// HTTP/1.1 200 OK\r\n\r\n
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) =
        if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "index.html")
        } else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

    let contents = read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]))
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}