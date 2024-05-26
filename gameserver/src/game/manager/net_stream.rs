use protocol::{AccountInfo, PlayerInfo, PropertyBlob};
use qwer::OctData;
use std::sync::Arc;
use tokio::sync::RwLock;

const CLIENT_PROP_FLAG: u16 = 1;

#[derive(Default)]
pub struct PropertyManager {
    pub account_info: Arc<RwLock<AccountInfo>>,
    pub player_info: Arc<RwLock<PlayerInfo>>,
}

impl PropertyManager {
    pub async fn serialize_account_info(&self) -> PropertyBlob {
        Self::serialize_property(&*self.account_info.read().await).unwrap()
    }

    pub async fn serialize_player_info(&self) -> PropertyBlob {
        Self::serialize_property(&*self.player_info.read().await).unwrap()
    }

    pub fn serialize_property(prop: &impl OctData) -> Result<PropertyBlob, std::io::Error> {
        let mut stream = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut stream);

        prop.marshal_to(&mut cursor, CLIENT_PROP_FLAG)?;
        Ok(PropertyBlob { stream })
    }
}
