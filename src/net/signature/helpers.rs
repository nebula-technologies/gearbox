use alloc::vec::Vec;
use hmac::{Hmac, KeyInit, Mac};
use sha2::{Digest, Sha256, Sha512};

type HmacSha512 = Hmac<Sha512>;
type HmacSha256 = Hmac<Sha256>;

pub fn sha256(value: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(value);
    hasher.finalize().as_slice().to_vec()
}

pub fn sha512(value: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(value);
    hasher.finalize().as_slice().to_vec()
}

pub fn hmac_sha512(enc_key: &[u8], value: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha512::new_from_slice(enc_key).expect("HMAC can take key of any size");
    mac.update(value);
    mac.finalize().into_bytes().to_vec()
}

pub fn hmac_sha256(enc_key: &Vec<u8>, value: &Vec<u8>) -> Vec<u8> {
    let mut mac =
        HmacSha256::new_from_slice(enc_key.as_slice()).expect("HMAC can take key of any size");
    mac.update(value.as_slice());
    mac.finalize().into_bytes().to_vec()
}

pub fn base64decode(value: &Vec<u8>) -> Vec<u8> {
    base64::decode(value).unwrap_or(Vec::new())
}
pub fn base64encode(value: &Vec<u8>) -> Vec<u8> {
    base64::encode(value).as_bytes().to_vec()
}

pub fn base58decode(value: &Vec<u8>) -> Vec<u8> {
    bs58::decode(value).into_vec().unwrap_or(Vec::new())
}
pub fn base58encode(value: &Vec<u8>) -> Vec<u8> {
    bs58::encode(value).into_string().as_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() {
        let data = b"hello world";
        let expected = hex::encode(Sha256::digest(data));
        assert_eq!(hex::encode(sha256(&data.to_vec())), expected);
    }

    #[test]
    fn test_sha512() {
        let data = b"hello world";
        let expected = hex::encode(Sha512::digest(data));
        assert_eq!(hex::encode(sha512(&data.to_vec())), expected);
    }

    #[test]
    fn test_hmac_sha512() {
        let key = b"secret key";
        let data = b"hello world";
        let mut mac = HmacSha512::new_from_slice(key).unwrap();
        mac.update(data);
        let expected = mac.finalize().into_bytes();
        assert_eq!(hmac_sha512(key, data), expected.to_vec());
    }

    #[test]
    fn test_hmac_sha256() {
        let key = b"secret key";
        let data = b"hello world";
        let mut mac = HmacSha256::new_from_slice(key).unwrap();
        mac.update(data);
        let expected = mac.finalize().into_bytes();
        assert_eq!(
            hmac_sha256(&key.to_vec(), &data.to_vec()),
            expected.to_vec()
        );
    }

    #[test]
    fn test_base64_encode_decode() {
        let data = b"hello world";
        let encoded = base64encode(&data.to_vec());
        let decoded = base64decode(&encoded);
        assert_eq!(decoded, data.to_vec());
    }

    #[test]
    fn test_base58_encode_decode() {
        let data = b"hello world";
        let encoded = base58encode(&data.to_vec());
        let decoded = base58decode(&encoded);
        assert_eq!(decoded, data.to_vec());
    }
}
