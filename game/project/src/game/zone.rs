use std::collections::HashMap;
use std::task::{Context, Poll};

use futures::{StreamExt, stream};
use tokio::net::TcpListener;

use nalgebra::Point3;

use super::tcp_client::TcpClient;
use super::zone_request_chash::ZoneRequestChash;
use crate::containts::containts_director::ContaintsDirector;
use crate::entity::entity::Entity;
use crate::entity::player::Player;

use crate::proto::*;
use protobuf::*;

pub struct Zone {
    name: String,
    players: HashMap<u64, Player>,

    containts_directors: Vec<ContaintsDirector>,

    tcp_listener: TcpListener,

    player_tcp_streams: HashMap<u64, TcpClient>,

    zone_request_chash: ZoneRequestChash,
}

impl Zone {
    pub fn new(name: String, listner: TcpListener) -> Self {
        Zone {
            name,
            players: HashMap::new(),

            containts_directors: Vec::new(),

            tcp_listener: listner,
            player_tcp_streams: HashMap::new(),

            zone_request_chash: ZoneRequestChash::new(),
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

                // ログイン要求をチャッシュに追加
                self.zone_request_chash
                    .push_login(player, TcpClient::new(stream, addr));

                let player_name = format!("Player{}", player_id);

                // 既存プレイヤーに通知するパケットを作成
                let mut notify_packet = crate::proto::Packet::new();
                notify_packet.set_loginPacketType(crate::proto::LoginPacketType::Loginnotification); // パケットタイプ
                let mut payload: LoginNotificationBody = crate::proto::LoginNotificationBody::new();
                payload.set_userId(player_id);
                payload.set_username(player_name);
                notify_packet.set_payload(payload.serialize().unwrap()); // 中身

                // 既存プレイヤーに通知
                self.player_tcp_streams.iter_mut().for_each(|(_, client)| {
                    client.stack_packet(notify_packet.clone());
                });
            }
            Poll::Ready(Err(e)) => {
                eprintln!("Accept error: {} (Zone: {})", e, self.name);
            }
            Poll::Pending => {}
        }
    }

    pub async fn recv_all(&mut self) {
        stream::iter(self.player_tcp_streams.values_mut())
            .for_each_concurrent(None, |client| async move {
                client.recv().await;
            })
            .await;
    }

    pub async fn update(&mut self) {
        self.player_tcp_streams
            .iter()
            .for_each(|(player_id, client)| {
                let messages = client.get_recv_messages();
                for msg in messages {
                    match msg.category() {
                        crate::proto::packet::CategoryOneof::LogoutPacketType(
                            crate::proto::LogoutPacketType::Logoutrequest,
                        ) => {
                            self.zone_request_chash.push_logout(*player_id);
                        }
                        crate::proto::packet::CategoryOneof::SyncPacketType(
                            crate::proto::SyncPacketType::Synctransform,
                        ) => {
                            let mut transform_packet = crate::proto::TransformSyncBody::new();
                            let parsed = transform_packet.clear_and_parse(&msg.payload());
                            if parsed.is_err() {
                                println!("Failed to parse TransformSyncBody: {:?}", parsed.err());
                                continue;
                            }
                            let player = self.players.get_mut(player_id);
                            if player.is_none() {
                                println!("Player not found for ID: {}", player_id);
                                continue;
                            }
                            let player = player.unwrap();
                            let position = transform_packet.position();
                            let rotation = transform_packet.rotation();

                            let player_position = player.position_mut();
                            *player_position =
                                Point3::new(position.x(), position.y(), position.z());
                        }
                        crate::proto::packet::CategoryOneof::TextMessageType(
                            crate::proto::TextMessageType::Messagechatsend,
                        ) => {}
                        _ => {
                            println!(
                                "Unknown packet type received from player {}: {:?}",
                                player_id,
                                msg.category()
                            );
                        }
                    }
                }
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

        // プレイヤー追加/削除処理
        let login_chash = self.zone_request_chash.get_login_chash_take();
        for login in login_chash {
            self.player_tcp_streams
                .insert(login.player.id(), login.tcp_client);
            self.players.insert(login.player.id(), login.player);
        }

        let logout_chash = self.zone_request_chash.get_logout_chash_take();
        for logout in logout_chash {
            self.players.remove(&logout.entity_id);
            self.player_tcp_streams.remove(&logout.entity_id);
        }
        self.zone_request_chash.clear();
    }

    pub async fn send_all(&mut self) {
        self.player_tcp_streams.iter_mut().for_each(|(_, client)| {
            client.send();
        });
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
