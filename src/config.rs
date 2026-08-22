//! SDK 配置与环境。

/// 爱签 API 环境。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    /// 测试环境：`https://prev.asign.cn/`
    Test,
    /// 正式环境：`https://oapi.asign.cn/`
    Production,
    /// 自定义地址（如自建代理 / 混合云）。
    Custom(&'static str),
}

impl Environment {
    /// 返回环境的基础地址（不含协议尾部的路径）。
    pub fn base_url(&self) -> &str {
        match self {
            Environment::Test => "https://prev.asign.cn/",
            Environment::Production => "https://oapi.asign.cn/",
            Environment::Custom(url) => url,
        }
    }
}

/// 爱签客户端配置。
#[derive(Debug, Clone)]
pub struct AsignConfig {
    pub environment: Environment,
    pub app_id: String,
    pub private_key: String,
}

impl AsignConfig {
    /// 创建配置。
    ///
    /// - `environment`：测试 / 正式 / 自定义。
    /// - `app_id`：应用 ID。
    /// - `private_key`：RSA 私钥（PEM 格式字符串）。
    pub fn new(environment: Environment, app_id: String, private_key: String) -> Self {
        Self {
            environment,
            app_id,
            private_key,
        }
    }

    /// 校验配置是否完整。
    pub fn validate(&self) -> crate::AsignResult<()> {
        if self.app_id.trim().is_empty() {
            return Err(crate::AsignError::Config(
                "app_id must not be empty".to_string(),
            ));
        }
        if self.private_key.trim().is_empty() {
            return Err(crate::AsignError::Config(
                "private_key must not be empty".to_string(),
            ));
        }
        Ok(())
    }
}
