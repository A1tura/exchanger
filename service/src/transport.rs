use bytes::{BufMut, BytesMut};
use engine::events::Event;
use order_book::order::OrderReq;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc::Sender,
};

use protocol::{
    Message, MessageType,
    errors::ProtocolErrors,
    header::{HEADER_SIZE, Header},
    traits::Encode,
};

use crate::session::Session;

async fn send_response<T: protocol::traits::Encode>(
    socket: &mut TcpStream,
    session: &mut Session,
    message_type: MessageType,
    res: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut response: Vec<u8> = Vec::new();

    let mut body = BytesMut::new();
    res.encode(&mut body);

    let header = Header::new(0, body.len() as u16, session.seq_num, message_type as u8).as_bytes();

    let _ = response.write_all(&header).await;
    let _ = response.write_all(&body).await;

    let _ = socket.write_all(&response).await?;
    let _ = socket.flush().await?;

    session.seq_num += 1;

    return Ok(());
}

pub async fn handle_connection(socket: &mut TcpStream, tx: Sender<Event>) {
    let mut session = Session::new();
    loop {
        let expected_seq_number = session.seq_num + 1;

        let mut client_header = [0u8; HEADER_SIZE];
        let n = socket.read(&mut client_header).await.unwrap();

        if n == 0 {
            return;
        }

        let client_header = match Header::from(&client_header) {
            Ok(header) => header,
            Err(err) => {
                let body = ProtocolErrors::HeaderError(err);

                if let Err(_) = send_response(socket, &mut session, MessageType::Error, &body).await
                {
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
            if let Err(_) = send_response(socket, &mut session, MessageType::Error, &body).await {
                break;
            }
            continue;
        }

        let mut body: Vec<u8> = vec![0u8; client_header.length.into()];
        let _ = socket.read_exact(&mut body).await;

        let message = Message::decode(client_header.msg_type, &mut BytesMut::from(body.as_slice()));
        println!("{:?}", message);
    }
}
