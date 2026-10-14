//! # asign-sdk
//!
//! 爱签电子合同 OpenAPI 的 Rust 客户端 SDK。
//!
//! 核心能力：
//! - 配置（AppId / 私钥 / 环境）
//! - RSA 私钥签名与请求封装
//! - 各业务接口分组（应用管理、实名认证、用户、文件、签署、模板、存证、回调）
//!
//! ```no_run
//! use asign_sdk::{AsignClient, AsignConfig, Environment};
//!
//! # fn main() {
//! let config = AsignConfig::new(
//!     Environment::Test,
//!     "your-app-id".to_string(),
//!     "your-private-key".to_string(),
//! );
//! let client = AsignClient::new(config);
//! # }
//! ```

mod client;
mod config;
mod error;
mod sign;
mod verify;

pub mod api;

pub use client::{ApiEnvelope, AsignClient};
pub use config::{AsignConfig, Environment};
pub use error::{AsignError, AsignResult};
pub use sign::{SignerKey, compute_sign, md5_hex, sort_json_keys};
pub use verify::{VerifierKey, canonical_signable_payload, normalize_public_key_pem, public_key_body};

#[cfg(test)]
mod verify_tests {
    use super::{SignerKey, VerifierKey, canonical_signable_payload, public_key_body};
    use rsa::pkcs1::EncodeRsaPrivateKey;
    use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding};
    use rsa::RsaPrivateKey;
    use std::collections::BTreeMap;

    fn test_keypair() -> (String, String) {
        let mut rng = rand::thread_rng();
        let private = RsaPrivateKey::new(&mut rng, 2048).expect("生成测试密钥失败");
        let private_pem = private.to_pkcs8_pem(LineEnding::LF).expect("私钥导出失败").to_string();
        let public_pem = private
            .to_public_key()
            .to_public_key_pem(LineEnding::LF)
            .expect("公钥导出失败");
        (private_pem, public_pem)
    }

    /// 正例：我们自己的私钥签名（与爱签同算法）→ 爱签公钥验签通过；改包则失败。
    #[test]
    fn verifies_sha1withrsa_signature_and_rejects_tampering() {
        let (private_pem, public_pem) = test_keypair();
        let signer = SignerKey::from_pem(&private_pem).expect("私钥解析失败");
        let verifier = VerifierKey::from_pem(&public_pem).expect("公钥解析失败");

        let mut params = BTreeMap::new();
        params.insert("contractNo".to_string(), "C-1".to_string());
        params.insert("status".to_string(), "3".to_string());
        params.insert("validityTime".to_string(), "2026-12-31".to_string());
        params.insert("sign".to_string(), "占位".to_string());
        params.insert("account".to_string(), "不参与签名".to_string());
        params.insert("remark".to_string(), "不参与签名".to_string());

        let payload = canonical_signable_payload(&params);
        assert!(payload.contains("contractNo") && !payload.contains("account") && !payload.contains("remark") && !payload.contains("sign"));
        let signature = signer.sign_sha1_base64(payload.as_bytes()).expect("签名失败");

        assert!(verifier.verify(&payload, &signature).expect("验签异常"));
        // 改一个字节就该失败（伪造 / 改包）
        assert!(!verifier.verify(&format!("{payload} "), &signature).expect("验签异常"));
    }

    /// 公钥归一化：爱签控制台给的是「去掉头尾与换行」的正文。
    #[test]
    fn normalizes_headerless_public_key() {
        let (_, public_pem) = test_keypair();
        let body: String = public_pem
            .lines()
            .filter(|line| !line.starts_with("-----"))
            .collect::<Vec<_>>()
            .join("");
        let verifier = VerifierKey::from_pem(&body).expect("无头尾公钥应当可解析");
        let signer_body = body.clone();
        assert!(!signer_body.is_empty());
        // 能解析就够了；这里再确认一下带空格的形态也能处理。
        let with_spaces = body
            .as_bytes()
            .chunks(37)
            .map(|chunk| String::from_utf8_lossy(chunk).to_string())
            .collect::<Vec<_>>()
            .join(" \n ");
        VerifierKey::from_pem(&with_spaces).expect("带空格换行的公钥应当可解析");
        drop(verifier);
    }

    /// 私钥导出公钥：PEM 是 PKCS#8（`BEGIN PUBLIC KEY`），正文是一行、无头尾、无换行空格，
    /// 且能原样解析回来（与 `normalize_public_key_pem` 互为逆行）。
    #[test]
    fn exports_pkcs8_public_key_body() {
        let (private_pem, expected_public_pem) = test_keypair();
        let signer = SignerKey::from_pem(&private_pem).expect("私钥解析失败");

        let pem = signer.public_key_pem().expect("导出公钥失败");
        assert!(pem.starts_with("-----BEGIN PUBLIC KEY-----"), "应当是 PKCS#8 / SubjectPublicKeyInfo：{pem}");
        // 与用 rsa 直接导出的公钥一致（同一把钥匙）。
        let expected_body: String = expected_public_pem
            .lines()
            .filter(|line| !line.starts_with("-----"))
            .collect();
        assert_eq!(public_key_body(&pem), expected_body);

        let body = signer.public_key_body().expect("导出公钥正文失败");
        assert_eq!(body, expected_body);
        assert!(!body.contains("-----"), "不应残留头尾标记");
        assert!(!body.contains('\n') && !body.contains('\r') && !body.contains(' '), "不应残留换行与空格");
        // 去掉头尾的正文仍可解析（爱签就是按这个口径收的）。
        VerifierKey::from_pem(&body).expect("无头尾公钥应当可解析");
    }
}
