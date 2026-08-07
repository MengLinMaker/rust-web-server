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
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let http_length = http_request.len();
    tracing::info!("handle_connection - http_length: {http_length}");
    tracing::trace!("handle_connection - http_request: {http_request:#?}");

    let status_line = "HTTP/1.1 222 CUSTOM STATUS";
    let content = fs::read_to_string("src/hello.html").unwrap();
    let length = content.len();
    // HTTP format is just a text file with key value pairs except status and content
    let response = format!("{status_line}\nContent-length: {length}\n\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}
