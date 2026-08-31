//! 签署接口（签署任务）。
//!
//! 官方接入流程（陌生用户）：
//! 1. `addStranger` 添加陌生用户；
//! 2. `createContract` 上传待签署文件；
//! 3. `addSigner` 添加签署方（自动返回 signUrl 签署链接，签署页自动引导实名）；
//! 4. 打开签署链接完成签署；
//! 5. `downloadContract` 下载文件。

use serde::{Deserialize, Serialize};

use crate::client::AsignClient;
use crate::AsignResult;

/// 上传待签署文件（createContract）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContractRequest {
    /// 业务方合同编号（唯一）。
    #[serde(rename = "contractNo")]
    pub contract_no: String,
    /// 合同名称。
    #[serde(rename = "contractName")]
    pub contract_name: String,
    /// 有效时长（小时，可选）。
    #[serde(rename = "validityTime", skip_serializing_if = "Option::is_none")]
    pub validity_time: Option<u64>,
    /// 签署顺序：1=顺序签，2=无序签（可选）。
    #[serde(rename = "signOrder", skip_serializing_if = "Option::is_none")]
    pub sign_order: Option<i32>,
    /// 合同文件列表（引用文件：`fileUrl` 传文件名，文件通过 multipart 的 `contractFiles` part 上传）。
    #[serde(rename = "contractFiles")]
    pub contract_files: Vec<ContractFile>,
    /// 通知地址（可选）。
    #[serde(rename = "notifyUrl", skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
    /// 回调地址（可选）。
    #[serde(rename = "callbackUrl", skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
}

/// 合同文件。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractFile {
    /// 文件名称（multipart 中上传文件的文件名）。
    #[serde(rename = "fileUrl", skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    /// 文件 base64 内容（部分场景支持，见官方文档）。
    #[serde(rename = "fileContent", skip_serializing_if = "Option::is_none")]
    pub file_content: Option<String>,
}

/// createContract 结果。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractResult {
    #[serde(default)]
    pub contract_no: Option<String>,
    #[serde(default)]
    pub contract_id: Option<String>,
}

/// 添加签署方（addSigner），自动返回签署链接。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddSignerRequest {
    /// 签署方列表。
    pub signers: Vec<AddSignerItem>,
}

/// 陌生签署人认证配置（authConfig）。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthConfig {
    /// 显示给个人/办理人的页面认证方式列表。
    #[serde(rename = "authTypeList", skip_serializing_if = "Vec::is_empty")]
    pub auth_type_list: Vec<i32>,
    /// 企业认证方式列表（展示企业认证选项）。
    #[serde(rename = "companyAuthTypeList", skip_serializing_if = "Vec::is_empty")]
    pub company_auth_type_list: Vec<i32>,
    /// 是否展示授权页。
    #[serde(rename = "showAuthPage", skip_serializing_if = "Option::is_none")]
    pub show_auth_page: Option<bool>,
    /// 人脸认证模式。
    #[serde(rename = "faceAuthMode", skip_serializing_if = "Option::is_none")]
    pub face_auth_mode: Option<i32>,
}

