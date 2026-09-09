//! 实名认证接口（网页版 / H5）。
//!
//! 爱签在同步用户前需要先完成实名认证，并获得「实名认证流水号」，
//! 该流水号在调用同步用户接口时必须传入。
//!
//! 关键接口：
//! - `POST /user/identifyH5`：个人实名认证 H5（返回 H5 链接，用户完成人脸/意愿认证）
//! - `POST /user/faceResult`：查询人脸活体认证结果
//! - `POST /user/identify`：个人实名认证（返回认证流水号）

use serde::{Deserialize, Serialize};

use crate::client::AsignClient;
use crate::AsignResult;

/// 实名认证结果（返回 H5 链接 + 认证流水号）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentifyUrlResult {
    #[serde(default)]
    pub auth_url: Option<String>,
    #[serde(default)]
    pub identify_url: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub auth_no: Option<String>,
    #[serde(default)]
    pub serial_no: Option<String>,
    #[serde(default)]
    pub flow_no: Option<String>,
}

impl IdentifyUrlResult {
    /// 获取可用的 H5/网页认证链接。
    pub fn link(&self) -> Option<&str> {
        self.auth_url
            .as_deref()
            .or(self.identify_url.as_deref())
            .or(self.url.as_deref())
            .filter(|v| !v.trim().is_empty())
    }

    /// 获取认证流水号（serialNo）。
    pub fn serial_no(&self) -> Option<&str> {
        self.serial_no
            .as_deref()
            .or(self.auth_no.as_deref())
            .or(self.flow_no.as_deref())
            .filter(|v| !v.trim().is_empty())
    }
}

/// 个人实名认证 H5（新规版）。
///
/// 接口：`POST /user/identifyH5`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonIdentifyH5Request {
    /// 用户姓名。
    pub name: String,
    /// 身份证号。
    pub id_card: String,
    /// 认证方式（以官方枚举为准）。
    pub identify_type: i32,
    /// 是否需要二次认证（以官方枚举为准）。
    pub need_identify_two: i32,
    /// 认证完成通知地址（可选）。
    #[serde(rename = "notifyUrl", skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
    /// 人脸认证模式（可选）。
    #[serde(rename = "faceAuthMode", skip_serializing_if = "Option::is_none")]
    pub face_auth_mode: Option<i32>,
    /// 业务 ID（最长 32 位，用于查询人脸活体认证结果）。
    #[serde(rename = "bizId", skip_serializing_if = "Option::is_none")]
    pub biz_id: Option<String>,
}

/// 个人实名认证（返回认证流水号）。
///
/// 接口：`POST /user/identify`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonIdentifyRequest {
    /// 用户姓名。
    pub name: String,
    /// 身份证号。
    pub id_card: String,
    /// 手机号（可选）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 银行卡号（可选）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_card: Option<String>,
    /// 认证方式（必填）。
    pub identify_type: i32,
}

