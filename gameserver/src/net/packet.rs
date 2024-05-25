use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use qwer::ProtocolHeader;

pub struct Packet {
    #[allow(unused)]
    pub to_channel: u16,
    pub header: ProtocolHeader,
    pub body: Vec<u8>,
}

pub struct RequestBody {
    pub protocol_id: u16,
    pub payload: Vec<u8>,
}

pub struct ResponseBody {
    pub middleware_id: u16,
    pub middleware_error_code: u16,
    pub payload: Vec<u8>,
}

impl Packet {
    pub async fn read(stream: &mut TcpStream) -> std::io::Result<Self> {
        let to_channel = stream.read_u16_le().await?;
        let body_size = stream.read_u32_le().await? as usize;
        let header_size = stream.read_u16_le().await? as usize;

        let mut header = vec![0; header_size];
        stream.read_exact(&mut header).await?;

        let mut body = vec![0; body_size];
        stream.read_exact(&mut body).await?;

        Ok(Self {
            to_channel,
            header: header.into(),
            body,
        })
    }
}

impl From<Vec<u8>> for RequestBody {
    fn from(value: Vec<u8>) -> Self {
        let protocol_id = u16::from_le_bytes(value[0..2].try_into().unwrap());
        let payload_length = u32::from_be_bytes(value[2..6].try_into().unwrap()) as usize;
        let payload = value[6..payload_length + 6].to_vec();

        Self {
            protocol_id,
            payload,
        }
    }
}

impl From<RequestBody> for Vec<u8> {
    fn from(value: RequestBody) -> Self {
        let mut out = Self::new();

        out.extend(value.protocol_id.to_le_bytes());
        out.extend((value.payload.len() as u32).to_be_bytes());
        out.extend(value.payload);

        out
    }
}

impl From<ResponseBody> for Vec<u8> {
    fn from(value: ResponseBody) -> Self {
        let mut out = Self::with_capacity(4 + value.payload.len());
        out.extend(value.middleware_id.to_le_bytes());
        out.extend(value.middleware_error_code.to_le_bytes());
        out.extend(value.payload);

        out
    }
}
