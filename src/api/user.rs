//! 用户接口（签约参与方同步）。

use serde::{Deserialize, Serialize};

use crate::client::AsignClient;
use crate::AsignResult;

/// 添加个人用户（V2）。
///
/// 接口：`POST /v2/user/addPersonalUser`
///
/// 特别说明：
/// - 爱签平台认证模式：同步用户前需先通过认证服务完成实名认证，获得「实名认证流水号」
///   （serialNo），调用时必传；
/// - 业务方自行认证模式：若业务方已自行完成实名认证，可不传 serialNo，直接传
///   name/idCard/idCardType/mobile。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddPersonalUserRequest {
    /// 用户唯一识别码（可用证件号、手机号等唯一属性，签署时作为 account）。
    pub account: String,
    /// 实名认证流水号（爱签平台认证模式下必传）。
    #[serde(rename = "serialNo", skip_serializing_if = "Option::is_none")]
    pub serial_no: Option<String>,
    /// 用户姓名。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 身份证号。
    #[serde(rename = "idCard", skip_serializing_if = "Option::is_none")]
    pub id_card: Option<String>,
    /// 证件类型（默认 1=居民身份证）。
    #[serde(rename = "idCardType", default = "default_id_card_type")]
    pub id_card_type: i32,
    /// 手机号（用于接收合同签署短信验证码）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 签约密码明文（为空时爱签随机生成）。
    #[serde(rename = "signPwd", skip_serializing_if = "Option::is_none")]
    pub sign_pwd: Option<String>,
    /// 是否将签约密码以短信形式通知用户（0/1）。
    #[serde(rename = "isSignPwdNotice", skip_serializing_if = "Option::is_none")]
    pub is_sign_pwd_notice: Option<i32>,
    /// 是否在签署时进行短信通知（0/1）。
    #[serde(rename = "isNotice", skip_serializing_if = "Option::is_none")]
    pub is_notice: Option<i32>,
    /// base64 格式的印模图片（可选）。
    #[serde(rename = "sealPic", skip_serializing_if = "Option::is_none")]
    pub seal_pic: Option<String>,
}

fn default_id_card_type() -> i32 {
    1
}

/// 添加企业用户（V2）。
///
/// 接口：`POST /v2/user/addEnterpriseUser`
///
/// 特别说明：
/// - 爱签平台认证模式：同步用户前需先完成实名认证，获得流水号（serialNo），调用时传；
/// - 业务方自行认证模式：若业务方已自行完成企业实名认证，可不传 serialNo，直接传
///   companyName/creditCode/creditType/name/idCard/mobile/contactName/contactIdCard。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddEnterpriseUserRequest {
    /// 用户唯一识别码（可用证件号、手机号等唯一属性，签署时作为 account）。
    pub account: String,
    /// 实名认证流水号（爱签平台认证模式下传）。
    #[serde(rename = "serialNo", skip_serializing_if = "Option::is_none")]
    pub serial_no: Option<String>,
    /// 企业名称。
    #[serde(rename = "companyName", skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// 企业证件号（统一社会信用代码）。
    #[serde(rename = "creditCode", skip_serializing_if = "Option::is_none")]
    pub credit_code: Option<String>,
    /// 企业证件类型（默认 1=统一社会信用代码）。
    #[serde(rename = "creditType", default = "default_credit_type")]
    pub credit_type: i32,
    /// 企业法人姓名。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 法人身份证/台胞证/港澳通行证等证件号。
    #[serde(rename = "idCard", skip_serializing_if = "Option::is_none")]
    pub id_card: Option<String>,
    /// 证件类型（默认 1=居民身份证）。
    #[serde(rename = "idCardType", default = "default_id_card_type")]
    pub id_card_type: i32,
    /// 签约手机号（用于合同签署时短信验证）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 联系人姓名（与法定代表人可以不是同一人）。
    #[serde(rename = "contactName", skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    /// 联系人身份证号（与法定代表人可以不是同一人）。
    #[serde(rename = "contactIdCard", skip_serializing_if = "Option::is_none")]
    pub contact_id_card: Option<String>,
    /// 签约密码明文（为空时爱签随机生成）。
    #[serde(rename = "signPwd", skip_serializing_if = "Option::is_none")]
    pub sign_pwd: Option<String>,
    /// 是否将签约密码以短信形式通知用户（0/1）。
    #[serde(rename = "isSignPwdNotice", skip_serializing_if = "Option::is_none")]
    pub is_sign_pwd_notice: Option<i32>,
    /// 是否在签署时进行短信通知（0/1）。
    #[serde(rename = "isNotice", skip_serializing_if = "Option::is_none")]
    pub is_notice: Option<i32>,
}

fn default_credit_type() -> i32 {
    1
}

/// 添加陌生用户（V2）。
///
/// 接口：`POST /v2/user/addStranger`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddStrangerRequest {
    /// 用户唯一标识（业务方自定义，签署时作为 account）。
    pub account: String,
    /// 用户类型（爱签官方定义）：1=个人，2=企业。
    #[serde(rename = "userType")]
    pub user_type: i32,
    /// 用户姓名（个人）或企业联系人/法人姓名（企业）。
    pub name: String,
    /// 企业名称（企业用户必填）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// 手机号。
    pub mobile: String,
}

/// 用户同步结果。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSyncResult {
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub user_no: Option<String>,
}

impl AsignClient {
    /// 添加个人用户。
    ///
    /// 注意：爱签 V2 接口路径为 `/v2/user/addPersonalUser`。
    pub async fn add_personal_user(
        &self,
        req: &AddPersonalUserRequest,
    ) -> AsignResult<UserSyncResult> {
        self.invoke("/v2/user/addPersonalUser", req).await
    }

    /// 添加企业用户。
    pub async fn add_enterprise_user(
        &self,
        req: &AddEnterpriseUserRequest,
    ) -> AsignResult<UserSyncResult> {
        self.invoke("/v2/user/addEnterpriseUser", req).await
    }

    /// 添加陌生用户。
    ///
    /// 注意：爱签 V2 接口路径为 `/v2/user/addStranger`。
    pub async fn add_stranger(&self, req: &AddStrangerRequest) -> AsignResult<UserSyncResult> {
        self.invoke("/v2/user/addStranger", req).await
    }

    /// 查询用户信息。
    pub async fn get_user(&self, req: &serde_json::Value) -> AsignResult<serde_json::Value> {
        self.invoke("/user/getUser", req).await
    }
}
