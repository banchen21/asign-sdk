//! 错误类型定义。

use thiserror::Error;

pub type AsignResult<T> = Result<T, AsignError>;

#[derive(Debug, Error)]
pub enum AsignError {
    /// 配置缺失或非法（如私钥为空、AppId 为空）。
    #[error("config error: {0}")]
    Config(String),

    /// 私钥解析失败。
    #[error("invalid private key: {0}")]
    InvalidPrivateKey(String),

    /// 验签公钥解析失败（爱签渠道公钥未配置或格式不对）。
    #[error("invalid public key: {0}")]
    InvalidPublicKey(String),

    /// 回调签名校验失败（签名不是合法 base64，或长度/算法不符）。
    #[error("invalid signature: {0}")]
    InvalidSignature(String),

    /// 签名失败。
    #[error("sign error: {0}")]
    Sign(String),

    /// 网络请求失败。
    #[error("request error: {0}")]
    Request(#[from] reqwest::Error),

    /// JSON 序列化 / 反序列化失败。
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    /// 服务端返回业务错误码。
    #[error("asign api error (code={code}): {message}")]
    Api { code: String, message: String },
}