/// 签章策略项（signStrategyList）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignStrategyItem {
    /// 附件序号。
    #[serde(rename = "attachNo", skip_serializing_if = "Option::is_none")]
    pub attach_no: Option<i32>,
    /// 定位方式。
    #[serde(rename = "locationMode", skip_serializing_if = "Option::is_none")]
    pub location_mode: Option<i32>,
    /// 签署关键字。
    #[serde(rename = "signKey", skip_serializing_if = "Option::is_none")]
    pub sign_key: Option<String>,
    /// 签署方式（signStrategyList 内）。
    #[serde(rename = "signType", skip_serializing_if = "Option::is_none")]
    pub sign_type: Option<i32>,
    /// 签署页码。
    #[serde(rename = "signPage", skip_serializing_if = "Option::is_none")]
    pub sign_page: Option<i32>,
    /// 签署 X 坐标（比例）。
    #[serde(rename = "signX", skip_serializing_if = "Option::is_none")]
    pub sign_x: Option<f64>,
    /// 签署 Y 坐标（比例）。
    #[serde(rename = "signY", skip_serializing_if = "Option::is_none")]
    pub sign_y: Option<f64>,
    /// 偏移 X。
    #[serde(rename = "offsetX", skip_serializing_if = "Option::is_none")]
    pub offset_x: Option<f64>,
    /// 偏移 Y。
    #[serde(rename = "offsetY", skip_serializing_if = "Option::is_none")]
    pub offset_y: Option<f64>,
    /// 是否可拖动。
    #[serde(rename = "canDrag", skip_serializing_if = "Option::is_none")]
    pub can_drag: Option<bool>,
}

/// 单个签署方。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddSignerItem {
    /// 业务方合同编号。
    #[serde(rename = "contractNo")]
    pub contract_no: String,
    /// 用户标识（爱签用户同步结果，或陌生用户标识）。
    pub account: String,
    /// 签署类型（2=无感知签约，3=有感知签约）。
    #[serde(rename = "signType", skip_serializing_if = "Option::is_none")]
    pub sign_type: Option<i32>,
    /// 印章号（可选）。
    #[serde(rename = "sealNo", skip_serializing_if = "Option::is_none")]
    pub seal_no: Option<String>,
    /// 签署顺序。
    #[serde(rename = "signOrder", skip_serializing_if = "Option::is_none")]
    pub sign_order: Option<i32>,
    /// 添加签署方后是否触发无感知签章（0=否，1=是）。
    #[serde(rename = "triggerSignFlag", skip_serializing_if = "Option::is_none")]
    pub trigger_sign_flag: Option<i32>,
    /// 指定具有印章授权的个人用章账号。
    #[serde(rename = "authSignAccount", skip_serializing_if = "Option::is_none")]
    pub auth_sign_account: Option<String>,
    /// 是否短信通知。
    #[serde(rename = "isNotice", skip_serializing_if = "Option::is_none")]
    pub is_notice: Option<bool>,
    /// 通知手机号（可选）。
    #[serde(rename = "noticeMobile", skip_serializing_if = "Option::is_none")]
    pub notice_mobile: Option<String>,
    /// 通知邮箱（可选）。
    #[serde(rename = "noticeEmail", skip_serializing_if = "Option::is_none")]
    pub notice_email: Option<String>,
    /// 陌生签署人认证配置（可选）。
    #[serde(rename = "authConfig", skip_serializing_if = "Option::is_none")]
    pub auth_config: Option<AuthConfig>,
    /// 签章策略列表（可选）。
    #[serde(rename = "signStrategyList", skip_serializing_if = "Option::is_none")]
    pub sign_strategy_list: Option<Vec<SignStrategyItem>>,
}

/// 签署链接结果。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignUrlResult {
    #[serde(default)]
    pub sign_url: Option<String>,
    #[serde(rename = "signUrl", default)]
    pub sign_url_camel: Option<String>,
    #[serde(rename = "contractNo", default)]
    pub contract_no: Option<String>,
    #[serde(rename = "contract_no", default)]
    pub contract_no_snake: Option<String>,
    /// 签署用户列表（爱签 addSigner 返回 signUser[].signUrl）。
    #[serde(rename = "signUser", default)]
    pub sign_user: Vec<AddSignerUserResult>,
}

/// addSigner 返回的签署用户信息。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddSignerUserResult {
    #[serde(default)]
    pub account: Option<String>,
    #[serde(rename = "signUrl", default)]
    pub sign_url: Option<String>,
    #[serde(rename = "pwdSignUrl", default)]
    pub pwd_sign_url: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub mobile: Option<String>,
}

