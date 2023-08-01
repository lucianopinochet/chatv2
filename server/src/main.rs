use tokio::{
	net::TcpListener, 
	sync::broadcast,
	io::{BufReader, AsyncBufReadExt, AsyncWriteExt}
	};

#[tokio::main]
async fn main(){
  let listener = TcpListener::bind("localhost:3000").await.unwrap();

	let (tx, _rx) = broadcast::channel(5);

	loop{
		let (mut stream, socket_addr) = listener.accept().await.unwrap();

		let tx = tx.clone();

		let mut rx = tx.subscribe();

		tokio::spawn(async move {
			let (reader, mut writer) = stream.split();

			let mut reader = BufReader::new(reader);

			let mut line = String::new();

			loop {

					tokio::select! {

						result = reader.read_line(&mut line) =>{
							
							if result.unwrap() == 0{

								break;
							}
							
							tx.send((line.clone(), socket_addr)).unwrap();

							line.clear();
						}
						result = rx.recv() => {
							
							let (msg, _other_addr) = result.unwrap();

							println!("{msg}");

							// if other_addr != socket_addr {
							writer.write_all(msg.as_bytes()).await.unwrap();
							// }
						}
					}
			}
		});

	}
}
