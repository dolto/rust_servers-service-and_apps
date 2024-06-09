use std::io::Error;

use server::Server;

mod handler;
mod router;
mod server;
fn main() -> Result<(), Error> {
    let server = Server::new("localhost:3000");
    server.run()?;
    Ok(())
}
