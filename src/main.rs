use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

const ADDR: &str = "127.0.0.1:8080";

fn main() {
    let listener = TcpListener::bind(ADDR).unwrap();
    listener.incoming().for_each(|i| {
        let stream = i.unwrap();
        println!("Connection Enstablished!");
        handle_connection(stream);
    })
}

fn handle_connection(mut stream: TcpStream) {
    let buffer: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let request: Vec<String> = buffer
        .lines()
        .map(Result::unwrap)
        .take_while(|p| !p.is_empty())
        .collect();
    request.iter().for_each(|i| println!("{}", i));

    let (status, file) = match request.get(0).unwrap() == "GET / HTTP/1.1" {
        true => ("HTTP/1.1 200 OK", "index.html"),
        false => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    stream.write_all(status.as_bytes()).unwrap();

    println!("");
}
