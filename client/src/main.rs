// use std::net::TcpStream;
// use std::net::TcpStream;
// use std::io::{self, Write, Read};
use std::thread;
use std::time::Duration;
use tokio::io::AsyncWriteExt;

use tokio::net::TcpStream;

#[tokio::main]
async fn main(){
  let client = TcpStream::connect("localhost:3000").await.unwrap();
  
  println!("Messages: ");
  
    let mut data = String::new();
    let mut buffer = [0;64];
      // tokio::select! {
    tokio::spawn(async {
      loop{
        std::io::stdin().read_line(&mut data).expect("Couldn't read message");
          
          client.write_all(data.as_bytes()).unwrap();
      }
    });
    tokio::spawn(async {
      loop {
        client.read(&mut buffer).expect("Couldn't read from server");
        
        println!("Recivied from server: {}", String::from_utf8_lossy(&buffer));
          
        thread::sleep(Duration::from_millis(100));
          
      }
    })
        
        
        
        
        
        
        

      // }  
    // let mut client_cloned_1 = client.try_clone().unwrap();
    // let mut client_cloned_2 = client.try_clone().unwrap();
    
    
  
}