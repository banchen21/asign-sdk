//! 签名与工具函数测试。

use asign_sdk::{compute_sign, md5_hex, sort_json_keys, SignerKey};

/// 生成一个用于测试的 RSA 私钥（PKCS#8）。
#[test]
fn sort_json_keys_orders_alphabetically() {
    let sorted = sort_json_keys(r#"{"ba":1,"ac":2}"#).unwrap();
    assert_eq!(sorted, r#"{"ac":2,"ba":1}"#);
}

#[test]
fn md5_hex_is_lowercase() {
    // md5("hello") = 5d41402abc4b2a76b9719d911017c592
    assert_eq!(md5_hex("hello"), "5d41402abc4b2a76b9719d911017c592");
}

#[test]
fn compute_sign_produces_nonempty_base64() {
    use rsa::pkcs8::EncodePrivateKey;
    use rsa::RsaPrivateKey;

    let mut rng = rand::thread_rng();
    let key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
    let pem = key.to_pkcs8_pem(rsa::pkcs8::LineEnding::LF).unwrap();

    let signer = SignerKey::from_pem(pem.as_str()).unwrap();
    let sign = compute_sign(
        &signer,
        r#"{"contractNo":"0001"}"#,
        "app123",
        "1700000000000",
    )
    .unwrap();

    assert!(!sign.is_empty());
    assert!(!sign.contains('\n'));
}
