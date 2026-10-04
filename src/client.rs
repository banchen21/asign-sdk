//! HTTP 客户端核心：爱签 form-data 请求封装与鉴权签名。

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::sign::{SignerKey, compute_sign};
use crate::{AsignConfig, AsignError, AsignResult};

/// 爱签统一响应包裹结构。
///
/// 真实响应形如：
/// ```json
/// {"code":100025,"data":null,"isp":null,"msg":"[test]用户不存在","requestId":null}
/// ```
/// 业务成功时 `code` 通常为 `1000` 或 `200`（以官方文档为准），`data` 为业务数据。
#[derive(Debug, Default, serde::Deserialize)]
pub struct ApiEnvelope {
    #[serde(default)]
    pub code: Option<i64>,
    #[serde(default)]
    pub msg: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
    #[serde(default)]
    pub request_id: Option<String>,
}

impl ApiEnvelope {
    /// 是否为业务成功。
    ///
    /// 爱签统一响应中，`code = 100000` 表示成功；
    /// `code` 缺失视为成功；其余非零 code 视为业务错误（如 100025 用户不存在）。
    pub fn is_success(&self) -> bool {
        match self.code {
            None => true,
            Some(code) => code == 100000,
        }
    }

    /// 获取错误信息。
    pub fn message(&self) -> String {
        self.msg
            .clone()
            .or_else(|| self.message.clone())
            .unwrap_or_default()
    }
}

/// 爱签客户端。
pub struct AsignClient {
    config: AsignConfig,
    signer: SignerKey,
    http: reqwest::Client,
}

impl AsignClient {
    /// 创建客户端并校验配置。
    pub fn new(config: AsignConfig) -> AsignResult<Self> {
        config.validate()?;
        let signer = SignerKey::from_pem(&config.private_key)?;
        let http = reqwest::Client::builder()
            .build()
            .map_err(AsignError::Request)?;
        Ok(Self {
            config,
            signer,
            http,
        })
    }

    /// 暴露配置（AppId / 环境）。
    pub fn config(&self) -> &AsignConfig {
        &self.config
    }

    /// 拼接完整接口 URL。
    pub fn url(&self, path: &str) -> String {
        let base = self.config.environment.base_url().trim_end_matches('/');
        let path = path.trim_start_matches('/');
        format!("{base}/{path}")
    }

    /// 生成 13 位毫秒时间戳（当前时间 + 10 分钟，与官方 Java demo 一致）。
    fn timestamp(&self) -> String {
        let now = chrono::Utc::now() + chrono::Duration::minutes(10);
        now.timestamp_millis().to_string()
    }

