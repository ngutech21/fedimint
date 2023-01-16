use fedimint_api::db::DatabaseKeyPrefixConst;
use fedimint_api::encoding::{Decodable, Encodable};
use serde::Serialize;
use strum_macros::EnumIter;

#[repr(u8)]
#[derive(Clone, EnumIter, Debug)]
pub enum DbKeyPrefix {
    Example = 0x80,
}

impl std::fmt::Display for DbKeyPrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Encodable, Decodable, Eq, PartialEq, Hash, Serialize)]
pub struct UUIDKey(pub String);

impl UUIDKey {
    pub fn new() -> Self {
        let my_uuid = uuid::Uuid::new_v4();
        let urn = my_uuid.hyphenated().to_string();
        Self(urn)
    }
}

impl Default for UUIDKey {
    fn default() -> Self {
        Self::new()
    }
}

impl From<String> for UUIDKey {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl DatabaseKeyPrefixConst for UUIDKey {
    const DB_PREFIX: u8 = DbKeyPrefix::Example as u8;
    type Key = Self;
    type Value = StringValue;
}

#[derive(Debug, Encodable, Decodable)]
pub struct ExampleKeyPrefix;

impl DatabaseKeyPrefixConst for ExampleKeyPrefix {
    const DB_PREFIX: u8 = DbKeyPrefix::Example as u8;
    type Key = UUIDKey;
    type Value = StringValue;
}

#[derive(Debug, Encodable, Decodable)]
pub struct StringValue(pub String);
