use tokio::net::TcpListener;

use super::zone::Zone;

pub async fn run() {
    // 初期化
    let tcp_listener = TcpListener::bind("127.0.0.1:3215").await;
    if tcp_listener.is_err() {
        eprintln!("Error binding TCP listener: {}", tcp_listener.unwrap_err());
        return;
    }
    let mut zone = Zone::new("TestZone".to_string(), tcp_listener.unwrap());

    loop {
        // proxy受信
        zone.recv_all().await;

        // proxy内にプログラム終了処理がある場合にbreak
        if false {
            break;
        }

        // 更新
        zone.update().await;

        // proxy送信
    }

    // 終了処理
}
