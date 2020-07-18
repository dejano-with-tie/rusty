use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use crv::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    
    let pool = ThreadPool::new(10);
    for stream in listener.incoming() {
        let connection = stream.unwrap();
        println!(
            "Got new connection, {:?} <-> {:?}",
            connection.local_addr().unwrap(),
            connection.peer_addr().unwrap()
        );

        pool.execute(|| {
            handle_connection(connection);
        });
    }
}

fn handle_connection(mut conn: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];

    let request = conn.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", request);

    let contents = fs::read_to_string("index.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    conn.write(response.as_bytes()).unwrap();
    conn.flush().unwrap();

    //    println!("Request body: {}", request);
    //    buffer = [0; 512];
}
