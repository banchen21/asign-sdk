//! 预览合同签章落位：调 `previewContract`，拿到带签章预览的地址。
//!
//! 用法：
//!   cargo run --example preview_seal -- <contractNo> <account>
//!
//! 运行前设置 `ASIGN_APP_ID` 与 `ASIGN_PRIVATE_KEY_PATH`。
//!
//! 下面的 `signKey` 锚点必须与合同模板里的文字**逐字一致**，漂移了会报
//! `100091 关键字未找到`。

use asign_sdk::api::sign_task::{PreviewContractRequest, SignStrategyItem};
use asign_sdk::{AsignClient, AsignConfig, Environment};

/// 签署方行的签名下划线锚点，按你的合同模板改。
const SIGN_KEY_PARTY_A: &str = "______________________";
/// 日期行的全角下划线锚点，按你的合同模板改。
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

    let app_id = std::env::var("ASIGN_APP_ID").unwrap_or_else(|_| "your-app-id".to_string());
    let key_path = std::env::var("ASIGN_PRIVATE_KEY_PATH")
        .unwrap_or_else(|_| "keys/asign_private_key.pem".to_string());
    let key = std::fs::read_to_string(&key_path).expect("read private key");
    let client = AsignClient::new(AsignConfig::new(Environment::Test, app_id, key)).expect("client");

    let req = PreviewContractRequest {
        contract_no,
        account,
        // 手写章标记：企业签署方不传。
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
