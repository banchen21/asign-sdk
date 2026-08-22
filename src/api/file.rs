//! 文件接口（上传、查询状态、合并）。

use serde::{Deserialize, Serialize};

use crate::client::AsignClient;
use crate::AsignResult;

/// 文件上传请求。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFileRequest {
    /// 文件名。
    pub file_name: String,
    /// 文件内容（base64 或原始内容，以官方文档为准）。
    pub file_content: String,
}

/// 文件上传结果。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFileResult {
    #[serde(default)]
    pub file_id: Option<String>,
    #[serde(default)]
    pub file_md5: Option<String>,
}

impl AsignClient {
    /// 上传文件。
    pub async fn upload_file(&self, req: &UploadFileRequest) -> AsignResult<UploadFileResult> {
        self.invoke("/file/upload", req).await
    }
}
