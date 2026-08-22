//! 印章接口（企业印章 / 个人签名）。

use serde::{Deserialize, Serialize};

use crate::client::AsignClient;
use crate::AsignResult;

/// 印章信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SealInfo {
    #[serde(default)]
    pub seal_id: Option<String>,
    #[serde(default)]
    pub seal_name: Option<String>,
}

impl AsignClient {
    /// 制作企业印章（V2）。
    pub async fn make_company_seal(&self, req: &serde_json::Value) -> AsignResult<serde_json::Value> {
        self.invoke("/seal/makeCompanySeal", req).await
    }
}
