#[derive(Debug, Default)]
pub struct ProtocolHeader {
    pub to_channel: u16,
    pub from_channel: u16,
    pub is_rpc_ret: bool,
    pub rpc_arg_uid: u64,
}

impl From<Vec<u8>> for ProtocolHeader {
    fn from(value: Vec<u8>) -> Self {
        let to_channel = u16::from_le_bytes(value[0..2].try_into().unwrap());
        let from_channel = u16::from_le_bytes(value[2..4].try_into().unwrap());
        let is_rpc_ret = value[4] != 100;
        let rpc_arg_uid = u64::from_le_bytes(value[5..13].try_into().unwrap());

        Self {
            to_channel,
            from_channel,
            is_rpc_ret,
            rpc_arg_uid,
        }
    }
}

impl From<ProtocolHeader> for Vec<u8> {
    fn from(value: ProtocolHeader) -> Self {
        let mut out = Self::with_capacity(13);

        out.extend(value.to_channel.to_le_bytes());
        out.extend(value.from_channel.to_le_bytes());
        out.push(if value.is_rpc_ret { 1 } else { 100 });
        out.extend(value.rpc_arg_uid.to_le_bytes());

        out
    }
}
