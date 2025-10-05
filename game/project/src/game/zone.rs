use std::collections::HashMap;
use std::task::{Context, Poll};

use futures::{StreamExt, stream};
use log;
use tokio::net::TcpListener;

use nalgebra::Point3;

use super::client::TcpClient;
use super::zone_request_chash::ZoneRequestChash;
use crate::contents::containts_director::ContaintsDirector;
use crate::entity::entity::Entity;
use crate::entity::player::Player;

use crate::game::client::{self, CommandTrait};
use crate::proto::{self, *};
use protobuf::*;

pub struct Zone {
    name: String,
    players: HashMap<u64, client::Cluster>,

    containts_directors: Vec<ContaintsDirector>,

    tcp_listener: TcpListener,

    zone_request_chash: ZoneRequestChash,
}

impl Zone {
    pub fn new(name: String, listner: TcpListener) -> Self {
        Zone {
            name,
            players: HashMap::new(),

            containts_directors: Vec::new(),

            tcp_listener: listner,

            zone_request_chash: ZoneRequestChash::new(),
        }
    }

    pub async fn update(&mut self) {
        // パケット受信
        self.recv_all().await;

        // クライアント接続受付
        self.accept_client().await;

        // クライアントのパケット処理
        self.players.iter_mut().for_each(|(_, client)| {
            client.process_packets();
        });

        // 通常更新処理
        self.players.iter_mut().for_each(|(_, player)| {
            player.update();
        });

        // コンテンツディレクターの処理
        self.containts_directors.iter_mut().for_each(|director| {
            director.update();
        });

        // コマンド処理
        self.execute_client_commands();

        // 位置同期
        self.sync_entity_transform_all();

        // クライアント追加/削除処理
        self.add_client_accepted();
        self.remove_client_chashed();

        // チャッシュクリア
        self.zone_request_chash.clear();

        // パケット送信
        self.send_all().await;
    }

    async fn recv_all(&mut self) {
        stream::iter(self.players.values_mut())
            .for_each_concurrent(None, |client| async move {
                client.recv_packets().await;
            })
            .await;
    }

    async fn accept_client(&mut self) {
        let waker = futures::task::noop_waker();
        let mut cx = Context::from_waker(&waker);
        match self.tcp_listener.poll_accept(&mut cx) {
            Poll::Ready(Ok((stream, addr))) => {
                // 後でDB接続に変える
                log::info!("Accepted connection from {}", addr);
                let player_id = self.players.len() as u64;
                let player_name = format!("Player{}", player_id);

                let position = Point3::new(0.0, 0.0, -10.0); // 初期位置は(0, 0, -10)
                let player = Player::new(player_id, position, 10000);
                let client_cluster =
                    client::Cluster::new(player, player_name.clone(), TcpClient::new(stream, addr));

                // ログイン要求をチャッシュに追加
                self.zone_request_chash.push_login(client_cluster);

                // 既存プレイヤーに通知するパケットを作成
                let mut notify_packet = crate::proto::Packet::new();
                notify_packet.set_loginPacketType(crate::proto::LoginPacketType::Loginnotification); // パケットタイプ
                let mut payload: LoginNotificationBody = crate::proto::LoginNotificationBody::new();
                payload.set_id(player_id);
                payload.set_username(player_name);
                notify_packet.set_payload(payload.serialize().unwrap()); // 中身

                // 既存プレイヤーに通知
                self.players.iter_mut().for_each(|(_, cluster)| {
                    cluster.stack_packet(notify_packet.clone());
                });
            }
            Poll::Ready(Err(e)) => {
                log::error!("Accept error: {} (Zone: {})", e, self.name);
            }
            Poll::Pending => {}
        }
    }

    async fn send_all(&mut self) {
        self.players.iter_mut().for_each(|(_, client)| {
            client.send_packets();
        });
    }

    // プレイヤー追加
    fn add_client_accepted(&mut self) {
        let mut login_chash = self.zone_request_chash.get_login_chash_take();
        login_chash.into_iter().for_each(|mut login| {
            // 接続完了通知
            login.client_cluster.on_accepted();
            // 接続したクライアントに既存プレイヤーの情報を送信
            self.players.iter_mut().for_each(|(id, cluster)| {
                let mut packet = crate::proto::Packet::new();
                packet.set_loginPacketType(LoginPacketType::Loginnotification);
                let mut body = LoginNotificationBody::new();
                body.set_id(*id);
                body.set_username(cluster.player_name().clone());
                let mut position = proto::Vector3::new();
                let cluster_position = cluster.player().position();
                position.set_x(cluster_position.x);
                position.set_y(cluster_position.y);
                position.set_z(cluster_position.z);
                body.set_position(position);
                packet.set_payload(body.serialize().unwrap());
                // パケットを積む
                login.client_cluster.stack_packet(packet);
            });
            // プレイヤーリストに追加
            self.players.insert(login.id, login.client_cluster);
        });
    }

