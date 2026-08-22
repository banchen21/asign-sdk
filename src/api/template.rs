//! 模板接口。

use serde::{Deserialize, Serialize};

use crate::client::AsignClient;
use crate::AsignResult;

/// 模板信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateInfo {
    #[serde(default)]
    pub template_id: Option<String>,
    #[serde(default)]
    pub template_name: Option<String>,
}

impl AsignClient {
    /// 查询模板列表。
    pub async fn query_template_list(&self) -> AsignResult<Vec<TemplateInfo>> {
        self.invoke("/template/list", &serde_json::json!({})).await
    }
}
