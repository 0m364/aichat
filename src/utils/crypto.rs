use base64::{engine::general_purpose::STANDARD, Engine};
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

pub fn sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    format!("{:x}", hasher.finalize())
}

pub fn hmac_sha256(key: &[u8], msg: &str) -> Vec<u8> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(msg.as_bytes());
    mac.finalize().into_bytes().to_vec()
}

pub fn hex_encode(bytes: &[u8]) -> String {
    bytes
        .iter()
        .fold(String::new(), |acc, b| acc + &format!("{b:02x}"))
}

pub fn encode_uri(uri: &str) -> String {
    uri.split('/')
        .map(|v| urlencoding::encode(v))
        .collect::<Vec<_>>()
        .join("/")
}

pub fn base64_encode<T: AsRef<[u8]>>(input: T) -> String {
    STANDARD.encode(input)
}
pub fn base64_decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, base64::DecodeError> {
    STANDARD.decode(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() {
        assert_eq!(
            sha256("hello"),
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
        assert_eq!(
            sha256(""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_hmac_sha256() {
        let key = b"key";
        let msg = "message";
        let result = hmac_sha256(key, msg);
        assert_eq!(
            hex_encode(&result),
            "6e9ef29b75fffc7998931995dca9715c21f003b305c4a769968244ad55307e62"
        );
    }

    #[test]
    fn test_hex_encode() {
        assert_eq!(hex_encode(b"hello"), "68656c6c6f");
        assert_eq!(hex_encode(&[0x01, 0x02, 0x0a, 0xff]), "01020aff");
        assert_eq!(hex_encode(b""), "");
    }

    #[test]
    fn test_encode_uri() {
        assert_eq!(encode_uri("a/b c/d"), "a/b%20c/d");
        assert_eq!(encode_uri(""), "");
        assert_eq!(encode_uri("foo/bar"), "foo/bar");
    }

    #[test]
    fn test_base64_encode() {
        assert_eq!(base64_encode("hello"), "aGVsbG8=");
        assert_eq!(base64_encode(""), "");
        assert_eq!(base64_encode(&[0x00, 0x01, 0x02]), "AAEC");
    }

    #[test]
    fn test_base64_decode() {
        assert_eq!(base64_decode("aGVsbG8=").unwrap(), b"hello");
        assert_eq!(base64_decode("").unwrap(), b"");
        assert_eq!(base64_decode("AAEC").unwrap(), &[0x00, 0x01, 0x02]);
        assert!(base64_decode("invalid base64!").is_err());
    }
}
