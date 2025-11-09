use anyhow::Result;
use base64::{Engine as _, engine::general_purpose};
use bollard::exec::{CreateExecOptions, StartExecResults};
use bytes::BytesMut;
use futures_util::StreamExt;
use serde_json::json;
use std::time::{Duration, Instant};
use tokio::time::timeout;
use uuid::Uuid;

use crate::codegen::CodeGenerator;
use crate::config::Config;
use crate::docker::client::get_docker;
use crate::model::request::TestCase;
use crate::parser::parse_output::parse_output_to_json;

pub async fn execute_code(
    lang: &str,
    code: &str,
    function: &str,
    test_cases: &[TestCase],
    config: &Config,
    generator: &dyn CodeGenerator,
) -> Result<serde_json::Value> {
    let total_start = Instant::now();
    let docker = get_docker();

    // 验证语言是否启用
    let lang_config = config
        .get_language(lang)
        .ok_or_else(|| anyhow::anyhow!("Language '{}' not supported or disabled", lang))?;

    // 获取容器名称
    let container_name = config
        .get_container(lang)
        .ok_or_else(|| anyhow::anyhow!("Container for '{}' not configured", lang))?;

    let file_id = Uuid::new_v4();
    let source_file = format!("/tmp/main_{}.{}", file_id, lang_config.file_extension);
    let binary_file = format!("/tmp/main_{}", file_id);

    println!("⏱️  [0ms] Starting {} execution", lang);

    // 生成测试代码
    let generated_code = generator.generate(code, function, test_cases)?;
    println!(
        "⏱️  [{}ms] Code generated",
        total_start.elapsed().as_millis()
    );

    // ===== 使用 Base64 写入文件 =====
    let encoded_code = general_purpose::STANDARD.encode(&generated_code);
    let write_cmd = format!("echo '{}' | base64 -d > {}", encoded_code, source_file);
    // ===============================

    let timeout_cmd = "timeout 5s";

    // 构造完整命令
    let full_cmd = if let Some(compile_cmd) = &lang_config.compile_cmd {
        // 编译型语言
        let compile = compile_cmd
            .replace("{source}", &source_file)
            .replace("{binary}", &binary_file);
        let run = lang_config.run_cmd.replace("{binary}", &binary_file);
        format!(
            "{} && {} && {} bash -c '{} 2>&1; EXIT_CODE=$?; rm -f {} {}; exit $EXIT_CODE'",
            write_cmd, compile, timeout_cmd, run, source_file, binary_file
        )
    } else {
        // 解释型语言
        let run = lang_config.run_cmd.replace("{source}", &source_file);
        format!(
            "{} && {} bash -c '{} 2>&1; EXIT_CODE=$?; rm -f {}; exit $EXIT_CODE'",
            write_cmd, timeout_cmd, run, source_file
        )
    };

    println!(
        "⏱️  [{}ms] Creating exec",
        total_start.elapsed().as_millis()
    );

    let exec = docker
        .create_exec(
            container_name,
            CreateExecOptions {
                cmd: Some(vec!["bash", "-c", &full_cmd]),
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                tty: Some(false),
                ..Default::default()
            },
        )
        .await?;

    println!(
        "⏱️  [{}ms] Exec created, starting",
        total_start.elapsed().as_millis()
    );

    let mut buffer = BytesMut::with_capacity(8192);

    let exec_future = async {
        if let StartExecResults::Attached { mut output, .. } =
            docker.start_exec(&exec.id, None).await?
        {
            while let Some(result) = output.next().await {
                if let Ok(msg) = result {
                    use bollard::container::LogOutput;
                    match msg {
                        LogOutput::StdOut { message } | LogOutput::StdErr { message } => {
                            buffer.extend_from_slice(&message);
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok::<(), anyhow::Error>(())
    };

    // Rust层超时保护
    let result = timeout(Duration::from_secs(config.server.run_timeout), exec_future).await;

    let elapsed = total_start.elapsed().as_millis();

    let output_text = match result {
        Ok(inner) => {
            if let Err(e) = inner {
                format!("❌ Internal error: {}", e)
            } else {
                String::from_utf8_lossy(&buffer).to_string()
            }
        }
        Err(_) => "⏰ Execution timed out (killed by host)".to_string(),
    };

    println!("⏱️  [{}ms] Completed", elapsed);

    Ok(json!({
        "language": lang,
        "execution_time_ms": elapsed,
        "output": parse_output_to_json(&output_text)
    }))
}
