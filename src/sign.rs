//! 爱签签名与请求封装。
//!
//! 签名规范（爱签 OpenAPI）：
//! 1. 请求方式：`form-data`；
//! 2. 请求头参数：`sign`（签名）、`timestamp`（13 位毫秒时间戳）；
//! 3. 请求体参数：`appId`、`timestamp`（与头部一致）、`bizData`（JSON 字符串）；
//! 4. 签名算法：
//!    - 将 `bizData` 的 JSON 字段按 key 字母序排序；
//!    - `rsaSuffix = sortedJson + md5(sortedJson) + appId + timestamp`；
//!    - 使用 `SHA1withRSA`（RSA PKCS#1 v1.5 + SHA-1）签名；
//!    - base64 编码后去除换行，放入 header 的 `sign`。

use base64::Engine;
use md5::Digest as _;
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs8::{DecodePrivateKey, EncodePublicKey, LineEnding};
use rsa::{Pkcs1v15Sign, RsaPrivateKey};
use sha1::Sha1;

use crate::{AsignError, AsignResult};

/// RSA 私钥封装（同时支持 PKCS#1 与 PKCS#8 PEM）。
pub struct SignerKey {
    key: RsaPrivateKey,
}

impl SignerKey {
    /// 从 PEM 格式私钥字符串构建。
    pub fn from_pem(pem: &str) -> AsignResult<Self> {
        let trimmed = pem.trim();

        let key = if trimmed.contains("BEGIN PRIVATE KEY") {
            RsaPrivateKey::from_pkcs8_pem(trimmed)
                .map_err(|err| AsignError::InvalidPrivateKey(err.to_string()))?
        } else if trimmed.contains("BEGIN RSA PRIVATE KEY") {
            RsaPrivateKey::from_pkcs1_pem(trimmed)
                .map_err(|err| AsignError::InvalidPrivateKey(err.to_string()))?
        } else {
            return Err(AsignError::InvalidPrivateKey(
                "unrecognized private key format (expected PEM)".to_string(),
            ));
        };

        Ok(Self { key })
    }

    /// SHA1withRSA 签名，返回 base64（已去除换行）。
    ///
    /// 先对原始数据做 SHA-1 摘要，再用 PKCS#1 v1.5 私钥签名（等价于 Java 的
    /// `SHA1withRSA`）。
    pub fn sign_sha1_base64(&self, data: &[u8]) -> AsignResult<String> {
        use sha1::Digest as _;
        let digest = Sha1::digest(data);
        let signing_key = Pkcs1v15Sign::new::<Sha1>();
        let signature = self
            .key
            .sign(signing_key, &digest)
            .map_err(|err| AsignError::Sign(err.to_string()))?;
        let b64 = base64::engine::general_purpose::STANDARD.encode(signature);
        Ok(b64.replace("\r\n", "").replace('\n', ""))
    }

    /// 导出这把私钥配对的**公钥**，PEM 形态（PKCS#8 / SubjectPublicKeyInfo，`BEGIN PUBLIC KEY`）。
    ///
    /// 爱签开放平台的应用配置要填「应用公钥」，用来自验证我们发出去的请求签名，
    /// 填的就是这把公钥。
    pub fn public_key_pem(&self) -> AsignResult<String> {
        self.key
            .to_public_key()
            .to_public_key_pem(LineEnding::LF)
            .map_err(|err| AsignError::InvalidPublicKey(err.to_string()))
    }

    /// 导出这把私钥配对的公钥，**去掉头尾标记、换行与空格的一行 base64**。
    ///
    /// 这是爱签应用配置要求的填写口径（官方备注「去掉头尾与换行」）。
    pub fn public_key_body(&self) -> AsignResult<String> {
        Ok(crate::verify::public_key_body(&self.public_key_pem()?))
    }
}

/// MD5 hex 摘要（小写）。
pub fn md5_hex(data: &str) -> String {
    let digest = md5::Md5::digest(data.as_bytes());
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// 对 JSON 对象按 key 字母序排序后重新序列化。
///
/// 例：`{"ba":1,"ac":2}` → `{"ac":2,"ba":1}`
pub fn sort_json_keys(json: &str) -> AsignResult<String> {
    let value: serde_json::Value = serde_json::from_str(json).map_err(AsignError::Json)?;
    Ok(serde_json::to_string(&value)?)
}

/// 计算爱签鉴权签名。
///
/// - `biz_data_json`：`bizData` 的 JSON 字符串（调用方已序列化）。
/// - `app_id`：应用 ID。
/// - `timestamp`：13 位毫秒时间戳字符串。
pub fn compute_sign(
    key: &SignerKey,
    biz_data_json: &str,
    app_id: &str,
    timestamp: &str,
) -> AsignResult<String> {
    let sorted = sort_json_keys(biz_data_json)?;
    let suffix = format!("{}{}{}{}", sorted, md5_hex(&sorted), app_id, timestamp);
    key.sign_sha1_base64(suffix.as_bytes())
}

