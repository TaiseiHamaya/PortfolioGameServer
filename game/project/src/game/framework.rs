use std::net::Ipv4Addr;
use tokio::net::TcpListener;

use super::zone::Zone;

pub async fn run() {
    // 初期化
    let tcp_listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, 3215)).await;
    if tcp_listener.is_err() {
        eprintln!("Error binding TCP listener: {}", tcp_listener.unwrap_err());
        return;
    }
    let mut zone = Zone::new("TestZone".to_string(), tcp_listener.unwrap());

    loop {
        zone.update().await;
    }

    // 終了処理
}
