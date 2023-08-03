use std::process::exit;

use tokio::{
	net::TcpListener, 
	sync::broadcast,
	io::{self, BufReader, AsyncBufReadExt, AsyncWriteExt, AsyncReadExt},
	};
#[tokio::main]
async fn main(){
  let listener = TcpListener::bind("localhost:3000").await.unwrap();

	let (tx, _rx) = broadcast::channel(10);
	
	let mut cli = [0;64];

	loop{
		let (mut stream, socket_addr) = listener.accept().await.unwrap();
	
		let tx = tx.clone();

		let mut rx = tx.subscribe();

		
		tokio::spawn(async move {
			let mut name = [0;16];
	
			stream.read(&mut name).await.unwrap();
	
			let name = String::from_utf8_lossy(&name).to_string();

			println!("{name} Connected.");
			

			let (reader, mut writer) = stream.split();
			// writer.write_all("hola".as_bytes()).await.unwrap();
			let mut reader = BufReader::new(reader);
			
			let mut line = String::new();


      let mut stdin = io::stdin();

			loop {
					tokio::select! {
						result = reader.read_line(&mut line) => {
							
							if let Err(_) = result{
								println!("Connection lost with {name}");
								break;
							}
							let mut line = format!("{}: {}", &name, &line);
							
							tx.send((line.clone(), socket_addr)).unwrap();

							line.clear();
						}
						result = rx.recv() => {

							let (msg, other_addr) = result.unwrap();
							
							let msg = msg.trim_end_matches('\n');

							if other_addr == socket_addr {
							} 
							if other_addr != socket_addr {
								let msg = msg.as_bytes();
								writer.write_all(msg).await.unwrap();
							}
						}
						_ = stdin.read(&mut cli) => {
							
							let msg = cli.into_iter().take_while(|&x| x != 0).collect::<Vec<u8>>();
							
							cli = [0;64];

							let msg = String::from_utf8_lossy(&msg).to_string();

							if msg.len() == 2{
								exit(0);
							}

						}
					}
			}
		});
	}
}