    /// 调用爱签接口。
    ///
    /// 按官方规范：
    /// - `form-data` 提交；
    /// - header：`sign`、`timestamp`；
    /// - body：`appId`、`timestamp`、`bizData`（JSON 字符串）。
    pub async fn invoke<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        biz_data: &B,
    ) -> AsignResult<T> {
        let biz_json = serde_json::to_string(biz_data)?;
        let timestamp = self.timestamp();
        let sign = compute_sign(&self.signer, &biz_json, &self.config.app_id, &timestamp)?;

        let url = self.url(path);
        let form = reqwest::multipart::Form::new()
            .text("appId", self.config.app_id.clone())
            .text("timestamp", timestamp.clone())
            .text("bizData", biz_json);

        let resp = self
            .http
            .post(&url)
            .header("sign", sign)
            .header("timestamp", timestamp)
            .multipart(form)
            .send()
            .await
            .map_err(AsignError::Request)?;

        self.parse_response(resp).await
    }

    /// 官方「步骤 2」：把文件字节 PUT 到步骤 1 返回的 `uploadUrl`。
    ///
    /// 该地址是文件系统的**预签名地址**，不带爱签的 `sign`/`timestamp` 头，
    /// 也不能走 `invoke`（那条路是 form-data + 签名）。
    ///
    /// 两个必须遵守的点（都实测踩过）：
    /// - **必须带 `Content-MD5`**（就是步骤 1 那个 Content-MD5）：OSS 的签名串含这一行，
    ///   爱签签名时把它算进去了，不带就 `SignatureDoesNotMatch`；
    /// - **不能带 `Content-Type`**：签名串里该行为空，补上 `image/jpeg` 同样会签名不符。
    pub async fn put_upload_stream(
        &self,
        upload_url: &str,
        content_md5: &str,
        bytes: Vec<u8>,
    ) -> AsignResult<()> {
        let resp = self
            .http
            .put(upload_url)
            .header("Content-MD5", content_md5)
            .body(bytes)
            .send()
            .await
            .map_err(AsignError::Request)?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(AsignError::Api {
                code: status.as_u16().to_string(),
                message: format!("上传文件流失败：{text}"),
            });
        }
        Ok(())
    }

    /// 调用爱签接口并附带 multipart 文件上传。
    ///
    /// 部分接口（如 `/contract/createContract`）需要在 multipart 中附带
    /// 真实文件（part 名为 `contractFiles`），同时在 `bizData` JSON 中以
    /// `{"fileUrl": "<文件名>"}` 引用文件。
    pub async fn invoke_with_files<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        biz_data: &B,
        file_part_name: &str,
        files: &[(String, Vec<u8>, String)], // (part 名, 文件字节, 文件名)
    ) -> AsignResult<T> {
        let biz_json = serde_json::to_string(biz_data)?;
        let timestamp = self.timestamp();
        let sign = compute_sign(&self.signer, &biz_json, &self.config.app_id, &timestamp)?;

        let url = self.url(path);
        let mut form = reqwest::multipart::Form::new()
            .text("appId", self.config.app_id.clone())
            .text("timestamp", timestamp.clone())
            .text("bizData", biz_json);

        for (name, bytes, file_name) in files {
            let part = reqwest::multipart::Part::bytes(bytes.clone())
                .file_name(file_name.clone())
                .mime_str("application/pdf")
                .map_err(AsignError::Request)?;
            form = form.part(name.clone(), part);
        }
        // 兼容：若 file_part_name 与 files 里的名字不重合，仍附加一次占位
        if !files.iter().any(|(name, _, _)| name == file_part_name) {
            form = form.part(
                file_part_name.to_string(),
                reqwest::multipart::Part::bytes(Vec::new()).file_name("empty.pdf"),
            );
        }

        let resp = self
            .http
            .post(&url)
            .header("sign", sign)
            .header("timestamp", timestamp)
            .multipart(form)
            .send()
            .await
            .map_err(AsignError::Request)?;

        self.parse_response(resp).await
    }

    async fn parse_response<T: DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> AsignResult<T> {
        let status = resp.status();
        let text = resp.text().await.map_err(AsignError::Request)?;

        if !status.is_success() {
            return Err(AsignError::Api {
                code: status.as_u16().to_string(),
                message: text,
            });
        }

        body_to_value(&text)
    }
}

/// 把爱签响应体解码成目标类型。
///
/// 单独抽成自由函数是为了能脱离 HTTP 直接单测：这里的顺序对「成功但 `data` 为空」
/// 的接口很关键（如 `/v2/user/reAuthUser` 成功时返回 `data: null`）。
fn body_to_value<T: DeserializeOwned>(text: &str) -> AsignResult<T> {
    // 先解析统一包裹结构，判断业务是否成功。
    let envelope: ApiEnvelope = serde_json::from_str(text).unwrap_or_default();

    if !envelope.is_success() {
        return Err(AsignError::Api {
            code: envelope.code.map(|c| c.to_string()).unwrap_or_default(),
            message: envelope.message(),
        });
    }

    // 业务成功且 `data` 有内容：优先从 `data` 字段反序列化目标类型。
    if let Some(data) = envelope.data.as_ref() {
        if let Ok(value) = serde_json::from_value::<T>(data.clone()) {
            return Ok(value);
        }
    }

    // 整体反序列化：部分接口（如 identifyH5）直接把业务数据当响应体返回。
    let envelope_err = match serde_json::from_str::<T>(text) {
        Ok(value) => return Ok(value),
        Err(err) => err,
    };

    // 成功包裹但 `data` 为空（缺失 / `null` / 空对象 / 空数组）：目标类型多为
    // `Option<String>` / `()` 这类能接受空值的类型，按 `null` 再解一次。
    // 否则会把「调用成功、没有返回值」误报成 JSON 解析失败——
    // 用户看到的是与本次操作无关的 `invalid type: map, expected a string`
    // （把整个包裹当成字符串去解，`reAuthUser` / `modifyUserName` / `modifyCompanyInfo`
    // 成功时都会走到这里）。
    if data_is_empty(&envelope.data) {
        if let Ok(value) = serde_json::from_value::<T>(serde_json::Value::Null) {
            return Ok(value);
        }
    }

    Err(AsignError::Json(envelope_err))
}

