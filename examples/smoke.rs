//! 最小联调示例：用真实 appId + 私钥调用爱签接口。
//!
//! 运行：`cargo run --example smoke --features...`（无需额外 feature）

use asign_sdk::{AsignClient, AsignConfig, Environment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let private_key = std::fs::read_to_string("keys/asign_private_key.pem")?;

    let config = AsignConfig::new(
        Environment::Test, // 测试环境 prev.asign.cn
        "140593923".to_string(),
        private_key,
    );

    let client = AsignClient::new(config)?;
    println!("客户端创建成功，AppId = {}", client.config().app_id);
    println!("目标环境 = {}", client.config().environment.base_url());

    // 测试「查询用户信息」接口连通性（/user/getUser）
    match client
        .get_user(&serde_json::json!({ "account": "test" }))
        .await
    {
        Ok(resp) => {
            println!("✅ 查询用户信息成功：{resp}");
        }
        Err(err) => {
            println!("❌ 查询用户信息失败：{err}");
        }
    }

    Ok(())
}
