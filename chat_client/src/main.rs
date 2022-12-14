use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use std::error::Error;
use std::io;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // Open a TCP stream to the socket address.
    //
    // Note that this is the Tokio TcpStream, which is fully async.
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    loop{

        println!("created stream");

        let result = stream.write_all(b"hello world\n").await;
        println!("wrote to stream; success={:?}", result.is_ok());

        stream.readable().await?;

        let mut buf = [0; 4096];

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                println!("read {} bytes, buffer : {}", n, String::from_utf8_lossy(&buf));
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}