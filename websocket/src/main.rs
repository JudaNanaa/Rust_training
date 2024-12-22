use tokio::{io::AsyncReadExt, net::TcpSocket};
use std::io;

const MAX_CLIENT: u32 = u32::max_value();

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "127.0.0.1:8080".parse().unwrap();

    let socket = TcpSocket::new_v4()?;
    socket.set_reuseaddr(true)?;
    socket.bind(addr)?;

    let listener = socket.listen(MAX_CLIENT)?;

	let mut str = String::new();

	loop {
		match listener.accept().await {
			Ok((mut client_stream, addr)) => {
				println!("new client: {:?}", addr);
				let mut buffer = [0; 1024];
				client_stream.read(&mut buffer).await?;
				str = String::from_utf8_lossy(buffer.as_slice()).to_string();
			},
			Err(e) => println!("couldn't get client: {:?}", e),
		}
		println!("Str content == \n{str}");
	}
	Ok(())
}
