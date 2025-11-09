use crate::codegen::CodeGenerator;
use crate::model::request::TestCase;
use anyhow::Result;

pub struct PythonCodeGenerator;

impl CodeGenerator for PythonCodeGenerator {
    fn generate(&self, code: &str, function: &str, test_cases: &[TestCase]) -> Result<String> {
        let mut full = String::with_capacity(code.len() + test_cases.len() * 200 + 200);

        full.push_str("import time\n");
        full.push_str("import resource\n\n");
        full.push_str(code);
        full.push_str("\n\n");

        full.push_str("def get_memory_kb():\n");
        full.push_str("    return resource.getrusage(resource.RUSAGE_SELF).ru_maxrss\n\n");

        full.push_str("if __name__ == '__main__':\n");
        full.push_str("    program_start = time.time()\n");
        full.push_str("    start_mem = get_memory_kb()\n\n");

        for t in test_cases {
            let args = t
                .input
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            full.push_str("    start = time.time()\n");
            full.push_str(&format!("    result = {}({})\n", function, args));
            full.push_str("    end = time.time()\n");
            full.push_str("    time_ms = (end - start) * 1000\n");
            full.push_str(&format!(
                "    status = '✅ PASS' if result == {} else '❌ FAIL'\n",
                t.expected
            ));
            full.push_str(&format!(
                "    print(f\"Input: ({}) => Output: {{result}} | Expected: {} | {{status}} | Time: {{time_ms:.3f}}ms\")\n\n",
                args, t.expected
            ));
        }

        full.push_str("    program_end = time.time()\n");
        full.push_str("    end_mem = get_memory_kb()\n");
        full.push_str("    total_time = (program_end - program_start) * 1000\n");
        full.push_str("    print(\"\\n=== Performance ===\")\n");
        full.push_str("    print(f\"Time: {total_time:.3f}ms\")\n");
        full.push_str("    print(f\"Memory: {end_mem} KB\")\n");

        Ok(full)
    }
}