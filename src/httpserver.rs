// This feature will enable you to spin up your own http server using the command line
// This implimentation is inspired by Ramesh Vyas medium article on
// Building a simple HTTP server in Rust
// go give it a read: https://medium.com/@rameshovyas/a-step-by-step-guide-to-build-a-custom-http-server-of-own-in-rust-7308cead63a2

use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;


pub fn create_server() {
    // Local host Tcplistener
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "8477";


    let end_point: String = format!("{}:{}", HOST, PORT);

    // Creating a TCP listener at our endpoint
    let listener = TcpListener::bind(end_point).unwrap();

    println!("Webserver is listening at port {}", PORT);

    // connecting to incoming connections
    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        // Process incoming connections
        handle_connections(_stream);

        println!("Connection established 🥳");
    }
}


pub fn handle_connections(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    // this is temporary and only here for testing
    let response_contents = fs::read_to_string("index.html").unwrap_or_else(|e| {
        eprintln!("Error reading index.html: {}", e);
        let fallback = "<h1>404 - index.html not found</h1>".to_string();
        fallback
    });

    // response
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        response_contents.len(),
        response_contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}