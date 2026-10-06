//! 诊断工具：以指定爱签账号调用 previewContract，看签章 / 时间戳的落位。
//!
//! 用户端预览接口（`user_preview_asign_contract`）只允许合同当事人调用，平台账号不是可登录
//! 用户，所以「平台公章落在哪一行」没法走后端接口看，只能在 SDK 侧直接调。
//!
//! 用法：
//!   cargo run --example preview_seal -- <contractNo> <account> [private_key_path]
//!
//! 锚点与后端 `src/modules/contract/service.rs` 的 `sign_anchors()` / `sign_date_anchor()`
//! 逐字一致，漂移了就会报 100091 关键字未找到。

use asign_sdk::api::sign_task::{PreviewContractRequest, SignStrategyItem};
use asign_sdk::{AsignClient, AsignConfig, Environment};

/// 甲方（平台盖章）行的签名下划线锚点，与后端 `SIGN_KEY_PARTY_A` 一致。
const SIGN_KEY_PARTY_A: &str = "______________________";
/// 日期行的全角下划线锚点，与后端 `SIGN_KEY_DATE` 一致。
const SIGN_KEY_DATE: &str = "＿＿＿＿＿＿＿＿";

fn strategy(sign_key: &str, sign_type: i32, keyword_index: i32) -> SignStrategyItem {
    SignStrategyItem {
        attach_no: Some(1),
        location_mode: Some(3),
        sign_key: Some(sign_key.to_string()),
        keyword_index: Some(keyword_index),
        sign_type: Some(sign_type),
        sign_page: None,
        sign_x: Some(0.0),
        sign_y: Some(0.0),
        offset_x: Some(0.0),
        offset_y: Some(0.0),
        can_drag: Some(true),
    }
}

#[tokio::main]
async fn main() {
    let mut args = std::env::args().skip(1);
    let contract_no = args.next().expect("contractNo");
    let account = args.next().expect("account");
    let key_path = args.next().unwrap_or_else(|| {
        "../backend_73info/config/keys/asign_private_key.pem".to_string()
    });

    let key = std::fs::read_to_string(&key_path).expect("read private key");
    let client = AsignClient::new(AsignConfig::new(
        Environment::Test,
        "140593923".to_string(),
        key,
    ))
    .expect("client");

    let req = PreviewContractRequest {
        contract_no,
        account,
        // 企业账号不传手写章标记（与后端 is_enterprise 分支一致）。
        is_write: None,
        sign_strategy_list: vec![
            strategy(SIGN_KEY_PARTY_A, 1, -1),
            strategy(SIGN_KEY_DATE, 2, 1),
        ],
    };

    match client.preview_contract(&req).await {
        Ok(url) => println!("PREVIEW_URL={url}"),
        Err(err) => eprintln!("previewContract ERROR: {err}"),
    }
}
