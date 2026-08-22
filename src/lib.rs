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

pub mod api;

pub use client::{ApiEnvelope, AsignClient};
pub use config::{AsignConfig, Environment};
pub use error::{AsignError, AsignResult};
pub use sign::{SignerKey, compute_sign, md5_hex, sort_json_keys};
