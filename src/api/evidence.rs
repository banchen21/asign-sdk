//! 存证接口。

use serde::{Deserialize, Serialize};

use crate::client::AsignClient;
use crate::AsignResult;

/// 存证请求。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceRequest {
    /// 关联的签署任务 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 业务方自定义编号。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_no: Option<String>,
}

/// 存证结果。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceResult {
    #[serde(default)]
    pub evidence_no: Option<String>,
}

impl AsignClient {
    /// 发起存证。
    pub async fn create_evidence(&self, req: &EvidenceRequest) -> AsignResult<EvidenceResult> {
        self.invoke("/evidence/create", req).await
    }
}
