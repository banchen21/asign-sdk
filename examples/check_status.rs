//! 诊断工具：查询爱签 contract/status 与 contract/getContract 的原始返回结构。
//! 用法：cargo run --example check_status -- <contractNo>

use asign_sdk::api::sign_task::ContractStatusRequest;
use asign_sdk::{AsignClient, AsignConfig, Environment};

#[tokio::main]
async fn main() {
    let contract_no = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "C20260910004834B67AB52F".to_string());

    let key_path = "/home/banch/dev73info/backend_73info/config/keys/asign_private_key.pem";
    let key = std::fs::read_to_string(key_path).expect("read private key");
    let client = AsignClient::new(AsignConfig::new(
        Environment::Test,
        "140593923".to_string(),
        key,
    ))
    .expect("client");

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

    // 2) contract/getContract（查询合同信息，文档：getContract）
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
