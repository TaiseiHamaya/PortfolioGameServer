mod game;
mod entity;
mod contents;
mod log_init;

mod proto;

#[tokio::main]
async fn main() {
    log_init::init();

    log::info!("Server starting...\n");

    log::info!("Build info");
    log::info!("Package version: {}, OS: {}, Arch: {}\n", env!("CARGO_PKG_VERSION"), std::env::consts::OS, std::env::consts::ARCH);

    game::framework::run().await;
}
