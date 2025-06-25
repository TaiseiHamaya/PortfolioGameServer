use std::collections::HashMap;

use std::future::poll_fn;
use std::task::Poll;
use tokio::net::TcpListener;

use nalgebra::Point3;

use crate::entity::entity::Entity;

use crate::entity::player::Player;

pub struct Zone {
    name: String,
    players: HashMap<u64, Player>,

    tcp_listener: TcpListener,
}

impl Zone {
    pub fn new(name: String, listner: TcpListener) -> Self {
        Zone {
            name,
            players: HashMap::new(),

            tcp_listener: listner,
        }
    }

    fn add_player(&mut self, player: Player) {
        self.players.insert(player.id(), player);
    }

    fn remove_player(&mut self, player_id: u64) {
        self.players.remove(&player_id);
    }

    pub async fn recv_all(&mut self ) {
        poll_fn(|context| match self.tcp_listener.poll_accept(context) {
            Poll::Ready(Ok((stream, addr))) => {
                // 後でDB接続に変える
                println!("Accepted connection from {}", addr);
                let player_id = self.players.len() as u64; // 簡易的なID生成
                let position = Point3::new(0.0, 0.0, 0.0); // 初期位置は(0, 0, 0)
                let player = Player::new(player_id, stream, position);
                self.add_player(player);
                Poll::Ready(())
            }
            Poll::Ready(Err(e)) => {
                eprintln!("Accept error: {} (Zone: {})", e, self.name);
                Poll::Ready(())
            }
            Poll::Pending => Poll::Pending,
        })
        .await;
    }

    pub async fn update(&mut self) {
        // update player
        self.players.iter_mut().for_each(|(_, player)| {
            player.update();
        });
    }
}
