mod game;
mod entity;
mod containts;

#[tokio::main]
async fn main() {
    game::framework::run().await;
}
