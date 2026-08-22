//! 应用管理接口。

use serde::{Deserialize, Serialize};

use crate::client::AsignClient;
use crate::AsignResult;

/// 查询应用列表响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    #[serde(default)]
    pub app_id: Option<String>,
    #[serde(default)]
    pub app_name: Option<String>,
}

impl AsignClient {
    /// 查询应用列表。
    pub async fn query_app_list(&self) -> AsignResult<Vec<AppInfo>> {
        self.invoke("/app/getList", &serde_json::json!({})).await
    }
}