/// 个人实名认证（CA 认证网页版，返回认证链接 + 认证流水号）。
///
/// 接口：`POST /auth/person/identifyUrl`
///
/// 这是爱签官方「CA 认证」流程的入口接口，响应为结构化 JSON：
/// ```json
/// {"faceUrl":null,"identifyUrl":"https://pre.asign.cn/m/#/mobile/apiIdentifyUrl/{token}",
///  "process":null,"result":0,"serialNo":"PA15020260820162527935182",
///  "shortUrl":"https://h5.asign.cn/web/short/xxx","type":"个人认证网页版"}
/// ```
/// `serialNo` 即「实名认证流水号」（PA 开头），认证完成后用于 `addPersonalUser`。
///
/// 参数说明（爱签文档）：
/// - `realName`：用户姓名，传值后认证页面上不可修改，传空表示可以由用户输入
/// - `idCardNo`：身份证号，传值后认证页面上不可修改，传空表示可以由用户输入
/// - `mobile`：手机号码，传值后认证页面上不可修改
/// - `authTypeList`：页面认证方式列表（151 运营商三要素 / 152 银行卡四要素 / 153 人脸 / 154 视频双录）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonIdentifyUrlRequest {
    /// 用户姓名（传值后认证页面上不可修改，传空表示可以由用户输入）。
    #[serde(rename = "realName", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 身份证号（传值后认证页面上不可修改，传空表示可以由用户输入）。
    #[serde(rename = "idCardNo", skip_serializing_if = "Option::is_none")]
    pub id_card_no: Option<String>,
    /// 手机号码（传值后认证页面上不可修改）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 银行卡号（可选）。
    #[serde(rename = "bankCard", skip_serializing_if = "Option::is_none")]
    pub bank_card: Option<String>,
    /// 页面认证方式列表：151 运营商三要素 / 152 银行卡四要素 / 153 人脸 / 154 视频双录。
    /// 不传默认开放所有开通的认证方式。
    #[serde(rename = "authTypeList", skip_serializing_if = "Vec::is_empty")]
    pub auth_type_list: Vec<i32>,
    /// 认证完成后同步重定向跳转 H5 页面地址（可选）。
    #[serde(rename = "redirectUrl", skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// 认证完成后异步通知地址（可选）。
    #[serde(rename = "notifyUrl", skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
    /// 业务方自定义业务编号，异步通知时会回传（最长 60 位）。
    #[serde(rename = "bizId", skip_serializing_if = "Option::is_none")]
    pub biz_id: Option<String>,
    /// 人脸识别方式：1=支付宝 / 2=H5（默认）/ 4=微信小程序 / 5=支付宝小程序 / 9=支付宝H5版。
    #[serde(rename = "faceAuthMode", skip_serializing_if = "Option::is_none")]
    pub face_auth_mode: Option<i32>,
    /// 认证失败后是否允许重新提交：1=是，0=否（默认）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resubmit: Option<i32>,
    /// 认证结束后是否展示结果页：1=是（默认），0=否。
    #[serde(rename = "showResult", skip_serializing_if = "Option::is_none")]
    pub show_result: Option<i32>,
    /// 签约手机号采集标记：1=是（默认），0=否。
    #[serde(rename = "mobileRequired", skip_serializing_if = "Option::is_none")]
    pub mobile_required: Option<i32>,
    /// H5 人脸需要开启无 Cookie 模式或使用 IFrame 框架：1=是，0=否（默认）。
    #[serde(rename = "isIframe", skip_serializing_if = "Option::is_none")]
    pub is_iframe: Option<i32>,
    /// 是否将认证链接以短信形式通知用户：0=否（默认），1=是。
    #[serde(rename = "isNotice", skip_serializing_if = "Option::is_none")]
    pub is_notice: Option<i32>,
    /// 通知手机号。
    #[serde(rename = "noticeMobile", skip_serializing_if = "Option::is_none")]
    pub notice_mobile: Option<String>,
    /// 实名认证时是否采集签约密码：0=否（默认），1=是。
    #[serde(rename = "needSignPwd", skip_serializing_if = "Option::is_none")]
    pub need_sign_pwd: Option<i32>,
}

impl PersonIdentifyUrlRequest {
    /// 便捷构造：姓名 + 证件号 + 手机号（其余参数取默认）。
    pub fn new(
        name: impl Into<String>,
        id_card_no: impl Into<String>,
        mobile: impl Into<String>,
    ) -> Self {
        Self {
            name: Some(name.into()),
            id_card_no: Some(id_card_no.into()),
            mobile: Some(mobile.into()),
            bank_card: None,
            auth_type_list: vec![],
            redirect_url: None,
            notify_url: None,
            biz_id: None,
            face_auth_mode: None,
            resubmit: None,
            show_result: None,
            mobile_required: None,
            is_iframe: None,
            is_notice: None,
            notice_mobile: None,
            need_sign_pwd: None,
        }
    }
}

/// 个人实名认证（CA 认证网页版）结果。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonIdentifyUrlResult {
    /// 人脸识别链接（独立人脸核身场景）。
    #[serde(default)]
    pub face_url: Option<String>,
    /// CA 认证网页链接（用户打开完成实名认证）。
    #[serde(rename = "identifyUrl", default)]
    pub identify_url: Option<String>,
    /// 流程状态（预留）。
    #[serde(default)]
    pub process: Option<String>,
    /// 发起结果（0=成功）。
    #[serde(default)]
    pub result: Option<i32>,
    /// 实名认证流水号（PA 开头，认证完成后用于 addPersonalUser/addEnterpriseUser）。
    #[serde(rename = "serialNo", default)]
    pub serial_no: Option<String>,
    /// 短链接（H5 分享用）。
    #[serde(rename = "shortUrl", default)]
    pub short_url: Option<String>,
    /// 认证类型说明。
    #[serde(rename = "type", default)]
    pub auth_type: Option<String>,
}

