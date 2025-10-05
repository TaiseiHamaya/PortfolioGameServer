use super::command::*;
use super::tcp_client::*;

use chrono::NaiveTime;
use futures::executor::EnterError;
use log;
use nalgebra::Point3;
use protobuf::ClearAndParse;

use crate::entity::{entity::Entity, player::Player};

use crate::game::client::command;
use crate::game::client::command::CommandTrait;
use crate::proto;
use protobuf::Serialize;

pub struct Cluster {
    player: Player,
    name: String,
    tcp: TcpClient,

    command_buffers: Vec<Box<dyn CommandTrait>>,
}

impl Cluster {
    pub fn new(player: Player, name: String, tcp: TcpClient) -> Self {
        Cluster {
            player,
            name,
            tcp,
            command_buffers: Vec::new(),
        }
    }

    pub async fn recv_packets(&mut self) {
        self.tcp.recv().await;
    }

    pub fn process_packets(&mut self) {
        let messages = self.tcp.into_recv_messages();
        for msg in messages {
            match msg.category() {
                crate::proto::packet::CategoryOneof::LogoutPacketType(
                    crate::proto::LogoutPacketType::Logoutrequest,
                ) => {
                    // ログアウトリクエスト
                    self.command_buffers
                        .push(Box::new(LogoutRequestCommand::new(self.player.id())));
                }
                crate::proto::packet::CategoryOneof::SyncPacketType(
                    crate::proto::SyncPacketType::Synctransform,
                ) => {
                    // 同期
                    let mut transform_packet = crate::proto::TransformSyncBody::new();
                    let parsed = transform_packet.clear_and_parse(&msg.payload());
                    if parsed.is_err() {
                        log::error!("Failed to parse TransformSyncBody: {:?}", parsed.err());
                        continue;
                    }
                    let position = transform_packet.position();


                    let player_position = self.player.position_mut();
                    *player_position = Point3::new(position.x(), position.y(), position.z());
                }
                crate::proto::packet::CategoryOneof::TextMessageType(
                    crate::proto::TextMessageType::Messagechatsend,
                ) => { // テキストチャット
                    let mut chat_packet = crate::proto::ChatMessageBody::new();
                    let parsed = chat_packet.clear_and_parse(&msg.payload());
                    if parsed.is_err() {
                        log::error!("Failed to parse ChatSendBody: {:?}", parsed.err());
                        continue;
                    }
                    let message = chat_packet.message().to_string();
                    log::info!("Player {} says: {}", self.player.id(), message);
                    self.command_buffers.push(Box::new(ChatBroadcastCommand::new(
                        self.player.id(),
                        message,
                    )));
                }
                _ => {
                    // 不明なパケット
                    println!(
                        "Unknown packet type received from player {}: {:?}",
                        self.player.id(),
                        msg.category()
                    );
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.player.update();

        if self.tcp.check_error() {
            self.command_buffers
                .push(Box::new(DisconnectForceCommand::new(self.player.id())));
        }
        if self.tcp.is_disconnected() {
            self.command_buffers
                .push(Box::new(command::LogoutRequestCommand::new(self.player.id())));
        }
    }

    pub fn send_packets(&mut self) {
        self.tcp.send();
    }

    pub fn take_commands(&mut self) -> Vec<Box<dyn CommandTrait>> {
        let commands = std::mem::take(&mut self.command_buffers);
        commands
    }

    pub fn stack_packet(&mut self, packet: proto::Packet) {
        self.tcp.stack_packet(packet);
    }

    pub fn on_accepted(&mut self) {
        let mut notify_packet = crate::proto::Packet::new();
        notify_packet.set_loginPacketType(crate::proto::LoginPacketType::Loginresult); // パケットタイプ
        let mut payload = crate::proto::LoginResultBody::new();
        payload.set_id(self.player.id());
        payload.set_isSuccessed(true);
        payload.set_username(self.name.clone());
        payload.set_position(proto::Vector3::new());
        notify_packet.set_payload(payload.serialize().unwrap()); // 中身
        self.tcp.stack_packet(notify_packet);
    }

    pub fn id(&self) -> u64 {
        self.player.id()
    }

    pub fn player_name(&self) -> &String {
        &self.name
    }

    pub fn player(&self) -> &Player {
        &self.player
    }
}
