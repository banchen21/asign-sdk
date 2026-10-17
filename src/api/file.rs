//! 文件接口：官方「两步上传」（获取上传地址 → PUT 文件流）与 OCR 识别。
//!
//! 官方文档：
//! - 步骤1 获取文件上传地址 `POST /get/upload/url`
//! - 步骤2 上传文件流（PUT 到步骤1返回的 `uploadUrl`）
//! - OCR 营业执照识别 `POST /ocr/business`
//! - OCR 身份证识别 `POST /user/ocrIdentify`（人像面 / 国徽面，见 `ocr_identify`）

use serde::{Deserialize, Serialize};

use crate::client::AsignClient;
use crate::AsignResult;

/// 获取文件上传地址（官方步骤 1）。
///
/// 三个易错点（都踩过）：
/// - `contentMd5` 是**文件的 Content-MD5**：先取 MD5 的 128 位二进制，再做 Base64 —— 不是 hex；
/// - `filename` **不带扩展名**，扩展名单独放 `extension`（不带点）；
/// - 返回的 `fileId` 要留着，后续发起签署 / OCR 都用它引用这个文件。
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUploadUrlRequest {
    /// Content-MD5：MD5 128 位二进制的 Base64。
    pub content_md5: String,
    /// 文件名（**不含扩展名**）。
    pub filename: String,
    /// 扩展名（不带点）：doc/docx/pdf/xls/xlsx/png/jpg/jpeg/ofd。
    pub extension: String,
    /// 文件字节数。
    pub filesize: i64,
    /// 1 合同附件（不传默认）/ 2 印章图片 / 3 模板 / 4 待合并文件 /
    /// 5 实名认证同步文件 / 6 实名认证证件照。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<i32>,
}

/// 上传地址与文件 ID（步骤 1 的响应 `data`）。
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadTicket {
    pub file_id: String,
    /// 预签名上传地址，直接 PUT 文件字节即可（不需要签名头）。
    pub upload_url: String,
    #[serde(default)]
    pub expired_time: Option<i64>,
}

/// 营业执照 OCR 请求。
///
/// 渠道目前**只接受 `fileId`**：内联 base64 的 `image` 会被拒（实测 `100004 文件编号不能为空`
/// 与 `100109 参数错误`），所以必须先走步骤 1/2 把文件传上去。
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OcrBusinessRequest {
    pub file_id: String,
}

/// 身份证 OCR 请求（人像面 / 国徽面共用）。
///
/// 与营业执照 OCR 同口径：渠道**只接受 `fileId`**，必须先走步骤 1/2 把证件图传上去，
/// 内联 base64 的 `image` 会被拒（渠道回 `100004 文件编号不能为空`）。
/// 实测（2026-10-12，测试环境）请求体只要 `{"fileId": "..."}`，不需要 `side` 之类的正反面标记。
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OcrIdentifyRequest {
    pub file_id: String,
}

/// 计算官方要求的 `contentMd5`：MD5 128 位二进制 → Base64。
pub fn content_md5(bytes: &[u8]) -> String {
    use base64::Engine;
    use md5::Digest as _;

    let digest = md5::Md5::digest(bytes);
    base64::engine::general_purpose::STANDARD.encode(digest)
}

impl AsignClient {
    /// 步骤 1：获取上传地址与文件 ID。
    pub async fn get_upload_url(&self, req: &GetUploadUrlRequest) -> AsignResult<UploadTicket> {
        self.invoke("/get/upload/url", req).await
    }

    /// OCR 营业执照识别，返回渠道原始 `data`（字段以渠道文档为准，由调用方做白名单映射）。
    pub async fn ocr_business(&self, req: &OcrBusinessRequest) -> AsignResult<serde_json::Value> {
        self.invoke("/ocr/business", req).await
    }

    /// OCR 身份证识别（人像面 / 国徽面），返回渠道原始 `data`。
    ///
    /// 由调用方做白名单映射：渠道还回姓名、证件号、人像 base64 等敏感字段，
    /// **只允许取有效期起止**对外，与营业执照 OCR 同一口径
    /// （见 `backend_73info` 的 `ocr_business_license`）。
    ///
    /// 实测（2026-10-12，测试环境；人像面与国徽面各跑一次）：
    /// - 请求体只要 `{"fileId": "..."}`，**不需要 `side`**——渠道按图自动判正反面，
    ///   返回体里的 `side` 是 `front` / `back`；
    /// - 有效期在返回体 `data` 里叫 `begin`（起始）与 `end`（截止），
    ///   已是 `YYYYMMDD` 无分隔（例 `20210903` / `20310903`）；长期证件以渠道返回为准。
    ///
    /// 人像面不含有效期时 `begin` / `end` 为 `null`（有效期印在国徽面）。
    pub async fn ocr_identify(&self, req: &OcrIdentifyRequest) -> AsignResult<serde_json::Value> {
        self.invoke("/user/ocrIdentify", req).await
    }
}