/// `data` 是否等于「没有返回值」：缺失、`null`、空对象、空数组。
///
/// 只看「空」而不看「有字段的对象」，是为了不把真正的结构不匹配（例如
/// `data` 是 `{"url": "..."}` 而目标类型是字符串）静默吞成成功。
fn data_is_empty(data: &Option<serde_json::Value>) -> bool {
    match data {
        None => true,
        Some(serde_json::Value::Null) => true,
        Some(serde_json::Value::Object(map)) => map.is_empty(),
        Some(serde_json::Value::Array(items)) => items.is_empty(),
        Some(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 从 `data` 取业务数据的常规成功响应。
    #[test]
    fn decodes_data_field() {
        let text = r#"{"code":100000,"data":{"codeToken":"tk-1"},"msg":"成功"}"#;
        let value: serde_json::Value = body_to_value(text).expect("decode");
        assert_eq!(value["codeToken"], "tk-1");
    }

    /// 成功包裹但 `data` 为 `null`：目标类型是 `Option<String>` 时必须得到 `None`，
    /// 而不是把整个包裹当成字符串去解（`reAuthUser`/`modifyUserName` 走的就是这条路）。
    #[test]
    fn decodes_null_data_as_none() {
        let text = r#"{"code":100000,"data":null,"isp":null,"msg":"成功","requestId":null}"#;
        let value: Option<String> = body_to_value(text).expect("decode");
        assert_eq!(value, None);
    }

    /// 成功包裹但 `data` 是空对象 / 缺失时同样不该报错。
    #[test]
    fn decodes_empty_data_as_none() {
        let empty_object = r#"{"code":100000,"data":{},"msg":"成功"}"#;
        let value: Option<String> = body_to_value(empty_object).expect("decode");
        assert_eq!(value, None);

        let no_data_field = r#"{"code":100000,"msg":"成功"}"#;
        let value: Option<String> = body_to_value(no_data_field).expect("decode");
        assert_eq!(value, None);
    }

    /// 成功包裹但 `data` 为 `null`、目标类型是 `()` 时同样不该报错。
    #[test]
    fn decodes_null_data_as_unit() {
        let text = r#"{"code":100000,"data":null,"msg":"成功"}"#;
        body_to_value::<()>(text).expect("decode");
    }

    /// 部分接口直接把业务数据当响应体（如 identifyH5 返回一个 JSON 字符串）。
    #[test]
    fn decodes_bare_payload() {
        let text = r#""https://pre.asign.cn/m/#/mobile/apiIdentify/PA123""#;
        let value: String = body_to_value(text).expect("decode");
        assert!(value.starts_with("https://pre.asign.cn/"));
    }

    /// 业务失败码要原样带出来，不能被当成解析错误。
    #[test]
    fn surfaces_business_error() {
        let text = r#"{"code":100025,"data":null,"msg":"[test]用户不存在"}"#;
        match body_to_value::<Option<String>>(text) {
            Err(AsignError::Api { code, message }) => {
                assert_eq!(code, "100025");
                assert_eq!(message, "[test]用户不存在");
            }
            other => panic!("expected api error, got {other:?}"),
        }
    }

    /// `data` 里有对象、目标类型却是字符串时仍然是解析错误（不静默吞掉）。
    #[test]
    fn reports_json_error_on_type_mismatch() {
        let text = r#"{"code":100000,"data":{"url":"https://x"},"msg":"成功"}"#;
        match body_to_value::<Option<String>>(text) {
            Err(AsignError::Json(_)) => {}
            other => panic!("expected json error, got {other:?}"),
        }
    }
}
