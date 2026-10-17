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
    /// 签署顺序：1=无序签约（默认），各方可独立签署；2=顺序签约（需等前一位签完）。
    ///
    /// ⚠️ 官方 `createContract` 文档原文是「1：无序签约（默认）、2：顺序签约」，
    /// 旧注释把它写反了（曾写成 1=顺序签 / 2=无序签）。本处只对齐注释，逻辑不变。
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
    /// 合同签署完成后同步回调地址（可选）：若不为空，可跳转业务方自己的前端过渡页面，
    /// 实现业务方自己的逻辑（例如返回平台用户端合同签署页并继续后续跳转）。
    #[serde(rename = "redirectUrl", skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// 合同发起页面完成后跳转 url（可选）：若不为空，可跳转业务方前端过渡页面。
    #[serde(
        rename = "redirectCompletedUrl",
        skip_serializing_if = "Option::is_none"
    )]
    pub redirect_completed_url: Option<String>,
    /// 合同发起页面返回按钮跳转 url（可选）。
    #[serde(rename = "redirectReturnUrl", skip_serializing_if = "Option::is_none")]
    pub redirect_return_url: Option<String>,
}

/// `createContract` 成功响应里回显的文件项（`data.contractFiles[]`）。
///
/// 只关心 `attachNo`（第几个文件）与文件名；其余字段按 `Value` 原样保留，
/// 避免渠道字段增减时解析失败。多文件建签时用它核对「主合同 = 1、附件 = 2」。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContractFileResult {
    /// 文件序号（`addSigner` 的签章策略用 `attachNo` 定位签在哪个文件上）。
    #[serde(rename = "attachNo", default)]
    pub attach_no: Option<i32>,
    /// 文件名（与上传时的 `fileUrl` 一致）。
    #[serde(rename = "fileUrl", default)]
    pub file_url: Option<String>,
    /// 文件名称（部分渠道版本用这个字段名）。
    #[serde(rename = "fileName", default)]
    pub file_name: Option<String>,
    /// 其余字段原样保留。
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
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
    /// 渠道回显的合同文件列表（`data.contractFiles[]`）。
    ///
    /// 多文件建签（R012：主合同 + 自定义附件）时，用它核对渠道确实收下了两个文件
    /// （主合同 `attachNo = 1`、附件 `attachNo = 2`）。单文件建签时长度为 0 或 1 都正常。
    #[serde(rename = "contractFiles", default)]
    pub contract_files: Vec<ContractFileResult>,
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
    /// 关键字或签署区名称（定位方式为关键字签章时此处需传定位关键字，模板坐标签章时此处需传模板中设置的签署区名称）。
    #[serde(rename = "signKey", skip_serializing_if = "Option::is_none")]
    pub sign_key: Option<String>,
    /// 关键字定位：1 - 仅第一个关键字出现位置签章；-1 - 仅最后一个关键字出现位置签章；不传 - 所有匹配位置。
    #[serde(rename = "keywordIndex", skip_serializing_if = "Option::is_none")]
    pub keyword_index: Option<i32>,
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
    /// 签章位置策略：0=由 signStrategyList/signStrikeList 指定（默认），
    /// 1=签署用户在签署时自行拖动签章位置，2=拖动签章位置和骑缝章位置。
    /// 对有感知签约（signType=3，手写）建议传 1，避免要求预置印模。
    #[serde(rename = "customSignFlag", skip_serializing_if = "Option::is_none")]
    pub custom_sign_flag: Option<i32>,
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
            .or_else(|| self.sign_user.iter().find_map(|u| u.sign_url.as_deref()))
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

/// 下载合同存证报告（`POST /v2/contract/downloadChainReport`）。
///
/// `type`：1=爱签存证报告；2=区块链存证报告；3=公证处存证报告。
/// 本项目仅使用 type=1（测试环境可用）；2/3 需账号另行开通。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadChainReportRequest {
    /// 爱签侧合同编号。
    #[serde(rename = "contractNo")]
    pub contract_no: String,
    /// 报告类型（固定传 1 表示爱签存证报告）。
    #[serde(rename = "type")]
    pub report_type: i32,
}

/// 存证报告下载结果（data 内字段）。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChainReportResult {
    /// 文件名。
    #[serde(rename = "fileName", default)]
    pub file_name: Option<String>,
    /// 文件 MD5。
    #[serde(default)]
    pub md5: Option<String>,
    /// 文件大小（渠道可能返回数字或字符串）。
    #[serde(default)]
    pub size: Option<serde_json::Value>,
    /// 文件 base64 内容。
    #[serde(default)]
    pub data: Option<String>,
    /// 文件类型：0=PDF。
    #[serde(rename = "fileType", default)]
    pub file_type: Option<i32>,
}

