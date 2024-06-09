use std::{
    io::{Error, Read, Write},
    net::TcpListener,
};

fn main() -> Result<(), Error> {
    let connection_listner = TcpListener::bind("127.0.0.1:3000")?;
    println!("run server port 3000");
    for stream in connection_listner.incoming() {
        let mut stream = stream?;
        println!("Connection established");
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;
        stream.write(&mut buffer)?;
    }
    Ok(())
}
