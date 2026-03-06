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
};

async fn send_response(socket: &mut TcpStream, header: &[u8], body: &[u8]) {
    let mut response: Vec<u8> = Vec::new();

    let _ = response.write_all(&header).await;
    let _ = response.write_all(&body).await;

    let _ = socket.write_all(&response).await;
    let _ = socket.flush().await;
}

pub async fn handle_connection(socket: &mut TcpStream, tx: Sender<Event>) {
    loop {
        let mut client_header = [0u8; HEADER_SIZE];
        let n = socket.read(&mut client_header).await.unwrap();

        if n == 0 {
            return;
        }

        let client_header = match Header::from(&client_header) {
            Ok(header) => header,
            Err(err) => {
                let body = protocol::Message::Error(ProtocolErrors::HeaderError(err)).as_byte();
                let header =
                    Header::new(0, body.len() as u16, 0, MessageType::Error as u8).as_bytes();

                send_response(socket, &header, &body).await;

                continue;
            }
        };

        let mut body: Vec<u8> = vec![0u8; client_header.length.into()];
        let _ = socket.read_exact(&mut body).await;

        let message = Message::from(
            MessageType::try_from(client_header.msg_type).unwrap(),
            &body,
        );
    }
}
