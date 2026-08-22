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
#[derive(Debug, serde::Deserialize)]
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

        // 先解析统一包裹结构，判断业务是否成功。
        let envelope: ApiEnvelope = serde_json::from_str(&text).unwrap_or(ApiEnvelope {
            code: None,
            msg: None,
            message: None,
            data: None,
            request_id: None,
        });

        if !envelope.is_success() {
            return Err(AsignError::Api {
                code: envelope.code.map(|c| c.to_string()).unwrap_or_default(),
                message: envelope.message(),
            });
        }

        // 业务成功：优先从 data 字段反序列化目标类型；无 data 时尝试整体反序列化。
        if let Some(data) = envelope.data {
            match serde_json::from_value::<T>(data) {
                Ok(value) => return Ok(value),
                Err(_) => {}
            }
        }
        match serde_json::from_str::<T>(&text) {
            Ok(value) => Ok(value),
            Err(err) => Err(AsignError::Json(err)),
        }
    }
}
