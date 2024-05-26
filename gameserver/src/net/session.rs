use anyhow::Result;
use qwer::{OctData, ProtocolHeader};
use std::io::Cursor;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, MutexGuard};

use crate::game::manager::net_stream;
use crate::game::GameContext;

use super::handlers::ProtocolHandler;
use super::{Packet, RequestBody, ResponseBody};

pub struct NetworkSession {
    client_socket: Arc<Mutex<TcpStream>>,
    cur_rpc_uid: u64,
    pub ns_prop_mgr: net_stream::PropertyManager,
    pub context: GameContext,
}

impl NetworkSession {
    pub fn new(client_socket: TcpStream) -> Self {
        let ns_prop_mgr = net_stream::PropertyManager::default();

        Self {
            client_socket: Arc::new(Mutex::new(client_socket)),
            cur_rpc_uid: 0,
            context: GameContext::new(ns_prop_mgr.player_info.clone()),
            ns_prop_mgr,
        }
    }

    async fn client_socket(&self) -> MutexGuard<'_, TcpStream> {
        self.client_socket.lock().await
    }

    pub async fn get_player_uid(&self) -> u64 {
        self.ns_prop_mgr.player_info.read().await.uid.unwrap()
    }

    pub async fn run(&mut self) -> Result<()> {
        let Some(_channel_id) = self.read_handshake().await? else {
            return Ok(());
        };

        loop {
            let packet = match Packet::read(&mut *self.client_socket().await).await {
                Ok(packet) => packet,
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(()),
                Err(e) => return Err(e.into()),
            };
            let request: RequestBody = packet.body.into();

            self.cur_rpc_uid = packet.header.rpc_arg_uid;
            Box::pin(Self::on_message(self, request.protocol_id, request.payload)).await?;
        }
    }

    async fn read_handshake(&mut self) -> Result<Option<u16>> {
        match self.client_socket().await.read_u16_le().await {
            Ok(channel_id) => Ok(Some(channel_id)),
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => Ok(None),
            Err(e) => return Err(e.into()),
        }
    }

    pub async fn send_rpc_arg(&self, protocol_id: u16, data: &impl OctData) -> Result<()> {
        let header: Vec<u8> = ProtocolHeader::default().into();

        let mut payload = Vec::new();
        let mut cursor = Cursor::new(&mut payload);
        data.marshal_to(&mut cursor, 0)?;

        let body: Vec<u8> = RequestBody {
            protocol_id,
            payload,
        }
        .into();

        let mut packet = Vec::new();
        packet.extend(0_u16.to_le_bytes());
        packet.extend(((body.len() + 2) as u32).to_le_bytes());
        packet.extend((header.len() as u16).to_le_bytes());
        packet.extend(header);
        packet.extend(body);
        packet.extend(0_u16.to_le_bytes()); // middleware count

        self.client_socket().await.write_all(&packet).await?;
        tracing::info!("Ptc with protocol id {protocol_id} sent");
        Ok(())
    }
}

impl ProtocolHandler for NetworkSession {
    async fn send_rpc_ret(&self, data: impl OctData) -> Result<()> {
        let header = ProtocolHeader {
            is_rpc_ret: true,
            rpc_arg_uid: self.cur_rpc_uid,
            ..Default::default()
        };

        let mut payload = Vec::new();
        let mut cursor = Cursor::new(&mut payload);
        data.marshal_to(&mut cursor, 0)?;

        let body: Vec<u8> = ResponseBody {
            middleware_id: 0,
            middleware_error_code: 0,
            payload,
        }
        .into();

        let header_buf: Vec<u8> = header.into();
        let mut packet = Vec::new();
        packet.extend(0_u16.to_le_bytes());
        packet.extend((body.len() as u32).to_le_bytes());
        packet.extend((header_buf.len() as u16).to_le_bytes());
        packet.extend(header_buf);
        packet.extend(body);

        self.client_socket().await.write_all(&packet).await?;
        Ok(())
    }
}
