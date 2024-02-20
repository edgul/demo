use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::collections::VecDeque;
use simple_db;

fn handle_client(mut stream: TcpStream, data: &mut VecDeque<String>) -> std::io::Result<()> {
    let mut s = "".to_string();
    let result = stream.read_to_string(&mut s);
    if result.is_err() {
        println!("Error reading to string");
    }

    let parsed = simple_db::parse(&s);
    let mut response = "".to_string();
    match parsed {
        Ok(simple_db::Command::Publish(s)) => {
            data.push_back(s);
            response = "OK\n".to_string();
        },
        Ok(simple_db::Command::Retrieve) => {
            if let Some(r) = data.pop_front() {
                response = format!("{r}\n");
            }
            else {
                response = "Nothing has been published\n".to_string();
            }
        },
        _ => {
            response = format!("{:?}\n", parsed);
        }
    }
    stream.write_all(response.as_bytes())?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("binding to socket");
    let ip_port = "127.0.0.1:7878";
    let listener = TcpListener::bind(ip_port)?;

    let mut deque = VecDeque::new();

    for stream in listener.incoming() {
        println!("incoming stream {:?}", stream);
        match stream {
            Ok(stream) => {
                handle_client(stream, &mut deque)?; 
            },
            Err(error) => {
                println!("Error: {error}");
            }
        }
    }
    Ok(())
}
