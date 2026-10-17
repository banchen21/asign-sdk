//! 查询合同状态：打印 `contract/status` 与 `contract/getContract` 的原始返回结构。
//!
//! 用法：cargo run --example check_status -- <contractNo>
//!
//! 运行前设置 `ASIGN_APP_ID` 与 `ASIGN_PRIVATE_KEY_PATH`。

use asign_sdk::api::sign_task::ContractStatusRequest;
use asign_sdk::{AsignClient, AsignConfig, Environment};

#[tokio::main]
async fn main() {
    let contract_no = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "your-contract-no".to_string());

    let app_id = std::env::var("ASIGN_APP_ID").unwrap_or_else(|_| "your-app-id".to_string());
    let key_path = std::env::var("ASIGN_PRIVATE_KEY_PATH")
        .unwrap_or_else(|_| "keys/asign_private_key.pem".to_string());
    let key = std::fs::read_to_string(&key_path).expect("read private key");
    let client = AsignClient::new(AsignConfig::new(Environment::Test, app_id, key)).expect("client");

    // 1) contract/status
    let req = ContractStatusRequest {
        contract_no: contract_no.clone(),
    };
    match client.contract_status(&req).await {
        Ok(raw) => {
            println!("=== contract/status raw for {contract_no} ===");
            println!("{}", serde_json::to_string_pretty(&raw).unwrap());
        }
        Err(err) => println!("contract/status ERROR: {err}"),
    }

    // 2) contract/getContract（查询合同信息）
    let biz = serde_json::json!({ "contractNo": contract_no });
    match client
        .invoke::<serde_json::Value, _>("/contract/getContract", &biz)
        .await
    {
        Ok(raw) => {
            println!("=== contract/getContract raw for {contract_no} ===");
            println!("{}", serde_json::to_string_pretty(&raw).unwrap());
        }
        Err(err) => println!("contract/getContract ERROR: {err}"),
    }
}