impl SignUrlResult {
    /// 获取签署链接（兼容多种字段名：顶层 signUrl、signUser[].signUrl）。
    pub fn link(&self) -> Option<&str> {
        self.sign_url
            .as_deref()
            .or(self.sign_url_camel.as_deref())
            .or_else(|| {
                self.sign_user
                    .iter()
                    .find_map(|u| u.sign_url.as_deref())
            })
            .filter(|v| !v.trim().is_empty())
    }

    /// 获取指定爱签账号对应的签署链接。
    pub fn link_for_account(&self, account: &str) -> Option<&str> {
        self.sign_user
            .iter()
            .find(|user| user.account.as_deref() == Some(account))
            .and_then(|user| user.sign_url.as_deref().or(user.pwd_sign_url.as_deref()))
            .filter(|value| !value.trim().is_empty())
    }
}

/// 下载签署文档（downloadContract）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadContractRequest {
    /// 业务方合同编号。
    pub contract_no: String,
}

/// 下载结果。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadResult {
    #[serde(default)]
    pub file_content: Option<String>,
    #[serde(default)]
    pub download_url: Option<String>,
    #[serde(default)]
    pub file_name: Option<String>,
}

/// 查询合同状态（contract/status）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractStatusRequest {
    /// 业务方合同编号。
    #[serde(rename = "contractNo")]
    pub contract_no: String,
}

/// 触发无感知签章。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerAutoSignRequest {
    /// 业务方合同编号。
    #[serde(rename = "contractNo")]
    pub contract_no: String,
    /// 具备印章使用权限的企业用户账号。
    pub account: String,
}

/// 合同状态结果（宽松反序列化，兼容字段差异）。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContractStatusResult {
    #[serde(default)]
    pub contract_no: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub sign_status: Option<String>,
    #[serde(default)]
    pub contract_status: Option<String>,
    #[serde(default)]
    pub sign_users: Option<Vec<SignUserInfo>>,
    #[serde(default)]
    pub sign_user: Option<Vec<SignUserInfo>>,
    #[serde(default)]
    pub raw: Option<serde_json::Value>,
}

/// 签署人信息。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignUserInfo {
    #[serde(default)]
    pub account: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub sign_status: Option<String>,
    #[serde(default)]
    pub sign_time: Option<String>,
}

impl AsignClient {
    /// 上传待签署文件（创建合同）。
    ///
    /// 使用 multipart 直接上传文件（part 名 `contractFiles`），
    /// `contract_files` 中的 `file_url` 填文件名引用。
    pub async fn create_contract(
        &self,
        req: &CreateContractRequest,
        file_bytes: Vec<u8>,
        file_name: &str,
    ) -> AsignResult<ContractResult> {
        self.invoke_with_files(
            "/contract/createContract",
            req,
            "contractFiles",
            &[("contractFiles".to_string(), file_bytes, file_name.to_string())],
        )
        .await
    }

    /// 添加签署方，返回签署链接。
    ///
    /// 注意：爱签 addSigner 的 bizData 是签署方数组（顶层为 `[{...}]`）。
    pub async fn add_signer(
        &self,
        req: Vec<AddSignerItem>,
    ) -> AsignResult<SignUrlResult> {
        self.invoke("/contract/addSigner", &req).await
    }

    /// 查询合同状态。
    pub async fn contract_status(
        &self,
        req: &ContractStatusRequest,
    ) -> AsignResult<serde_json::Value> {
        self.invoke("/contract/status", req).await
    }

    /// 触发无感知签章。
    pub async fn trigger_auto_sign(
        &self,
        req: &TriggerAutoSignRequest,
    ) -> AsignResult<serde_json::Value> {
        self.invoke("/contract/triggerAutoSign", req).await
    }

    /// 下载签署文档。
    pub async fn download_contract(
        &self,
        req: &DownloadContractRequest,
    ) -> AsignResult<DownloadResult> {
        self.invoke("/contract/downloadContract", req).await
    }
}
