use bytes::{BufMut, BytesMut};
use engine::events::Event;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc::Sender,
};

use protocol::{
    Message, MessageType,
    errors::ProtocolErrors,
    header::{HEADER_SIZE, Header},
};

use crate::{router::Client, session::Session, utils};

pub struct Connection {
    session: Session,
    socket: TcpStream,
    order_book_tx_sender: Sender<Event>,
    client: Client,
}

impl Connection {
    pub fn new(
        client: Client,
        socket: TcpStream,
        order_book_tx_sender: Sender<Event>,
    ) -> Connection {
        return Connection {
            session: Session::new(),
            socket,
            order_book_tx_sender,
            client,
        };
    }

    async fn send_response<T: protocol::traits::Encode>(
        &mut self,
        message_type: MessageType,
        res: &T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut response: Vec<u8> = Vec::new();

        let mut body = BytesMut::new();
        res.encode(&mut body);

        let header = Header::new(
            0,
            body.len() as u16,
            self.session.seq_num,
            message_type as u8,
        )
        .as_bytes();

        let _ = response.write_all(&header).await;
        let _ = response.write_all(&body).await;

        let _ = self.socket.write_all(&response).await?;
        let _ = self.socket.flush().await?;

        self.session.seq_num += 1;

        return Ok(());
    }

    pub async fn handle_connection(&mut self) {
        let mut event_rx = self.client.events_rx.take().unwrap();
        loop {
            let expected_seq_number = self.session.seq_num + 1;
            let mut client_header = [0u8; HEADER_SIZE];

            tokio::select! {
                n = self.socket.read(&mut client_header) => {
                    let n = match n {
                        Ok(n) => n,
                        Err(_) => break,
                    };

                    if n == 0 {
                        return;
                    }

                    let client_header = match Header::from(&client_header) {
                        Ok(header) => header,
                        Err(err) => {
                            let body = ProtocolErrors::HeaderError(err);

                            if let Err(_) = self.send_response(MessageType::Error, &body).await {
                                break;
                            }
                               continue;
                            }
                        };

                        if client_header.seq_num < expected_seq_number {
                            let body = ProtocolErrors::SequenceError {
                                expected: expected_seq_number,
                                received: client_header.seq_num,
                            };
                            if let Err(_) = self.send_response(MessageType::Error, &body).await {
                                break;
                            }
                            continue;
                        }

                        let mut body: Vec<u8> = vec![0u8; client_header.length.into()];
                        let _ = self.socket.read_exact(&mut body).await;

                        let message =
                            Message::decode(client_header.msg_type, &mut BytesMut::from(body.as_slice()))
                                .unwrap();

                        let event = utils::message_as_event(self.client.client_id, &message);
                        let _ = self.order_book_tx_sender.send(event).await;
                 }
                event = event_rx.recv() => {
                    let event = match event {
                        Some(event) => event,
                        None => break,
                    };

                    let (message_bytes, message_type) = utils::engine_event_as_bytes(&event);
                    let header = Header::new(0, message_bytes.len() as u16, 0, message_type as u8).as_bytes();
                    let _ = self.socket.write_all(&header).await;
                    let _ = self.socket.write_all(&message_bytes).await;
                    let _ = self.socket.flush().await;
                }
            }
        }
    }
}

