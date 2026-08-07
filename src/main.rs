use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
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
    
    let (status_line, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 222 CUSTOM STATUS", "html/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "html/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND OOPS", "html/404.html"),
    };

    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();

    // HTTP format is just a text file with key value pairs except status and content
    let response = format!("{status_line}\nContent-length: {length}\n\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}
