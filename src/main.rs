use crate::server::server;

#[macro_use]
extern crate lazy_static;

use actix_web::{middleware::Logger, App};
use env_logger::{init_from_env, Env};
mod server;
mod tests;
use env_logger::Builder;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let env = Env::new().filter("MY_LOG").write_style("MY_LOG_STYLE");
    env_logger::init_from_env(Env::new().default_filter_or("info"));
    server().await
}
