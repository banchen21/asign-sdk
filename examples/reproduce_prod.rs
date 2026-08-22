//! 复现线上爱签错误：用线上配置（新密钥 + production + app_id）直接调用爱签。
//! 运行：`cargo run --example reproduce_prod --features ...`
//! （在 asign-sdk 目录下执行：cargo run --example reproduce_prod）

use asign_sdk::{AsignClient, AsignConfig, Environment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let private_key = std::fs::read_to_string("/home/banchen/73info/keys/sign_private_key.pem")?;
    let private_key = private_key.trim().to_string();

    let config = AsignConfig::new(
        Environment::Production, // 正式环境 oapi.asign.cn（与线上一致）
        "140593923".to_string(),
        private_key,
    );

    let client = AsignClient::new(config)?;
    println!("客户端创建成功，AppId = {}", client.config().app_id);
    println!("目标环境 = {}", client.config().environment.base_url());

    // 1) 测试查询用户信息
    println!("\n=== 1) /user/getUser (account=test) ===");
    match client
        .get_user(&serde_json::json!({ "account": "test" }))
        .await
    {
        Ok(resp) => println!("✅ 成功：{resp}"),
        Err(err) => println!("❌ 失败：{err}"),
    }

    // 2) 测试发起个人实名认证（与线上相同的调用）
    println!("\n=== 2) /auth/person/identifyUrl ===");
    let mut req = asign_sdk::api::auth::PersonIdentifyUrlRequest::new(
        "张三".to_string(),
        "110101199001011234".to_string(),
        "13800138000".to_string(),
    );
    req.notify_url = Some("https://api.73info.cn/api/realname/asign/notify".to_string());
    match client.person_identify_url(&req).await {
        Ok(resp) => println!("✅ 成功：{:?}", resp),
        Err(err) => println!("❌ 失败：{err}"),
    }

    Ok(())
}
