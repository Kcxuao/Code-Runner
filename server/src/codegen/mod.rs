pub mod c_generator;
pub mod cpp_generator;
pub mod python_generator;

use crate::model::request::TestCase;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait CodeGenerator: Send + Sync {
    fn generate(&self, code: &str, function: &str, test_cases: &[TestCase]) -> Result<String>;
}