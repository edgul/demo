use std::collections::VecDeque;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use std::sync::{Arc, Mutex};
use std::thread;

fn handle_client(mut stream: TcpStream, data: Arc<Mutex<VecDeque<String>>>) -> std::io::Result<()> {
    let mut s = "".to_string();
    stream.read_to_string(&mut s)?;

    let parsed = simple_db::parse(&s);
    let response;
    match parsed {
        Ok(simple_db::Command::Publish(s)) => {
            data.lock().unwrap().push_back(s);
            response = "OK\n".to_string();
        }
        Ok(simple_db::Command::Retrieve) => {
            if let Some(r) = data.lock().unwrap().pop_front() {
                response = format!("{r}\n");
            } else {
                response = "Nothing has been published\n".to_string();
            }
        }
        _ => {
            response = format!("{:?}\n", parsed);
        }
    }
    stream.write_all(response.as_bytes())?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Listening");

    let deque = Arc::new(Mutex::new(VecDeque::new()));

    thread::scope(|s| {
        for stream in listener.incoming() {
            println!("incoming stream {:?}", stream);
            match stream {
                Ok(stream) => {
                    let deque_arc = deque.clone();
                    s.spawn(move || {
                        handle_client(stream, deque_arc).unwrap();
                    });
                }
                Err(error) => {
                    println!("Error: {error}");
                }
            }
        }
    });
    Ok(())
}
