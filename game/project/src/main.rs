mod game;
mod entity;
mod containts;

mod proto;

#[tokio::main]
async fn main() {
    game::framework::run().await;
}
