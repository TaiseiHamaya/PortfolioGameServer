use std::collections::HashMap;
use std::task::{Context, Poll};
use futures::task::noop_waker;

use tokio::net::TcpListener;

use nalgebra::Point3;

use super::tcp_client::TcpClient;
use crate::containts::containts_director::ContaintsDirector;
use crate::entity::entity::Entity;
use crate::entity::player::Player;

pub struct Zone {
    name: String,
    players: HashMap<u64, Player>,

    containts_directors: Vec<ContaintsDirector>,

    tcp_listener: TcpListener,

    player_tcp_streams: HashMap<u64, TcpClient>,
}

impl Zone {
    pub fn new(name: String, listner: TcpListener) -> Self {
        Zone {
            name,
            players: HashMap::new(),

            containts_directors: Vec::new(),

            tcp_listener: listner,
            player_tcp_streams: HashMap::new(),
        }
    }

    pub async fn join_client(&mut self) {
        let waker = futures::task::noop_waker();
        let mut cx = Context::from_waker(&waker);
        match self.tcp_listener.poll_accept(&mut cx) {
            Poll::Ready(Ok((stream, addr))) => {
                // 後でDB接続に変える
                println!("Accepted connection from {}", addr);
                let player_id = self.players.len() as u64;
                let position = Point3::new(0.0, 0.0, -10.0); // 初期位置は(0, 0, -10)
                let player = Player::new(player_id, position, 10000);

                // サーバープログラム上で追加
                self.player_tcp_streams
                    .insert(player_id, TcpClient::new(stream, addr));
                self.add_player(player);

                // クライアントに通知
                self.player_tcp_streams
                    .iter_mut()
                    .for_each(|(key, client)| {
                        if *key != player_id {
                            let message = format!("New player joined: {}", player_id);
                            client.message(message);
                        } else {
                            let message = format!("Welcome to the zone: {}", self.name);
                            client.message(message);
                        }
                    });
            }
            Poll::Ready(Err(e)) => {
                eprintln!("Accept error: {} (Zone: {})", e, self.name);
            }
            Poll::Pending => {}
        }
    }

    pub async fn recv_all(&mut self) {
        self.player_tcp_streams.iter_mut().for_each(|(_, client)| {
            client.recv();
        });
    }

    pub async fn update(&mut self) {
        self.player_tcp_streams.iter().for_each(|(player_id, client)| {
            client.get_recv_messages().iter().for_each(|message| {
                match message {
                    // メッセージを受信した場合
                    crate::game::tcp_client::ClientRecvBuffer::Message(msg) => {
                        
                    }
                    // 切断要求が来た場合
                    crate::game::tcp_client::ClientRecvBuffer::Close => {
                    }
                }
            });
        });

        // update player
        self.players.iter_mut().for_each(|(_, player)| {
            player.update();
        });

        // update containts_directors
        self.containts_directors.iter_mut().for_each(|director| {
            director.update();
        });

        // check player errors
        let remove_ids: Vec<u64> = self
            .player_tcp_streams
            .iter()
            .filter_map(|(id, client)| {
                if client.check_error() {
                    println!("Removing player {} due to too many errors", id);
                    Some(*id)
                } else {
                    None
                }
            })
            .collect();

        // remove player
        for id in remove_ids {
            self.disconnect_client(&id);
        }
    }

    pub async fn send_all(&mut self) {
        self.player_tcp_streams.iter_mut().for_each(|(_, client)| {
            client.send();
        });
    }

    fn add_player(&mut self, player: Player) {
        self.players.insert(player.id(), player);
    }

    fn remove_player(&mut self, player_id: u64) {
        self.players.remove(&player_id);
        self.player_tcp_streams.remove(&player_id);
    }

    // サーバーからエラーとして切断
    fn disconnect_client(&mut self, player_id: &u64) {
        self.players.remove(&player_id);
        let removed_stram = self.player_tcp_streams.remove(&player_id);
        if removed_stram.is_none() {
            return;
        }

        let mut removed_stram = removed_stram.unwrap();
        // disconnect処理を非同期で行う
        tokio::spawn(async move {
            removed_stram.disconnect();
        });
    }

    // クライアントから切断要求が来たとき
    fn quit_client(&mut self, player_id: u64) {
        println!("Player {} has quit the game", player_id);
        self.players.remove(&player_id);
        let removed_stream = self.player_tcp_streams.remove(&player_id);
        if removed_stream.is_none() {
            return;
        }

        let mut removed_stream = removed_stream.unwrap();
        tokio::spawn(async move {
            // DB保存処理
        });
    }
}