    fn execute_client_commands(&mut self) {
        let commands: Vec<Box<dyn CommandTrait>> = self
            .players
            .values_mut()
            .flat_map(|cluster| cluster.take_commands())
            .collect();
        for command in commands {
            command.execute(self);
        }
    }

    // アプリケーション内での削除処理
    fn remove_client_chashed(&mut self) {
        let logout_chash = self.zone_request_chash.get_logout_chash_take();

        logout_chash.into_iter().for_each(|logout| {
            self.players.remove(&logout.entity_id);
        });
    }

    // クライアントからの切断要求
    pub fn dissconnect_request(&mut self, player_id: &u64) {
        log::info!("Player {} requested disconnection.", player_id);
        // ログアウト要求をチャッシュに追加
        self.zone_request_chash.push_logout(*player_id);
        // パケット送信
        // 要求を受けたクライアントに切断パケットを送信
        {
            if let Some(cluster) = self.players.get_mut(player_id) {
                let mut packet = crate::proto::Packet::new();
                packet.set_logoutPacketType(crate::proto::LogoutPacketType::Logoutresponse);
                let mut body = crate::proto::LogoutResponseBody::new();
                body.set_isSuccessed(true);
                let payload = body.serialize();
                if payload.is_ok() {
                    packet.set_payload(payload.unwrap());
                    cluster.stack_packet(packet);
                }
            }
        }

        // その他のクライアントにログアウト通知パケットを送信
        {
            let mut packet = crate::proto::Packet::new();
            packet.set_logoutPacketType(crate::proto::LogoutPacketType::Logoutnotification);
            let mut body = crate::proto::LogoutNotificationBody::new();
            body.set_id(*player_id);
            let payload = body.serialize();
            if payload.is_ok() {
                packet.set_payload(payload.unwrap());
                self.players.iter_mut().for_each(|(_, cluster)| {
                    cluster.stack_packet(packet.clone());
                });
            }
        }
    }

    // サーバーからエラーとして切断
    pub fn dissconnect_client_force(&mut self, player_id: &u64) {
        log::info!("Forcefully disconnecting player {}.", player_id);
        // ログアウト要求をチャッシュに追加
        self.zone_request_chash.push_logout(*player_id);

        // 既存プレイヤーに通知するパケットを作成
        {
            let mut packet = crate::proto::Packet::new();
            packet.set_logoutPacketType(crate::proto::LogoutPacketType::Logoutnotification);
            let mut body = crate::proto::LogoutNotificationBody::new();
            body.set_id(*player_id);
            let payload = body.serialize();
            if payload.is_ok() {
                packet.set_payload(payload.unwrap());
                self.players.iter_mut().for_each(|(_, cluster)| {
                    cluster.stack_packet(packet.clone());
                });
            }
        }
    }

    // チャットメッセージを全員に送信
    pub fn broadcast_chat_message(&mut self, id: u64, message: &str) {
        log::info!("Broadcasting chat message from {}: {}", id, message);
        let mut packet = crate::proto::Packet::new();
        packet.set_textMessageType(crate::proto::TextMessageType::Messagechatreceive);
        let mut body = crate::proto::ChatMessageBody::new();
        body.set_id(id);
        body.set_message(message.to_string());
        let payload = body.serialize();
        if payload.is_err() {
            return;
        }
        packet.set_payload(payload.unwrap());
        self.players.iter_mut().for_each(|(_, cluster)| {
            cluster.stack_packet(packet.clone());
        });
    }

    pub fn sync_entity_transform_all(&mut self) {
        let timestamp = chrono::Utc::now().timestamp_micros() as u64;
        let transforms = self
            .players
            .iter()
            .map(|(id, cluster)| {
                let position = cluster.player().position();
                (*id, timestamp, *position)
            })
            .collect::<Vec<_>>();
        transforms
            .into_iter()
            .for_each(|(entity_id, timestamp, position)| {
                self.sync_entity_transform(entity_id, timestamp, position);
            });
    }

    // 位置の同期をクライアントに通知
    pub fn sync_entity_transform(&mut self, entity_id: u64, timestamp: u64, position: Point3<f32>) {
        let mut packet = crate::proto::Packet::new();
        packet.set_syncPacketType(crate::proto::SyncPacketType::Synctransform);
        let mut body = crate::proto::TransformSyncBody::new();
        body.set_id(entity_id);
        body.set_timestamp(timestamp);
        let mut pos = crate::proto::Vector3::new();
        pos.set_x(position.x);
        pos.set_y(position.y);
        pos.set_z(position.z);
        body.set_position(pos);
        let payload = body.serialize();
        if payload.is_err() {
            return;
        }
        packet.set_payload(payload.unwrap());
        self.players.iter_mut().for_each(|(player_id, cluster)| {
            if entity_id == *player_id {
                return; // 自分には送らない
            }
            cluster.stack_packet(packet.clone());
        });
    }
}