impl PersonIdentifyUrlResult {
    /// 获取可用的认证网页链接。
    pub fn link(&self) -> Option<&str> {
        self.identify_url
            .as_deref()
            .or(self.face_url.as_deref())
            .filter(|v| !v.trim().is_empty())
    }

    /// 获取实名认证流水号（serialNo）。
    pub fn serial_no(&self) -> Option<&str> {
        self.serial_no
            .as_deref()
            .filter(|v| !v.trim().is_empty())
    }
}

/// 企业实名认证（CA 认证网页版）。
///
/// 接口：`POST /auth/company/identifyUrl`
///
/// 这也是爱签「CA 认证」流程的企业入口，返回企业实名认证链接与认证流水号，
/// 认证完成后（经办人/法人完成核验）可用流水号调用 `addEnterpriseUser` 同步企业用户。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyIdentifyUrlRequest {
    /// 页面认证方式列表（151 运营商三要素 / 152 银行卡四要素 / 153 人脸 / 154 视频双录）。
    #[serde(rename = "authTypeList", skip_serializing_if = "Vec::is_empty")]
    pub auth_type_list: Vec<i32>,
    /// 是否授权经办人签署。
    #[serde(rename = "agentSignPower", skip_serializing_if = "Option::is_none")]
    pub agent_sign_power: Option<bool>,
    /// 公司认证方式列表（351 法定代表人 / 369 线下盖章授权（默认）/ 370 / 371 对公打款）。
    #[serde(rename = "companyAuthTypeList", skip_serializing_if = "Vec::is_empty")]
    pub company_auth_type_list: Vec<i32>,
    /// 预填企业名称。
    #[serde(rename = "companyName", skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// 预填社会统一信用代码号。
    #[serde(rename = "creditCode", skip_serializing_if = "Option::is_none")]
    pub credit_code: Option<String>,
    /// 预填法人姓名。
    #[serde(rename = "legalPersonName", skip_serializing_if = "Option::is_none")]
    pub legal_person_name: Option<String>,
    /// 预填法人身份证号。
    #[serde(rename = "legalPersonIdCardNo", skip_serializing_if = "Option::is_none")]
    pub legal_person_id_card_no: Option<String>,
    /// 预填法人手机号。
    #[serde(rename = "legalPersonMobile", skip_serializing_if = "Option::is_none")]
    pub legal_person_mobile: Option<String>,
    /// 预填办理人姓名。
    #[serde(rename = "agentName", skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    /// 预填办理人身份证。
    #[serde(rename = "agentIdCardNo", skip_serializing_if = "Option::is_none")]
    pub agent_id_card_no: Option<String>,
    /// 预填办理人手机号。
    #[serde(rename = "agentMobile", skip_serializing_if = "Option::is_none")]
    pub agent_mobile: Option<String>,
    /// 指定页面不可修改的参数（companyName/creditCode/legalPersonName/.../agentMobile 等）。
    #[serde(rename = "immutableInfoList", skip_serializing_if = "Vec::is_empty")]
    pub immutable_info_list: Vec<String>,
    /// 认证完成后同步重定向跳转 H5 页面地址（可选）。
    #[serde(rename = "redirectUrl", skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// 认证完成后异步通知地址（可选）。
    #[serde(rename = "notifyUrl", skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
    /// 是否开启支付宝快捷认证：1 不开启（默认）/ 2 开启支付宝企业认证。
    #[serde(rename = "authMode", skip_serializing_if = "Option::is_none")]
    pub auth_mode: Option<i32>,
    /// 支付宝认证结束后回跳 scheme 地址（authMode=2 时生效）。
    #[serde(rename = "zhimaScheme", skip_serializing_if = "Option::is_none")]
    pub zhima_scheme: Option<String>,
    /// 业务方自定义业务编号，异步通知时会回传（最长 60 位）。
    #[serde(rename = "bizId", skip_serializing_if = "Option::is_none")]
    pub biz_id: Option<String>,
    /// 人脸识别方式：1=支付宝 / 2=H5（默认）/ 4=微信小程序 / 5=支付宝小程序 / 9=支付宝H5版。
    #[serde(rename = "faceAuthMode", skip_serializing_if = "Option::is_none")]
    pub face_auth_mode: Option<i32>,
    /// 认证结束后是否展示结果页：1=是（默认），0=否。
    #[serde(rename = "showResult", skip_serializing_if = "Option::is_none")]
    pub show_result: Option<i32>,
    /// 签约手机号采集标记：1=是（默认），0=否。
    #[serde(rename = "mobileRequired", skip_serializing_if = "Option::is_none")]
    pub mobile_required: Option<i32>,
    /// H5 人脸是否开启无 Cookie 模式或 IFrame 框架：1=是，0=否（默认）。
    #[serde(rename = "isIframe", skip_serializing_if = "Option::is_none")]
    pub is_iframe: Option<i32>,
    /// 是否将认证链接以短信形式通知用户：0=否（默认），1=是。
    #[serde(rename = "isNotice", skip_serializing_if = "Option::is_none")]
    pub is_notice: Option<i32>,
    /// 通知手机号。
    #[serde(rename = "noticeMobile", skip_serializing_if = "Option::is_none")]
    pub notice_mobile: Option<String>,
    /// 实名认证时是否采集签约密码：0=否（默认），1=是。
    #[serde(rename = "needSignPwd", skip_serializing_if = "Option::is_none")]
    pub need_sign_pwd: Option<i32>,
}

