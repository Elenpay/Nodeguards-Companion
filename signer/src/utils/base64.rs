use base64::engine::general_purpose;
use base64::{DecodeError, Engine};

pub fn to_base64<T: AsRef<[u8]>>(data: &T) -> String {
    general_purpose::STANDARD.encode(data)
}

pub fn from_base64<T: AsRef<[u8]>>(data: T) -> Result<Vec<u8>, DecodeError> {
    general_purpose::STANDARD.decode(data)
}
