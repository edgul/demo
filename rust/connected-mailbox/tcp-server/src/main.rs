use std::net::{TcpListener};
use std::io::prelude::*;
use std::collections::VecDeque;
use simple_db;

fn main() -> std::io::Result<()> {
    println!("binding to socket");
    let ip_port = "127.0.0.1:7878";
    let listener = TcpListener::bind(ip_port)?;

    let mut deque = VecDeque::new();

    for stream in listener.incoming() {
        println!("incoming stream {:?}", stream);
        let mut stream = stream.unwrap();
        let mut s = "".to_string();
        let result = stream.read_to_string(&mut s);
        if result.is_err() {
            println!("Error reading to string");
        }

        let parsed = simple_db::parse(&s);
        let mut response = "".to_string();
        match parsed {
            Ok(simple_db::Command::Publish(s)) => {
                deque.push_back(s);
                response = "OK\n".to_string();
            },
            Ok(simple_db::Command::Retrieve) => {
                if let Some(r) = deque.pop_front() {
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
    }
    Ok(())
}
