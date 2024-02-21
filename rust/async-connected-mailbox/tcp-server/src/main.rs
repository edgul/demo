use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_client(
    mut stream: TcpStream,
    data: Arc<Mutex<VecDeque<String>>>,
) -> std::io::Result<()> {
    let mut s = String::new();
    stream.read_to_string(&mut s).await?;

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
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    println!("Listening");

    let deque = Arc::new(Mutex::new(VecDeque::new()));

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("new client: {:?}", addr);
                let deque_arc = deque.clone();
                tokio::spawn(async move {
                    handle_client(socket, deque_arc).await.unwrap();
                });
            }
            Err(e) => {
                println!("couldn't get client: {:?}", e);
                break;
            }
        }
    }
    Ok(())
}
