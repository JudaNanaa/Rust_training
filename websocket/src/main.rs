use std::io;

use tokio::{io::AsyncReadExt, net::{TcpListener, TcpSocket}};


fn set_server() -> Result<TcpListener, io::Error>
{
	let addr = "127.0.0.1:8080".parse().unwrap();
	let max_client = u32::max_value();
    let socket = TcpSocket::new_v4()?;
    socket.set_reuseaddr(true)?;
    socket.bind(addr)?;
    let listener = socket.listen(max_client)?;

	return Ok(listener);
}



#[tokio::main]
async fn main() {
	let listener = match set_server() {
		Ok(value) => value,
		Err(error) => {
			println!("Error: {}", error);
			return ;
		},
	};

	let mut str = String::new();

	loop {
		match listener.accept().await {
			Ok((mut client_stream, addr)) => {
				println!("new client: {:?}", addr);
				let mut buffer = [0; 1024];
				let nb_read = match client_stream.read(&mut buffer).await {
					Ok(n) => n,
					Err(error) => {
						println!("Error: {}", error);
						return;
					},
				};
				str = String::from_utf8_lossy(&buffer[..nb_read]).to_string();
			},
			Err(e) => println!("couldn't get client: {:?}", e),
		}
		println!("Str content == \n{str}");
	}
}
