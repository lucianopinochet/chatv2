use std::net::TcpStream;
// use std::io::{self, Write, Read};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
// #[tokio::main]
fn main(){
  let mut client = TcpStream::connect("localhost:3000").expect("Error connecting to the port");
  
  println!("Messages: ");
  
  loop {
    
    let mut data = String::new();
    
    io::stdin().read_line(&mut data).expect("Couldn't read message");
    
    data.pop();
    
    client.write_all(data.as_bytes()).expect("Failed to send message");
    
    thread::sleep(Duration::from_millis(100));
    // let mut buffer = [0;64];
    
    // client.read(&mut buffer).expect("Couldn't read from server");
    
    // println!("Recivied from server: {}", String::from_utf8_lossy(&buffer));
    
  }  
}
