use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::{Value, json};

/// 解析每行测试用例输出为统一 JSON
pub fn parse_output_to_json(output: &str) -> Value {
    static RE_OUTPUT: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            r"Input: \((?P<input>.*?)\) => Output: (?P<output>-?\d+) \| Expected: (?P<expected>-?\d+) \| (?P<result>✅ PASS|❌ FAIL) \| Time: (?P<time>[0-9.]+)ms"
        ).unwrap()
    });

    let mut cases = Vec::new();
    let mut total_time = 0.0;
    let mut peak_memory_kb = 0i64;

    for line in output.lines() {
        if let Some(cap) = RE_OUTPUT.captures(line) {
            let input_str = &cap["input"];
            let output_val: i64 = cap["output"].parse().unwrap_or(0);
            let expected_val: i64 = cap["expected"].parse().unwrap_or(0);
            let passed = &cap["result"] == "✅ PASS";
            let time_ms: f64 = cap["time"].parse().unwrap_or(0.0);

            total_time += time_ms;

            let input_vec: Vec<i64> = if input_str.trim().is_empty() {
                vec![]
            } else {
                input_str
                    .split(',')
                    .map(|s| s.trim().parse().unwrap_or(0))
                    .collect()
            };

            cases.push(json!({
                "input": input_vec,
                "output": output_val,
                "expected": expected_val,
                "passed": passed,
                "time_ms": time_ms
            }));
        } else if line.starts_with("Memory:") {
            // 提取峰值内存
            if let Some(val) = line.split(':').nth(1) {
                peak_memory_kb = val.trim().trim_end_matches(" KB").parse().unwrap_or(0);
            }
        }
    }

    let total = cases.len();
    let pass_count = cases
        .iter()
        .filter(|c| c.get("passed").and_then(|v| v.as_bool()) == Some(true))
        .count();
    let pass_rate = if total > 0 {
        pass_count as f64 / total as f64
    } else {
        0.0
    };

    if total == 0 {
        json!({"error": "代码未产生有效输出，可能存在编译或运行错误。"})
    } else {
        json!({
            "summary": {
                "total": total,
                "passed": pass_count,
                "pass_rate": pass_rate,
                "total_time_ms": total_time,
                "peak_memory_kb": peak_memory_kb,
                "time_complexity": estimate_time_complexity(total_time),
                "space_complexity": estimate_space_complexity(peak_memory_kb as f64)
            },
            "cases": cases
        })
    }
}

// 分别估算时间和空间复杂度
fn estimate_time_complexity(ms: f64) -> &'static str {
    if ms < 1.0 {
        "O(1)"
    } else if ms < 10.0 {
        "O(log n)"
    } else if ms < 100.0 {
        "O(n)"
    } else if ms < 1000.0 {
        "O(n log n)"
    } else {
        "O(n²)"
    }
}

fn estimate_space_complexity(kb: f64) -> &'static str {
    if kb < 256.0 {
        "O(1)"
    } else if kb < 1024.0 {
        "O(n)"
    } else if kb < 10240.0 {
        "O(nlog n)"
    } else {
        "O(n²)"
    }
}
