//! 最小示例：创建客户端并调一次接口，验证连通性与加签是否被渠道接受。
//!
//! 运行前设置：
//!   ASIGN_APP_ID=<你的 AppId>
//!   ASIGN_PRIVATE_KEY_PATH=<RSA 私钥 PEM 路径>（缺省 keys/asign_private_key.pem）

use asign_sdk::{AsignClient, AsignConfig, Environment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("ASIGN_APP_ID").unwrap_or_else(|_| "your-app-id".to_string());
    let key_path = std::env::var("ASIGN_PRIVATE_KEY_PATH")
        .unwrap_or_else(|_| "keys/asign_private_key.pem".to_string());
    let private_key = std::fs::read_to_string(&key_path)?;

    let config = AsignConfig::new(
        Environment::Test, // 测试环境 prev.asign.cn；正式环境用 Environment::Production
        app_id,
        private_key,
    );

    let client = AsignClient::new(config)?;
    println!("客户端创建成功，AppId = {}", client.config().app_id);
    println!("目标环境 = {}", client.config().environment.base_url());

    // 查询用户信息（/user/getUser）
    match client
        .get_user(&serde_json::json!({ "account": "test" }))
        .await
    {
        Ok(resp) => println!("查询用户信息成功：{resp}"),
        Err(err) => println!("查询用户信息失败：{err}"),
    }

    Ok(())
}
