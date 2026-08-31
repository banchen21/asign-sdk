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

/// 制作/更新企业印章（V2）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MakeCompanySealRequest {
    pub account: String,
    #[serde(rename = "sealNo")]
    pub seal_no: String,
    #[serde(rename = "sealName", skip_serializing_if = "Option::is_none")]
    pub seal_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<i32>,
    #[serde(rename = "isQrCodeSeal", skip_serializing_if = "Option::is_none")]
    pub is_qr_code_seal: Option<i32>,
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling: Option<f64>,
    #[serde(rename = "titleText", skip_serializing_if = "Option::is_none")]
    pub title_text: Option<String>,
    #[serde(rename = "viceText", skip_serializing_if = "Option::is_none")]
    pub vice_text: Option<String>,
    #[serde(rename = "rtnBase64", skip_serializing_if = "Option::is_none")]
    pub rtn_base64: Option<i32>,
}

/// 制作/更新企业印章结果。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MakeCompanySealResult {
    #[serde(rename = "sealNo", default)]
    pub seal_no: Option<String>,
    #[serde(default)]
    pub base64: Option<String>,
}

/// 将企业印章授权给个人用户（无校验）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoCheckSealAuthRequest {
    #[serde(rename = "authAccount")]
    pub auth_account: String,
    #[serde(rename = "sealNo")]
    pub seal_no: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<String>,
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<i32>,
}

/// 设置印章授权的默认使用者。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SealAuthDefaultRequest {
    #[serde(rename = "authAccount")]
    pub auth_account: String,
    #[serde(rename = "sealNo")]
    pub seal_no: String,
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<i32>,
}

/// 发起印章授权验证码校验。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptchaSealAuthRequest {
    #[serde(rename = "authAccount")]
    pub auth_account: String,
    #[serde(rename = "sealNo")]
    pub seal_no: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<String>,
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<i32>,
}

/// 校验印章授权验证码。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptchaSealAuthVerifyRequest {
    #[serde(rename = "captchaId")]
    pub captcha_id: String,
    pub captcha: String,
}

impl AsignClient {
    /// 制作企业印章（V2）。
    pub async fn make_company_seal(&self, req: &serde_json::Value) -> AsignResult<serde_json::Value> {
        self.invoke("/seal/makeCompanySeal", req).await
    }

    /// 制作/更新企业印章（V2）。
    pub async fn make_company_seal_v2(
        &self,
        req: &MakeCompanySealRequest,
    ) -> AsignResult<MakeCompanySealResult> {
        self.invoke("/seal/makeCompanySeal", req).await
    }

    /// 将企业印章授权给个人用户（无校验）。
    pub async fn authorize_seal_without_verification(
        &self,
        req: &NoCheckSealAuthRequest,
    ) -> AsignResult<serde_json::Value> {
        self.invoke("/seal/nocheckAuth", req).await
    }

    /// 设置印章授权默认使用者。
    pub async fn set_seal_auth_default(
        &self,
        req: &SealAuthDefaultRequest,
    ) -> AsignResult<serde_json::Value> {
        self.invoke("/seal/authDefault", req).await
    }

    /// 查询企业印章授权记录。
    pub async fn get_seal_auth_list(
        &self,
        req: &serde_json::Value,
    ) -> AsignResult<serde_json::Value> {
        self.invoke("/seal/authList", req).await
    }

    /// 发起印章授权验证码校验。
    pub async fn captcha_seal_auth(
        &self,
        req: &CaptchaSealAuthRequest,
    ) -> AsignResult<String> {
        self.invoke("/seal/captchaAuth", req).await
    }

    /// 校验印章授权验证码。
    pub async fn verify_captcha_seal_auth(
        &self,
        req: &CaptchaSealAuthVerifyRequest,
    ) -> AsignResult<serde_json::Value> {
        self.invoke("/seal/captchaAuthVerify", req).await
    }
}
