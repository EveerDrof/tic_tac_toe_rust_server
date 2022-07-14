use crate::server::server;

#[macro_use]
extern crate lazy_static;

mod server;
mod tests;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    server().await
}
