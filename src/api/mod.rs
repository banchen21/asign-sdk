//! 各业务接口分组。
//!
//! 分组与爱签 OpenAPI V3.0 文档目录一致：
//! 应用管理、实名认证、信息比对、用户、印章、文件、签署、模板、存证、移动端。

pub mod app;
pub mod auth;
pub mod evidence;
pub mod file;
pub mod seal;
pub mod sign_task;
pub mod template;
pub mod user;