impl CompanyIdentifyUrlRequest {
    /// 便捷构造：企业名称 + 统一社会信用代码 + 办理人姓名 + 办理人身份证号 + 办理人手机号。
    pub fn new(
        company_name: impl Into<String>,
        credit_code: impl Into<String>,
        agent_name: impl Into<String>,
        agent_id_card_no: impl Into<String>,
        agent_mobile: impl Into<String>,
    ) -> Self {
        Self {
            auth_type_list: vec![],
            agent_sign_power: None,
            company_auth_type_list: vec![],
            company_name: Some(company_name.into()),
            credit_code: Some(credit_code.into()),
            legal_person_name: None,
            legal_person_id_card_no: None,
            legal_person_mobile: None,
            agent_name: Some(agent_name.into()),
            agent_id_card_no: Some(agent_id_card_no.into()),
            agent_mobile: Some(agent_mobile.into()),
            immutable_info_list: vec![],
            redirect_url: None,
            notify_url: None,
            auth_mode: None,
            zhima_scheme: None,
            biz_id: None,
            face_auth_mode: None,
            show_result: None,
            mobile_required: None,
            is_iframe: None,
            is_notice: None,
            notice_mobile: None,
            need_sign_pwd: None,
        }
    }
}

/// 查询实名认证流水号。
///
/// 接口：`POST /auth/getSerialNo`
///
/// 可通过用户名称（个人/企业）查询实名认证流水号信息，用于获取已完成的
/// 认证记录 serialNo，再调用 `addEnterpriseUser`/`addPersonalUser` 同步用户。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSerialNoRequest {
    /// 用户类型：1=企业，2=个人。
    #[serde(rename = "userType", skip_serializing_if = "Option::is_none")]
    pub user_type: Option<i32>,
    /// 姓名 或 企业名称。
    #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 身份证 或 企业证件号。
    #[serde(rename = "idNo", skip_serializing_if = "Option::is_none")]
    pub id_no: Option<String>,
}

/// 实名认证流水号单条记录。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetSerialNoItem {
    #[serde(rename = "authTypeCode", default)]
    pub auth_type_code: Option<i32>,
    #[serde(rename = "authTypeName", default)]
    pub auth_type_name: Option<String>,
    #[serde(default)]
    pub result: Option<i32>,
    #[serde(rename = "serialNo", default)]
    pub serial_no: Option<String>,
    #[serde(rename = "userType", default)]
    pub user_type: Option<i32>,
}

/// 查询实名认证流水号结果。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetSerialNoResult {
    #[serde(default)]
    pub data: Vec<GetSerialNoItem>,
}

impl GetSerialNoResult {
    /// 取第一条结果（result=1 表示认证成功）的流水号。
    pub fn first_serial_no(&self) -> Option<&str> {
        self.data
            .iter()
            .find(|item| item.result == Some(1))
            .or_else(|| self.data.first())
            .and_then(|item| item.serial_no.as_deref())
            .filter(|value| !value.trim().is_empty())
    }
}

/// 查询人脸活体认证结果。
///
/// 接口：`POST /user/faceResult`
///
/// 注意：爱签 `faceResult` 接口以 `bizId`（业务 ID）查询认证结果。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceResultRequest {
    /// 业务 ID（发起认证时传入的 bizId）。
    #[serde(rename = "bizId")]
    pub biz_id: String,
}

