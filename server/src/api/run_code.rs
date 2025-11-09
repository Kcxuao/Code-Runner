use axum::Json;
use serde_json::json;
use std::sync::Arc;

use crate::codegen::c_generator::CCodeGenerator;
use crate::codegen::cpp_generator::CppCodeGenerator;
use crate::codegen::python_generator::PythonCodeGenerator;
use crate::codegen::CodeGenerator;
use crate::config::Config;
use crate::executor::universal_executor::execute_code;
use crate::model::request::CodeRequest;

pub async fn run_code(
    config: Arc<Config>,
    Json(req): Json<CodeRequest>,
) -> Json<serde_json::Value> {
    println!("\n🚀 Request received for language: {}", req.language);

    // 根据语言选择代码生成器
    let generator: Box<dyn CodeGenerator> = match req.language.as_str() {
        "c" => Box::new(CCodeGenerator),
        "cpp" => Box::new(CppCodeGenerator),
        "python" => Box::new(PythonCodeGenerator),
        _ => {
            return Json(json!({
                "error": format!("Unsupported language: {}", req.language)
            }));
        }
    };

    match execute_code(
        &req.language,
        &req.code,
        &req.function,
        &req.test_cases,
        &config,
        generator.as_ref(),
    )
    .await
    {
        Ok(result) => {
            println!("✅ Success\n");
            Json(result)
        }
        Err(e) => {
            eprintln!("❌ Error: {}\n", e);
            Json(json!({ "error": e.to_string() }))
        }
    }
}