# asign-sdk

爱签电子合同 OpenAPI（V3.0）的 Rust 客户端 SDK。

## 能力

- 配置：环境（测试 / 正式 / 自定义）、AppId、RSA 私钥
- 鉴权：爱签标准签名（SHA1withRSA + MD5）
- HTTP：form-data 请求封装与统一响应解析
- 接口分组：应用管理、实名认证、用户同步、文件、印章、签署任务、模板、存证

## 安装

```toml
[dependencies]
asign-sdk = "0.1"
```

同仓协作时也可以按路径引入：`asign-sdk = { path = "../asign-sdk" }`。

## 快速开始

```rust
use asign_sdk::{AsignClient, AsignConfig, Environment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AsignConfig::new(
        Environment::Test,
        "your-app-id".to_string(),
        "-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----".to_string(),
    );

    let client = AsignClient::new(config)?;

    let identify = client
        .person_identify_url(&asign_sdk::api::auth::PersonIdentifyRequest {
            name: "张三".to_string(),
            id_card: "110101199001011234".to_string(),
            phone: None,
            redirect_url: Some("https://example.com/realname/callback".to_string()),
        })
        .await?;

    println!("auth_url = {:?}", identify.auth_url);

    Ok(())
}
```

## 环境

| 环境 | 域名 |
|---|---|
| 测试 | `https://prev.asign.cn/` |
| 正式 | `https://oapi.asign.cn/` |

## 签名规范（已实现）

1. 请求方式：`form-data`；
2. 请求头：`sign`、`timestamp`（13 位毫秒）；
3. 请求体：`appId`、`timestamp`、`bizData`（JSON 字符串）；
4. 签名：
   - `bizData` 的 JSON 字段按 key 字母序排序；
   - `rsaSuffix = sortedJson + md5(sortedJson) + appId + timestamp`；
   - `SHA1withRSA`（SHA-1 摘要 + RSA PKCS#1 v1.5）；
   - base64 编码、去除换行后放入 header `sign`。

## 接入流程

普通用户：CA 认证 → 新增用户 → 自定义印模（可选）→ 上传文件 → 添加签署方（返回 signUrl）→ 签署 → 下载。

陌生用户：`add_stranger` → `create_contract` → `add_signer` → 签署 → `download_contract`。

## 注意

- 签名算法已按官方文档实现并通过测试。
- 各接口字段名需按官方接口文档逐一对齐（如 `serialNo`、`contractNo`、`signUrl`）。
- 测试环境短信验证码可用万能验证码 `111111`。
