use hello::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() { // remove .take(2) done playing with pool
        // for stream in listener.incoming() {
            let stream = stream.unwrap();

        // Closures have the ability that functions don't have: they can 
        // capture their environment and access variables from the scope in 
        // which they're defined
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}", 
        status, 
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    // if buffer.starts_with(get) {
    //     let contents = fs::read_to_string("hello.html").unwrap();

    //     let response = format!(
    //         "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
    //         contents.len(),
    //         contents
    //     );

    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();

    // } else {
    //     let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    //     let contents = fs::read_to_string("404.html").unwrap();
    
    //     let response = format!("{}{}", status_line, contents);

    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // }
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

