use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_connection(mut stream: TcpStream, count: Arc<AtomicUsize>) {
    // Ordering.Relaxed ensures only atomicity
    let total = count.fetch_add(1, Ordering::Relaxed);
    println!("Client connected — total connections: {}", total + 1);

    let mut buf = [0; 1024];
    loop {
        match stream.read(&mut buf).await {
            Ok(0) => break,
            Ok(n) => {
                if stream.write_all(&buf[..n]).await.is_err() {
                    break;
                }
            }
            Err(_) => break,
        }
    }
    count.fetch_sub(1, Ordering::Relaxed);  // always runs

}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    // Arc: let task share ownership of the same value
    let count = Arc::new(AtomicUsize::new(0));
    println!("Listening on port 8080");

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("New connection: {}", addr);
                // give the same counter without copy
                tokio::spawn(handle_connection(stream, Arc::clone(&count)));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}
