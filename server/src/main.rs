use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream}, ops::Add,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buff_reader = BufReader::new(&stream);
    let req_line = buff_reader.lines().next().unwrap().unwrap();
    let mut response = String::from("");
    if req_line == "GET / HTTP/1.1" {

        let status_line = "HTTP/1.1 200 OK";
        let content = fs::read_to_string("src/templates/index.html").unwrap();
        response = response.add(&format!("{status_line}\r\n\r\n{content}"));
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        response = response.add(&format!("{status_line}"));    
    }
    stream
    .write_all(
        response.as_bytes(),
    )
    .unwrap();
}
