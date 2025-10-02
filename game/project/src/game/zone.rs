use std::collections::HashMap;
use std::task::{Context, Poll};

use futures::{StreamExt, stream};
use tokio::net::TcpListener;

use nalgebra::Point3;

use super::client::TcpClient;
use super::zone_request_chash::ZoneRequestChash;
use crate::containts::containts_director::ContaintsDirector;
use crate::entity::player::Player;

use crate::game::client::{self, CommandTrait};
use crate::proto::*;
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
                println!("Accepted connection from {}", addr);
                let player_id = self.players.len() as u64;
                let position = Point3::new(0.0, 0.0, -10.0); // 初期位置は(0, 0, -10)
                let player = Player::new(player_id, position, 10000);
                let client_cluster = client::Cluster::new(player, TcpClient::new(stream, addr));

                // ログイン要求をチャッシュに追加
                self.zone_request_chash.push_login(client_cluster);

                let player_name = format!("Player{}", player_id);

                // 既存プレイヤーに通知するパケットを作成
                let mut notify_packet = crate::proto::Packet::new();
                notify_packet.set_loginPacketType(crate::proto::LoginPacketType::Loginnotification); // パケットタイプ
                let mut payload: LoginNotificationBody = crate::proto::LoginNotificationBody::new();
                payload.set_userId(player_id);
                payload.set_username(player_name);
                notify_packet.set_payload(payload.serialize().unwrap()); // 中身

                // 既存プレイヤーに通知
                self.players.iter_mut().for_each(|(_, cluster)| {
                    cluster.stack_packet(notify_packet.clone());
                });
            }
            Poll::Ready(Err(e)) => {
                eprintln!("Accept error: {} (Zone: {})", e, self.name);
            }
            Poll::Pending => {}
        }
    }

    async fn send_all(&mut self) {
        self.players.iter_mut().for_each(|(_, client)| {
            client.send_packets();
        });
    }

    fn add_client_accepted(&mut self) {
        // プレイヤー追加/削除処理
        let login_chash = self.zone_request_chash.get_login_chash_take();
        login_chash.into_iter().for_each(|mut login| {
            login.client_cluster.on_accepted();
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
            body.set_userId(*player_id);
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
        self.zone_request_chash.push_logout(*player_id);

        // 既存プレイヤーに通知するパケットを作成
        {
            let mut packet = crate::proto::Packet::new();
            packet.set_logoutPacketType(crate::proto::LogoutPacketType::Logoutnotification);
            let mut body = crate::proto::LogoutNotificationBody::new();
            body.set_userId(*player_id);
            let payload = body.serialize();
            if payload.is_ok() {
                packet.set_payload(payload.unwrap());
                self.players.iter_mut().for_each(|(_, cluster)| {
                    cluster.stack_packet(packet.clone());
                });
            }
        }
    }

    pub fn broadcast_chat_message(&mut self, id: u64, message: &str) {
        let mut packet = crate::proto::Packet::new();
        packet.set_textMessageType(crate::proto::TextMessageType::Messagechatreceive);
        let mut body = crate::proto::ChatMessageBody::new();
        body.set_userId(id);
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
}
