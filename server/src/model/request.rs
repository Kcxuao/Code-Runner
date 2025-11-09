use serde::Deserialize;

#[derive(Deserialize)]
pub struct TestCase {
    pub input: Vec<i32>,
    pub expected: i32,
}

#[derive(Deserialize)]
pub struct CodeRequest {
    pub language: String, // 指定语言
    pub code: String,
    pub function: String,
    pub test_cases: Vec<TestCase>,
}