use std::env;
use std::process::exit;
use std::thread;
use std::time::Duration;
use tokio::io::{self, AsyncWriteExt, BufReader, AsyncReadExt, AsyncBufReadExt};

use tokio::net::TcpStream;

#[tokio::main]
async fn main(){
  let args:Vec<String> = env::args().collect();
  if args.len() != 2{
    exit(1);
  }

  let mut client = TcpStream::connect("localhost:3000").await.unwrap();
  println!("Messages: "); 
    tokio::spawn(async move {
      let (reader, mut writer) = client.split();

      writer.write_all(&args[1].as_bytes()).await.unwrap();

      let mut data = [0;64];

      let mut buffer = String::new();

      let mut reader = BufReader::new(reader);

      let mut stdin = io::stdin();
      
      loop{
        tokio::select! {
          _ = stdin.read(&mut data) => {

            writer.write_all(&data).await.expect("Error writing to stream");
          }
          result = reader.read_line(&mut buffer) => {
							
            if let Err(_) = result{
              println!("Connection Broked");
              exit(1);
            }
            println!("{}", buffer);

            buffer.clear();

            thread::sleep(Duration::from_millis(100));
          }
        }
      }
    }).await.unwrap();
}