/// 人脸活体认证结果。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceResultResponse {
    #[serde(default)]
    pub result: Option<bool>,
    #[serde(default)]
    pub passed: Option<bool>,
    #[serde(default)]
    pub result_code: Option<String>,
    #[serde(default)]
    pub result_message: Option<String>,
}

/// 查询实名认证结果。
///
/// 接口：`POST /auth/result`
///
/// 注意：爱签 `auth/result` 接口以 `serialNo`（实名认证流水号）查询认证结果。
/// 该流水号在 `identifyH5` 返回的 H5 链接路径 `apiIdentify/{serialNo}` 中。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResultRequest {
    /// 实名认证流水号（identifyH5 返回 URL 路径中的 serialNo）。
    #[serde(rename = "serialNo")]
    pub serial_no: String,
}

/// 实名认证结果。
///
/// ⚠️ 爱签 `auth/result` 返回的 `result` 是**整数**（1=成功），不是布尔值，
/// 因此用 `i32` 解析；同时兼容部分场景返回的字符串状态。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResultResponse {
    /// 认证结果（1=成功，0/其他=失败或未完成）。
    #[serde(default)]
    pub result: Option<i32>,
    /// 兼容布尔类型结果。
    #[serde(default)]
    pub passed: Option<bool>,
    /// 状态文本（pass / fail 等）。
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub serial_no: Option<String>,
    #[serde(default)]
    pub result_code: Option<String>,
    #[serde(default)]
    pub result_message: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}

impl AsignClient {
    /// 个人实名认证 H5。
    ///
    /// 爱签该接口直接返回 H5 认证链接的 JSON 字符串（如
    /// `"https://pre.asign.cn/m/#/mobile/apiIdentify/..."`），因此返回 `String`。
    /// 认证流水号在返回 URL 的路径 `apiIdentify/{serialNo}` 中，
    /// 认证完成后可用该流水号通过 `auth_result` 查询结果。
    pub async fn person_identify_h5(&self, req: &PersonIdentifyH5Request) -> AsignResult<String> {
        self.invoke("/user/identifyH5", req).await
    }

    /// 个人实名认证（返回认证流水号）。
    pub async fn person_identify(
        &self,
        req: &PersonIdentifyRequest,
    ) -> AsignResult<IdentifyUrlResult> {
        self.invoke("/user/identify", req).await
    }

    /// 个人实名认证（CA 认证网页版，返回认证链接 + 认证流水号）。
    ///
    /// 官方接入流程：`auth/person/identifyUrl` 发起 CA 认证 → 用户完成认证 →
    /// 用返回的 `serialNo`（PA 开头）调用 `addPersonalUser` 同步用户。
    pub async fn person_identify_url(
        &self,
        req: &PersonIdentifyUrlRequest,
    ) -> AsignResult<PersonIdentifyUrlResult> {
        self.invoke("/auth/person/identifyUrl", req).await
    }

    /// 企业实名认证（CA 认证网页版，返回认证链接 + 认证流水号）。
    ///
    /// 官方接入流程：`auth/company/identifyUrl` 发起企业 CA 认证 → 经办人/法人完成认证 →
    /// 用返回的 `serialNo` 调用 `addEnterpriseUser` 同步企业用户。
    pub async fn company_identify_url(
        &self,
        req: &CompanyIdentifyUrlRequest,
    ) -> AsignResult<PersonIdentifyUrlResult> {
        self.invoke("/auth/company/identifyUrl", req).await
    }

    /// 查询实名认证流水号。
    ///
    /// 官方接入：`auth/getSerialNo` — 通过企业名称/证件号查询已完成的认证记录，
    /// 拿到 `serialNo` 后再用 `addEnterpriseUser` 同步企业用户。
    pub async fn get_serial_no(
        &self,
        req: &GetSerialNoRequest,
    ) -> AsignResult<GetSerialNoResult> {
        self.invoke("/auth/getSerialNo", req).await
    }

    /// 查询实名认证结果（serialNo 查询）。
    pub async fn auth_result(&self, req: &AuthResultRequest) -> AsignResult<AuthResultResponse> {
        self.invoke("/auth/result", req).await
    }

    /// 查询人脸活体认证结果（bizId 查询，用于独立人脸核身场景）。
    pub async fn face_result(&self, req: &FaceResultRequest) -> AsignResult<FaceResultResponse> {
        self.invoke("/user/faceResult", req).await
    }
}
