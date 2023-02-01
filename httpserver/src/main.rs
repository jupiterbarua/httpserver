mod handler;
mod server;
mod router;
use server::Server;

fn main() {
    //start server
    let server = Server::new("localhost:3000");
    //Run the server
    server.run();
}
