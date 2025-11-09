use crate::codegen::CodeGenerator;
use crate::model::request::TestCase;
use anyhow::Result;

pub struct CCodeGenerator;

impl CodeGenerator for CCodeGenerator {
    fn generate(&self, code: &str, function: &str, test_cases: &[TestCase]) -> Result<String> {
        let mut full = String::with_capacity(code.len() + test_cases.len() * 250 + 200);

        full.push_str("#include <stdio.h>\n");
        full.push_str("#include <time.h>\n");
        full.push_str("#include <sys/resource.h>\n\n");
        full.push_str(code);
        full.push_str("\n\nlong get_memory_kb() {\n");
        full.push_str("    struct rusage usage;\n");
        full.push_str("    getrusage(RUSAGE_SELF, &usage);\n");
        full.push_str("    return usage.ru_maxrss;\n");
        full.push_str("}\n\n");

        full.push_str("int main() {\n");
        full.push_str("    clock_t program_start = clock();\n");
        full.push_str("    long start_mem = get_memory_kb();\n\n");

        for t in test_cases {
            let args = t
                .input
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            full.push_str("    {\n");
            full.push_str("        clock_t start = clock();\n");
            full.push_str(&format!("        int result = {}({});\n", function, args));
            full.push_str("        clock_t end = clock();\n");
            full.push_str(
                "        double time_ms = (double)(end - start) / CLOCKS_PER_SEC * 1000.0;\n",
            );
            full.push_str(&format!(
                "        printf(\"Input: ({}) => Output: %d | Expected: {} | %s | Time: %.3fms\\n\", \
                 result, result == {} ? \"✅ PASS\" : \"❌ FAIL\", time_ms);\n",
                args, t.expected, t.expected
            ));
            full.push_str("    }\n\n");
        }

        full.push_str("    clock_t program_end = clock();\n");
        full.push_str("    long end_mem = get_memory_kb();\n");
        full.push_str("    double total_time = (double)(program_end - program_start) / CLOCKS_PER_SEC * 1000.0;\n");
        full.push_str("    printf(\"\\n=== Performance ===\\n\");\n");
        full.push_str("    printf(\"Time: %.3fms\\n\", total_time);\n");
        full.push_str("    printf(\"Memory: %ld KB\\n\", end_mem - start_mem);\n");

        full.push_str("    return 0;\n");
        full.push('}');

        Ok(full)
    }
}