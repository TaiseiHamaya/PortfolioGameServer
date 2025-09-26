use std::{
    collections::LinkedList,
    fmt::Error,
    io::{ErrorKind, IoSlice},
    net::SocketAddr,
    result,
    sync::Arc,
};

use protobuf::{ClearAndParse, MergeFrom, Serialize};
use tokio::{
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf}, TcpStream
    }, runtime::Handle, sync::Mutex
};

use crate::proto::Packet;

pub struct TcpClient {
    recv_stream: OwnedReadHalf,
    send_stream: OwnedWriteHalf,
    addr: SocketAddr,

    recv_messages: Vec<crate::proto::Packet>,
    send_messages: LinkedList<crate::proto::Packet>,

    error_counter: Arc<Mutex<u32>>,
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
            error_counter: Arc::new(Mutex::new(0)),
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
                println!("Failed to serialize message: {:?}", buffer.err());
                continue;
            }
            let buffer = buffer.unwrap();
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
        // streamのlock
        //let result = locked_stream.try_write_vectored(&sliced_send_buffers.as_slice());
        let result = self.send_stream.try_write_vectored(&sliced_send_buffers);

        // ---------- エラーハンドリング ----------
        match result {
            Ok(_) => {
                let error_count = error_counter.try_lock();
                if error_count.is_err() {
                    println!("Failed to lock error counter: {:?}", error_count.err());
                    return;
                }
                *error_count.unwrap() = 0; // エラーカウンタをリセット
            }
            Err(e) => {
                match e.kind() {
                    ErrorKind::WriteZero => {}
                    _ => {
                        println!("Failed to send messages: {:?}", e);
                        let error_count = error_counter.try_lock();
                        if error_count.is_err() {
                            println!("Failed to lock error counter: {:?}", error_count.err());
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
        if self.recv_stream.readable().await.is_err() {
            return;
        }
        // ---------- 受信処理 ----------
        let mut buffer = Vec::new();
        match self.recv_stream.try_read_vectored(&mut buffer) {
            Ok(0) => {
                // 接続終了
                println!("Connection closed by peer");
            }
            Ok(_) => {
                // 通常パケット
                buffer.iter().for_each(|recv: &std::io::IoSliceMut<'_>| {
                    let mut packet = crate::proto::Packet::new();
                    let result = packet.clear_and_parse(recv);
                    if result.is_err() {
                        println!("Failed to parse packet: {:?}", result.err());
                        return;
                    }
                    self.recv_messages.push(packet);
                });
            }
            Err(e) => {
                println!("Failed to read from stream: {:?}", e);
            }
        }
    }

    pub fn check_error(&self) -> bool {
        let error_count = self.error_counter.try_lock();
        if error_count.is_err() {
            println!("Failed to lock error counter: {:?}", error_count.err());
            return false; // Indicate that the connection should remain open
        }
        if *error_count.unwrap() > 100 {
            println!("Too many errors, closing connection");
            return true; // Indicate that the connection should be closed
        }
        false // No error, keep the connection open
    }

    pub fn get_recv_messages(&self) -> &Vec<crate::proto::Packet> {
        &self.recv_messages
    }

    pub fn disconnect(&mut self) {
        // 強制disconnect
        // protobufでの実装忘れ
        //self.send_messages.push_back(ClientSendBuffer::Disconnect);
    }
}
