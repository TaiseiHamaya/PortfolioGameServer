mod game;
mod entity;
mod contents;
mod log_init;

mod proto;

#[tokio::main]
async fn main() {
    log_init::init();
    
    game::framework::run().await;
}
