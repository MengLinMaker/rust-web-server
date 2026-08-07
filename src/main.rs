use std::{
    fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream},
};

fn main() {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    tracing::info!("handle_connection - request_line: {request_line}");
    
    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 222 CUSTOM STATUS";
        let content = fs::read_to_string("html/hello.html").unwrap();
        let length = content.len();

        // HTTP format is just a text file with key value pairs except status and content
        let response = format!("{status_line}\nContent-length: {length}\n\n{content}");
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND OOPS";
        let content = fs::read_to_string("html/404.html").unwrap();
        let length = content.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{content}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    }
}
