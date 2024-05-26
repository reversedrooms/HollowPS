use encoding::all::UTF_16LE;
use encoding::{EncoderTrap, Encoding};

pub fn encrypt_config(content: &str, key: &str) -> Vec<u8> {
    let mut content_bytes = UTF_16LE.encode(content, EncoderTrap::Strict).unwrap();
    let key_bytes = UTF_16LE.encode(key, EncoderTrap::Strict).unwrap();

    for i in 0..content_bytes.len() {
        content_bytes[i] ^= key_bytes[i % key_bytes.len()];
    }

    let k = key.as_bytes();

    let mut out = Vec::with_capacity(4 + k.len() + 8 + content_bytes.len());

    leb128::write::unsigned(&mut out, u64::try_from(k.len()).unwrap()).unwrap();
    out.extend_from_slice(k);
    out.extend([0u8; 4]);
    out.extend(u32::try_from(content_bytes.len()).unwrap().to_le_bytes());
    out.extend(content_bytes);

    out
}
