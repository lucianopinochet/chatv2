use std::env;
use std::process::exit;
use std::thread;
use std::time::Duration;
use tokio::io::{self, AsyncWriteExt, BufReader, AsyncReadExt};
use clap::Parser;
use tokio::net::TcpStream;

///Program to connect for chat
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  ///Name of the user (Obligatory)
  #[arg(short, long, default_value = "Luciano")]
  name: String,
}

#[tokio::main]
async fn main(){
  
    let args:Vec<String> = env::args().collect();
  
  let args = Args::parse();
  
  /// if args.len() != 2{
  
  /// ///exit(1);
  ///}

  let mut client = TcpStream::connect("localhost:3000").await.unwrap();
  println!("Messages: "); 
    tokio::spawn(async move {
      let (reader, mut writer) = client.split();

      writer.write_all(&args.name.as_bytes()).await.unwrap();

      let mut data = [0;64];

      let mut buffer = [0;64];

      let mut reader = BufReader::new(reader);

      let mut stdin = io::stdin();
      
      loop{
        tokio::select! {
          result = reader.read(&mut buffer) => {
              
            println!("{}", String::from_utf8_lossy(&buffer));
            buffer = [0;64];
            
            if let Err(_) = result{
              println!("Connection Broked");
              exit(0);
            };
            thread::sleep(Duration::from_millis(100));
          }
          _ = stdin.read(&mut data) => {

            writer.write_all(&data).await.expect("Error writing to stream");
            data = [0;64];
          }
        }
      }
    }).await.unwrap();
}
