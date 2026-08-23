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

// ── 修改用户信息（爱签文档「修改用户信息」分组） ──────────────────

/// 发送短信验证码（修改用户绑定手机号）。
///
/// 接口：`POST /user/sendVerifyCode`。返回的 `codeToken` 用于后续
/// `modify_mobile_by_code` 校验。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendVerifyCodeRequest {
    /// 手机号。
    pub mobile: String,
}

/// 发送验证码结果。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendVerifyCodeResult {
    /// 验证码令牌，用于修改手机号（验证码方式）。
    #[serde(rename = "codeToken", default)]
    pub code_token: Option<String>,
}

/// 修改手机号（验证码方式）。
///
/// 接口：`POST /user/modifyMobileByCode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyMobileByCodeRequest {
    /// 用户唯一识别码。
    pub account: String,
    /// 用户新手机号。
    pub mobile: String,
    /// 用户接收到的验证码。
    pub code: String,
    /// 调用发送短信接口返回的 codeToken。
    pub ctoken: String,
}

/// 修改手机号（运营商三要素校验）。
///
/// 接口：`POST /user/modifyMobile`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyMobileThreeElementRequest {
    /// 用户唯一识别码。
    pub account: String,
    /// 用户姓名（个人本人 / 企业法定代表人）。
    pub name: String,
    /// 个人身份证号 / 企业法定代表人身份证号。
    #[serde(rename = "idCard")]
    pub id_card: String,
    /// 新手机号。
    pub mobile: String,
}

/// 修改用户签约密码。
///
/// 接口：`POST /user/signPasswd`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifySignPasswordRequest {
    /// 用户唯一识别码。
    pub account: String,
    /// 旧签约密码。
    #[serde(rename = "oldSignPwd")]
    pub old_sign_pwd: String,
    /// 新签约密码。
    #[serde(rename = "signPwd")]
    pub sign_pwd: String,
}

/// 初始化签约密码（返回修改链接）。
///
/// 接口：`POST /user/startModSignPasswd`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartModSignPasswordRequest {
    /// 用户唯一识别码。
    pub account: String,
    /// 是否将修改链接以短信形式通知用户：0-不通知（默认），1-通知。
    #[serde(rename = "isNotice", skip_serializing_if = "Option::is_none")]
    pub is_notice: Option<i32>,
    /// 是否展示结果页：0-不展示（默认），1-展示。
    #[serde(rename = "isShowResult", skip_serializing_if = "Option::is_none")]
    pub is_show_result: Option<i32>,
}

/// 初始化签约密码结果。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartModSignPasswordResult {
    #[serde(default)]
    pub mobile: Option<String>,
    #[serde(default)]
    pub account: Option<String>,
    #[serde(rename = "signUrl", default)]
    pub sign_url: Option<String>,
    #[serde(rename = "partnerId", default)]
    pub partner_id: Option<i64>,
}

/// 重置签约密码。
///
/// 接口：`POST /user/resetSignPwd`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetSignPasswordRequest {
    /// 用户唯一识别码。
    pub account: String,
    /// 新签约密码。
    #[serde(rename = "signPwd")]
    pub sign_pwd: String,
}

/// 修改企业信息。
///
/// 接口：`POST /user/modifyCompanyInfo`。修改企业名称时默认生成新印章，
/// data 返回新的印章编号。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyCompanyInfoRequest {
    /// 用户唯一识别码。
    pub account: String,
    /// 企业名称（修改企业名称时必传）。
    #[serde(rename = "companyName", skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// 法人名称（修改法人信息时必传）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 法人身份证（修改法人信息时必传）。
    #[serde(rename = "idCard", skip_serializing_if = "Option::is_none")]
    pub id_card: Option<String>,
    /// 法人证件类型（修改法人信息时必传）：1=身份证。
    #[serde(rename = "idCardType", skip_serializing_if = "Option::is_none")]
    pub id_card_type: Option<i32>,
}

/// 修改个人信息。
///
/// 接口：`POST /v2/user/modifyUserName`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyUserNameRequest {
    /// 用户唯一识别码。
    pub account: String,
    /// 姓名。
    pub name: String,
    /// 认证类型：2=运营商三要素认证；3=银行卡四要素认证。
    #[serde(rename = "identifyType")]
    pub identify_type: i32,
    /// 用户手机号（identifyType=2 时必传）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 用户银联卡号（identifyType=3 时必传）。
    #[serde(rename = "bankCard", skip_serializing_if = "Option::is_none")]
    pub bank_card: Option<String>,
}

/// 用户重新认证。
///
/// 接口：`POST /v2/user/reAuthUser`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReAuthUserRequest {
    /// 用户唯一识别码。
    pub account: String,
    /// 实名认证流水号。
    #[serde(rename = "serialNo")]
    pub serial_no: String,
    /// 联系人姓名（与法定代表人可以是同一人）。
    #[serde(rename = "contactName", skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    /// 联系人身份证号。
    #[serde(rename = "contactIdCard", skip_serializing_if = "Option::is_none")]
    pub contact_id_card: Option<String>,
}

impl AsignClient {
    /// 发送短信验证码（修改用户绑定手机号）。
    ///
    /// 从响应 `data` 中取 `code_token` 供 `modify_mobile_by_code` 使用。
    pub async fn send_verify_code(
        &self,
        req: &SendVerifyCodeRequest,
    ) -> AsignResult<SendVerifyCodeResult> {
        self.invoke("/user/sendVerifyCode", req).await
    }

    /// 修改手机号（验证码方式）。
    pub async fn modify_mobile_by_code(
        &self,
        req: &ModifyMobileByCodeRequest,
    ) -> AsignResult<()> {
        self.invoke("/user/modifyMobileByCode", req).await
    }

    /// 修改手机号（运营商三要素校验）。
    pub async fn modify_mobile_three_elements(
        &self,
        req: &ModifyMobileThreeElementRequest,
    ) -> AsignResult<()> {
        self.invoke("/user/modifyMobile", req).await
    }

    /// 修改用户签约密码。
    pub async fn modify_sign_password(&self, req: &ModifySignPasswordRequest) -> AsignResult<()> {
        self.invoke("/user/signPasswd", req).await
    }

    /// 初始化签约密码（返回修改链接）。
    pub async fn start_mod_sign_password(
        &self,
        req: &StartModSignPasswordRequest,
    ) -> AsignResult<StartModSignPasswordResult> {
        self.invoke("/user/startModSignPasswd", req).await
    }

    /// 重置签约密码。
    pub async fn reset_sign_password(&self, req: &ResetSignPasswordRequest) -> AsignResult<()> {
        self.invoke("/user/resetSignPwd", req).await
    }

    /// 修改企业信息。返回新的印章编号（修改企业名称时）。
    pub async fn modify_company_info(
        &self,
        req: &ModifyCompanyInfoRequest,
    ) -> AsignResult<Option<String>> {
        self.invoke("/user/modifyCompanyInfo", req).await
    }

    /// 修改个人信息。
    pub async fn modify_user_name(&self, req: &ModifyUserNameRequest) -> AsignResult<Option<String>> {
        self.invoke("/v2/user/modifyUserName", req).await
    }

    /// 用户重新认证。
    pub async fn re_auth_user(&self, req: &ReAuthUserRequest) -> AsignResult<Option<String>> {
        self.invoke("/v2/user/reAuthUser", req).await
    }
}