/// 查询合同状态（contract/status）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractStatusRequest {
    /// 业务方合同编号。
    #[serde(rename = "contractNo")]
    pub contract_no: String,
}

/// 查询合同信息（contract/getContract）。
///
/// 与 contract/status 相比，getContract 额外返回 `signUser` 签署人明细
/// （含每个签署人的 `signStatus`/`signFinishedTime`），可用于判断单个
/// 签署人是否已完成，而不必等待合同整体状态推进（如平台无感知盖章）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetContractRequest {
    /// 业务方合同编号。
    #[serde(rename = "contractNo")]
    pub contract_no: String,
}

/// 预览合同（previewContract）请求。
///
/// 创建待签署文件后、调用 addSigner 前，可预览合同在签署完成后的样式效果；
/// 合同已发起签署后调用同样返回预览链接，用于「查看合同」场景。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewContractRequest {
    /// 业务方合同编号（爱签侧）。
    #[serde(rename = "contractNo")]
    pub contract_no: String,
    /// 用户唯一识别码（爱签 account）。
    pub account: String,
    /// 是否手写章（个人用户必传）：1=是，0=否。
    #[serde(rename = "isWrite", skip_serializing_if = "Option::is_none")]
    pub is_write: Option<i32>,
    /// 合同附件中的签约策略（与 addSigner 的 signStrategyList 保持一致）。
    #[serde(rename = "signStrategyList")]
    pub sign_strategy_list: Vec<SignStrategyItem>,
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
            &[(
                "contractFiles".to_string(),
                file_bytes,
                file_name.to_string(),
            )],
        )
        .await
    }

    /// 上传待签署文件（创建合同），**多文件**版本（R012：主合同 + 自定义附件）。
    ///
    /// multipart 里每个文件都是 `contractFiles` part（part 名固定，可重复出现），
    /// 与 `bizData.contractFiles[].fileUrl` **按顺序、逐字**对应：第 1 个是主合同，
    /// 后续是附件。`addSigner` 的签章策略按 `attachNo` 定位，本项目写死 `Some(1)`，
    /// 因此主合同必须排在第一位、附件不落章。
    ///
    /// 渠道侧多文件下载产物是 zip（官方 `downloadContract`：文件数为 1 时是 pdf，否则为 zip）。
    ///
    /// `files`：`(文件名, 文件字节)`，文件名需与 `bizData` 里的 `fileUrl` 完全一致。
    pub async fn create_contract_with_files(
        &self,
        req: &CreateContractRequest,
        files: &[(String, Vec<u8>)],
    ) -> AsignResult<ContractResult> {
        let parts: Vec<(String, Vec<u8>, String)> = files
            .iter()
            .map(|(file_name, bytes)| {
                (
                    "contractFiles".to_string(),
                    bytes.clone(),
                    file_name.clone(),
                )
            })
            .collect();
        self.invoke_with_files("/contract/createContract", req, "contractFiles", &parts)
            .await
    }

    /// 添加签署方，返回签署链接。
    ///
    /// 注意：爱签 addSigner 的 bizData 是签署方数组（顶层为 `[{...}]`）。
    pub async fn add_signer(&self, req: Vec<AddSignerItem>) -> AsignResult<SignUrlResult> {
        self.invoke("/contract/addSigner", &req).await
    }

    /// 查询合同状态。
    pub async fn contract_status(
        &self,
        req: &ContractStatusRequest,
    ) -> AsignResult<serde_json::Value> {
        self.invoke("/contract/status", req).await
    }

    /// 查询合同信息（含签署人明细 signUser）。
    pub async fn get_contract(&self, req: &GetContractRequest) -> AsignResult<serde_json::Value> {
        self.invoke("/contract/getContract", req).await
    }

    /// 预览合同（previewContract），返回预览链接。
    ///
    /// 注意：与 addSigner 相同，官方请求示例的 bizData 是数组（`[{...}]`）。
    pub async fn preview_contract(&self, req: &PreviewContractRequest) -> AsignResult<String> {
        self.invoke("/contract/previewContract", &[req]).await
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

    /// 下载合同存证报告（爱签 / 区块链 / 公证处）。
    ///
    /// 接口：`POST /v2/contract/downloadChainReport`。
    pub async fn download_chain_report(
        &self,
        req: &DownloadChainReportRequest,
    ) -> AsignResult<ChainReportResult> {
        self.invoke("/v2/contract/downloadChainReport", req).await
    }
}
