use std::{
    collections::LinkedList,
    io::{ErrorKind, IoSlice},
    net::SocketAddr,
    sync::Arc,
};

use log;
use protobuf::Serialize;
use tokio::{
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::Mutex,
};

use super::receive_buffer;

pub struct TcpClient {
    recv_stream: OwnedReadHalf,
    send_stream: OwnedWriteHalf,
    addr: SocketAddr,

    recv_messages: Vec<crate::proto::Packet>,
    send_messages: LinkedList<crate::proto::Packet>,

    recv_buffer: receive_buffer::ReceiveBuffer,
    error_counter: Arc<Mutex<u32>>,

    is_disconnected: bool,
}

impl TcpClient {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> Self {
        let (read_half, write_half) = stream.into_split();

        TcpClient {
            recv_stream: read_half,
            send_stream: write_half,
            addr,
            recv_messages: Vec::new(),
            send_messages: LinkedList::new(),
            recv_buffer: receive_buffer::ReceiveBuffer::new(),
            error_counter: Arc::new(Mutex::new(0)),
            is_disconnected: false,
        }
    }

    pub fn stack_packet(&mut self, packet: crate::proto::Packet) {
        self.send_messages.push_back(packet);
    }

    pub fn send(&mut self) {
        let error_counter = Arc::clone(&self.error_counter);
        // 送信内容を移動
        // ---------- 送信用Bufferの生成 ----------
        let mut temp_send_buffers: LinkedList<Vec<u8>> = LinkedList::new();
        for msg in &self.send_messages {
            let buffer = msg.serialize();
            if buffer.is_err() {
                log::error!("Failed to serialize message: {:?}", buffer.err());
                continue;
            }
            let buffer = buffer.unwrap();
            let buffer_size = (buffer.len() as u32).to_le_bytes().to_vec();
            temp_send_buffers.push_back(buffer_size);
            temp_send_buffers.push_back(buffer);
        }
        let mut sliced_send_buffers: Vec<IoSlice> = Vec::new();
        if temp_send_buffers.is_empty() {
            return; // 送信するものがない場合は終了
        }

        for buffer in &temp_send_buffers {
            sliced_send_buffers.push(IoSlice::new(&buffer));
        }

        // ---------- 送信処理 ----------
        let result = self.send_stream.try_write_vectored(&sliced_send_buffers);

        // ---------- エラーハンドリング ----------
        match result {
            Ok(_) => {
                let error_count = error_counter.try_lock();
                if error_count.is_err() {
                    log::error!("Failed to lock error counter: {:?}", error_count.err());
                    return;
                }
                *error_count.unwrap() = 0; // エラーカウンタをリセット
                self.send_messages.clear(); // 送信成功したので送信バッファをクリア
            }
            Err(e) => {
                match e.kind() {
                    ErrorKind::WriteZero => {}
                    _ => {
                        log::error!("Failed to send messages: {:?}", e);
                        let error_count = error_counter.try_lock();
                        if error_count.is_err() {
                            log::error!("Failed to lock error counter: {:?}", error_count.err());
                            return; // Exit if we can't lock the error counter
                        }
                        *error_count.unwrap() += 1;
                    }
                }
            }
        }
    }

    pub async fn recv(&mut self) {
        // 受信チェック
        // if self.recv_stream.readable().await.is_err() {
        //     return;
        // }
        // ---------- 受信処理 ----------
        let mut temp_buffer = vec![0u8; 4096];
        let result = self.recv_stream.try_read(&mut temp_buffer);
        if result.is_err() {
            let error_opt = result.err();
            if let Some(error) = error_opt {
                if error.kind() != ErrorKind::WouldBlock {
                    log::error!("Failed to receive data: {:?}", error);
                    let error_count = self.error_counter.try_lock();
                    if error_count.is_err() {
                        log::error!("Failed to lock error counter: {:?}", error_count.err());
                        return; // Exit if we can't lock the error counter
                    }
                    *error_count.unwrap() += 1;
                }
            }
            return;
        }
        let bytes_read = result.unwrap();
        if bytes_read == 0 {
            // 接続終了
            log::info!("Connection closed by peer");
            self.is_disconnected = true;
            return;
        }

        // 受信したデータをバッファに追加
        let mut packet_data = temp_buffer.drain(0..bytes_read).collect::<Vec<u8>>();
        let packets = self.recv_buffer.read_stream(&mut packet_data);
        self.recv_messages.extend(packets);
    }

    pub fn check_error(&self) -> bool {
        let error_count = self.error_counter.try_lock();
        if error_count.is_err() {
            log::error!("Failed to lock error counter: {:?}", error_count.err());
            return false; // Indicate that the connection should remain open
        }
        if *error_count.unwrap() > 100 {
            log::error!("Too many errors, closing connection");
            return true; // Indicate that the connection should be closed
        }
        false // No error, keep the connection open
    }

    pub fn into_recv_messages(&mut self) -> Vec<crate::proto::Packet> {
        let result = std::mem::take(&mut self.recv_messages);
        return result;
    }

    pub fn is_disconnected(&self) -> bool {
        self.is_disconnected
    }
}
