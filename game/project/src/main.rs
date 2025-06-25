mod game;
mod entity;

#[tokio::main]
async fn main() {
    game::framework::run().await;
}
