use log;
use std::{net::Ipv4Addr, time::Duration};
use tokio::net::TcpListener;
use futures_ticker;
use chrono::Utc;

use super::zone::Zone;

pub async fn run() {
    // 初期化
    let tcp_listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, 3215)).await;
    if tcp_listener.is_err() {
        log::error!("Error binding TCP listener: {}", tcp_listener.unwrap_err());
        return;
    }
    let mut zone = Zone::new("TestZone".to_string(), tcp_listener.unwrap());
    let futures_ticker = futures_ticker::Ticker::new(Duration::from_millis(50));
    let mut tick_count: u64 = 0;

    loop {
        let now = tokio::time::Instant::now();
        let next = tokio::time::Instant::from_std(futures_ticker.next_tick());
        tokio::time::sleep_until(next).await;

        log::info!("Tick {} at {}", tick_count, Utc::now());

        if next < now {
            log::warn!("Tick is running behind schedule!");
        }

        zone.update().await;
        tick_count += 1;
    }

    // 終了処理
}
