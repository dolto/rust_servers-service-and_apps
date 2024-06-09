use std::{
    io::{Error, Read, Write},
    net::TcpStream,
};

fn main() -> Result<(), Error> {
    let mut stream = TcpStream::connect("localhost:3000")?;
    let message = "Hello Server!";
    println!("to server: {}", message);
    stream.write(message.as_bytes())?;
    let mut buffer = [0; 20];
    stream.read(&mut buffer)?;
    println!(
        "from server: {}",
        String::from_utf8(buffer.to_vec()).unwrap_or_default()
    );
    Ok(())
}
