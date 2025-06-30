use std::{
    fmt::Error,
    io::{IoSlice, IoSliceMut},
    net::SocketAddr,
    sync::Arc,
};

use tokio::{net::TcpStream, sync::Mutex};

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

enum ClientSendBuffer {
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
}

pub struct TcpClient {
    stream: Arc<Mutex<TcpStream>>,
    addr: SocketAddr,

    recv_message_buffers: Vec<ClientRecvBuffer>,
    send_message_buffers: Vec<ClientSendBuffer>,

    error_counter: Arc<Mutex<u32>>,
}

impl TcpClient {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> Self {
        TcpClient {
            stream: Arc::new(Mutex::new(stream)),
            addr,
            recv_message_buffers: Vec::new(),
            send_message_buffers: Vec::new(),
            error_counter: Arc::new(Mutex::new(0)),
        }
    }

    pub fn message(&mut self, message: String) {
        self.send_message_buffers
            .push(ClientSendBuffer::Message(message));
    }

    pub fn send(&mut self) {
        let stream_temp = Arc::clone(&self.stream);
        let error_counter = Arc::clone(&self.error_counter);
        let mut send_messages: Vec<ClientSendBuffer> = Vec::new();
        std::mem::swap(&mut send_messages, &mut self.send_message_buffers);
        tokio::spawn(async move {
            // streamのlock
            let locked_stream = stream_temp.lock().await;
            // 書き込み可能チェック
            if locked_stream.writable().await.is_err() {
                println!("Stream is not writable");
                let mut error_count = error_counter.lock().await;
                *error_count += 1;
                return;
            }
            // IoSliceは所有権を持たないので、Vec配列で保持
            let mut send_buffers = Vec::new();
            send_buffers.reserve(send_messages.len());
            for send_message in send_messages {
                let buffer = send_message.to_stream_write();
                send_buffers.push(buffer); // push時に置換が行われる可能性
            }
            let mut sliced_send_buffers = Vec::new();
            sliced_send_buffers.reserve(send_buffers.len());
            // バッファに詰める
            for buffer in send_buffers.iter_mut() {
                sliced_send_buffers.push(IoSlice::new(&buffer.as_slice()));
            }
            let result = locked_stream.try_write_vectored(&sliced_send_buffers.as_slice());
            match result {
                Ok(_) => {
                    println!("Sent messages successfully");
                    let mut error_count = error_counter.lock().await;
                    *error_count = 0; // Reset error count on successful send
                }
                Err(e) => {
                    println!("Failed to send messages: {:?}", e);
                    let mut error_count = error_counter.lock().await;
                    *error_count += 1;
                }
            }
        });
    }

    pub fn recv(&mut self) {
        let locked_stream = self.stream.blocking_lock();
        let mut buffer = Vec::new();
        match locked_stream.try_read_vectored(&mut buffer) {
            Ok(0) => {
                println!("Connection closed by peer");
                self.recv_message_buffers.push(ClientRecvBuffer::Close);
            }
            Ok(_) => {
                buffer.iter().for_each(|recv| {
                    let recv_data = ClientRecvBuffer::generate(recv);
                    if recv_data.is_err() {
                        println!("Failed to parse received data: {:?}", recv_data.err());
                        return;
                    }
                    let recv_data = recv_data.unwrap();
                    self.recv_message_buffers.push(recv_data);
                });
            }
            Err(e) => {
                println!("Failed to read from stream: {:?}", e);
            }
        }
    }

    pub fn check_error(&self) -> bool {
        let error_count = self.error_counter.blocking_lock();
        if *error_count > 100 {
            println!("Too many errors, closing connection");
            return true; // Indicate that the connection should be closed
        }
        false // No error, keep the connection open
    }

    pub fn disconnect(&mut self) {
        self.send_message_buffers.push(ClientSendBuffer::Disconnect);
    }
}
