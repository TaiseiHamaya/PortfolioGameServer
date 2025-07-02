use std::{
    collections::LinkedList,
    fmt::Error,
    io::{IoSlice, IoSliceMut},
    net::SocketAddr,
    sync::Arc,
};

use tokio::{
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::Mutex,
};

pub enum ClientRecvBuffer {
    Message(String),
    Close,
}

impl ClientRecvBuffer {
    pub fn generate(buffer: &IoSliceMut) -> Result<ClientRecvBuffer, Error> {
        if buffer.is_empty() {
            return Ok(ClientRecvBuffer::Close);
        }

        match buffer[0..4].try_into().map(u32::from_le_bytes).ok() {
            Some(1) => {
                let lenght_bytes = buffer[4..8].try_into();
                if lenght_bytes.is_err() {
                    return Err(Error);
                }
                let message_length = u32::from_be_bytes(lenght_bytes.unwrap()) as usize;

                Ok(ClientRecvBuffer::Message(
                    buffer[8..8 + message_length]
                        .iter()
                        .map(|c| *c as char)
                        .collect(),
                ))
            }
            Some(_) => Err(Error),
            None => Err(Error),
        }
    }
}

pub enum ClientSendBuffer {
    Message(String),
    Disconnect,
}

impl ClientSendBuffer {
    fn to_stream_write(&self) -> Vec<u8> {
        match self {
            ClientSendBuffer::Message(msg) => {
                let mut buffer = Vec::new();
                buffer.extend(1u32.to_le_bytes());
                buffer.extend(msg.len().to_be_bytes().to_vec());
                buffer
            }
            ClientSendBuffer::Disconnect => "".as_bytes().to_vec(),
        }
    }

    fn buffer_len(&self) -> usize {
        match self {
            ClientSendBuffer::Message(msg) => 8 + msg.len(), // 4 bytes for type + 4 bytes for length + message length
            ClientSendBuffer::Disconnect => 0,               // Disconnect has no data
        }
    }
}

pub struct TcpClient {
    recv_stream: OwnedReadHalf,
    send_stream: OwnedWriteHalf,
    addr: SocketAddr,

    recv_messages: Vec<ClientRecvBuffer>,
    send_messages: LinkedList<ClientSendBuffer>,

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

    pub fn message(&mut self, message: String) {
        self.send_messages
            .push_back(ClientSendBuffer::Message(message));
    }

    pub fn send(&mut self) {
        let error_counter = Arc::clone(&self.error_counter);
        // 送信内容を移動
        // ---------- 送信用Bufferの生成 ----------
        // IoSliceは所有権を持たないので、Vec配列で一時的に保持
        let mut send_buffers_owner = Vec::new();
        send_buffers_owner.reserve(self.send_messages.len());
        self.send_messages.iter().for_each(|send_message| {
            let buffer = send_message.to_stream_write();
            send_buffers_owner.push(buffer); // push時に置換が行われる可能性
        });
        let mut sliced_send_buffers = Vec::new();
        sliced_send_buffers.reserve(send_buffers_owner.len());
        // 送信用にsend_buffersをIoSliceに変換
        send_buffers_owner.iter().for_each(|buffer| {
            let slice = IoSlice::new(buffer.as_slice());
            sliced_send_buffers.push(slice);
        });
        // ---------- 送信処理 ----------
        // streamのlock
        //let result = locked_stream.try_write_vectored(&sliced_send_buffers.as_slice());
        let result = self.send_stream.try_write_vectored(&sliced_send_buffers);

        // ---------- 送信処理 ----------
        match result {
            Ok(size) => {
                let error_count = error_counter.try_lock();
                if error_count.is_err() {
                    println!("Failed to lock error counter: {:?}", error_count.err());
                    return; // Exit if we can't lock the error counter
                }
                *error_count.unwrap() = 0; // Reset error count on successful send
                let total_length: usize =
                    self.send_messages.iter().map(|msg| msg.buffer_len()).sum();
                if size < total_length {
                    println!(
                        "Warning: Not all messages were sent. Sent {} bytes, expected {}",
                        size, total_length
                    );
                    self.send_messages.clear();
                } else {
                    self.send_messages.clear(); // Clear messages after successful send
                }
            }
            Err(e) => {
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

    pub fn recv(&mut self) {
        // ----------  ----------
        let mut buffer = Vec::new();
        match self.recv_stream.try_read_vectored(&mut buffer) {
            Ok(0) => {
                println!("Connection closed by peer");
                self.recv_messages.push(ClientRecvBuffer::Close);
            }
            Ok(_) => {
                buffer.iter().for_each(|recv| {
                    let recv_data = ClientRecvBuffer::generate(recv);
                    if recv_data.is_err() {
                        println!("Failed to parse received data: {:?}", recv_data.err());
                        return;
                    }
                    let recv_data = recv_data.unwrap();
                    self.recv_messages.push(recv_data);
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

    pub fn get_recv_messages(&self) -> &Vec<ClientRecvBuffer> {
        &self.recv_messages
    }

    pub fn disconnect(&mut self) {
        self.send_messages.push_back(ClientSendBuffer::Disconnect);
    }
}
