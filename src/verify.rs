//! 爱签回调验签：用**爱签公钥**校验渠道回调里的 `sign`。
//!
//! 规范（见 `.cursor/skills/asign/references/protocol.md` 的「验签」段）：
//! `sign = RSAUtils.generateSHA1withRSASignature(除 sign 及不参与签名的 account / remark 外、
//! 按键字母序拼接的 JSON 字符串)` —— 即 **SHA1withRSA**（RSA PKCS#1 v1.5 + SHA-1）。
//!
//! 注意方向：调用爱签接口时用**我们自己的私钥**签名（[`crate::SignerKey`]）；
//! 校验爱签回调时用的是**爱签的公钥**（本模块），两者不是同一把钥匙。
//!
//! 爱签控制台给出的公钥常被**去掉头尾与换行**（官方备注如此），所以这里先把正文归一化成 PEM 再解析。

use base64::Engine;
use rsa::pkcs1::DecodeRsaPublicKey;
use rsa::pkcs8::DecodePublicKey;
use rsa::{Pkcs1v15Sign, RsaPublicKey};
use sha1::Sha1;
use crate::{AsignError, AsignResult};

/// 把「只有 base64 正文 / 带 PEM 头尾 / 带换行与空格 / 头尾与正文挤在一行」的公钥统统归一化成标准 PEM。
///
/// 返回 `None` 表示内容为空。注意：这里**只做格式归一化**，能不能解析由 [`VerifierKey::from_pem`] 决定。
pub fn normalize_public_key_pem(raw: &str) -> Option<String> {
    // PKCS#1（`RSA PUBLIC KEY`）与 PKCS#8（`PUBLIC KEY`）头尾不同，先记住原始写法。
    let wants_pkcs1 = raw.contains("RSA PUBLIC KEY");

    // 去掉所有 `-----XXX-----` 标记（可能跨行、也可能与正文挤在同一行），只留 base64 正文。
    let body: String = strip_pem_markers(raw)
        .chars()
        .filter(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '+' | '/' | '=')
        })
        .collect();
    if body.is_empty() {
        return None;
    }

    let wrapped = body
        .as_bytes()
        .chunks(64)
        .map(|chunk| String::from_utf8_lossy(chunk).to_string())
        .collect::<Vec<_>>()
        .join("\n");
    let (begin, end) = if wants_pkcs1 {
        ("-----BEGIN RSA PUBLIC KEY-----", "-----END RSA PUBLIC KEY-----")
    } else {
        ("-----BEGIN PUBLIC KEY-----", "-----END PUBLIC KEY-----")
    };
    Some(format!("{begin}\n{wrapped}\n{end}"))
}

/// 把公钥 PEM 压成**一行纯 base64 正文**：去掉 `-----…-----` 头尾与所有换行、空格。
///
/// 爱签开放平台的应用配置就要求这种口径（官方备注「去掉头尾与换行」），
/// 与 [`normalize_public_key_pem`] 正好互为逆行——一个把正文还原成 PEM，一个把 PEM 压成正文。
pub fn public_key_body(pem: &str) -> String {
    strip_pem_markers(pem)
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect()
}

/// 删掉 `-----…-----` 片段（头尾标记），返回其余内容。
pub(crate) fn strip_pem_markers(input: &str) -> String {
    let mut output = String::new();
    let mut rest = input;
    while let Some(start) = rest.find("-----") {
        output.push_str(&rest[..start]);
        let after = &rest[start + 5..];
        match after.find("-----") {
            Some(end) => rest = &after[end + 5..],
            None => {
                rest = "";
                break;
            }
        }
    }
    output.push_str(rest);
    output
}

/// 爱签公钥封装（同时支持 PKCS#8 的 `PUBLIC KEY` 与 PKCS#1 的 `RSA PUBLIC KEY`）。
pub struct VerifierKey {
    key: RsaPublicKey,
}

impl VerifierKey {
    /// 从 PEM（或只有正文的字符串）构建验签公钥。
    pub fn from_pem(pem: &str) -> AsignResult<Self> {
        let normalized = normalize_public_key_pem(pem)
            .ok_or_else(|| AsignError::InvalidPublicKey("public key is empty".to_string()))?;

        let key = if normalized.contains("BEGIN RSA PUBLIC KEY") {
            RsaPublicKey::from_pkcs1_pem(&normalized)
                .map_err(|err| AsignError::InvalidPublicKey(err.to_string()))?
        } else {
            // 无头尾正文按 PKCS#8（SubjectPublicKeyInfo）加了头；也允许调用方直接传 PKCS#1 的 PEM。
            match RsaPublicKey::from_public_key_pem(&normalized) {
                Ok(key) => key,
                Err(pkcs8_error) => RsaPublicKey::from_pkcs1_pem(&normalized).map_err(|_| {
                    AsignError::InvalidPublicKey(pkcs8_error.to_string())
                })?,
            }
        };

        Ok(Self { key })
    }

    /// 校验 `sign`（base64）是否匹配 `message`（已按规范拼好的待签名串）。
    ///
    /// `Ok(false)` = 签名不匹配（伪造或改包）；`Err` = 公钥/签名格式本身有问题。
    pub fn verify(&self, message: &str, signature_base64: &str) -> AsignResult<bool> {
        use sha1::Digest as _;

        let signature = base64::engine::general_purpose::STANDARD
            .decode(signature_base64.trim())
            .or_else(|_| base64::engine::general_purpose::STANDARD_NO_PAD.decode(signature_base64.trim()))
            .or_else(|_| {
                base64::engine::general_purpose::URL_SAFE
                    .decode(signature_base64.trim())
            })
            .map_err(|err| AsignError::InvalidSignature(err.to_string()))?;

        // 与 `SignerKey::sign_sha1_base64` 对称：rsa 0.9 的 `verify` 收的是**摘要**而不是原文。
        let digest = Sha1::digest(message.as_bytes());
        let scheme = Pkcs1v15Sign::new::<Sha1>();
        match self.key.verify(scheme, &digest, &signature) {
            Ok(()) => Ok(true),
            Err(rsa::Error::Verification) => Ok(false),
            Err(err) => Err(AsignError::InvalidSignature(err.to_string())),
        }
    }
}

/// 把回调参数拼成待验签的字符串：**除 `sign` / `account` / `remark` 外按键字母序**。
///
/// 调用方传进来的 `params` 应当**包含** `sign` 等字段（函数内部剔除），这样调用点不用自己记规则。
pub fn canonical_signable_payload(params: &std::collections::BTreeMap<String, String>) -> String {
    let mut sorted = std::collections::BTreeMap::new();
    for (name, value) in params {
        match name.as_str() {
            "sign" | "signV2" | "sign_v2" | "account" | "remark" => continue,
            _ => {
                sorted.insert(name.clone(), value.clone());
            }
        }
    }
    serde_json::to_string(&sorted).unwrap_or_default()
}
