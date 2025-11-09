use crate::codegen::CodeGenerator;
use crate::model::request::TestCase;
use anyhow::Result;

pub struct CppCodeGenerator;

impl CodeGenerator for CppCodeGenerator {
    fn generate(&self, code: &str, function: &str, test_cases: &[TestCase]) -> Result<String> {
        let mut full = String::with_capacity(code.len() + test_cases.len() * 300 + 200);

        full.push_str("#include <iostream>\n");
        full.push_str("#include <chrono>\n");
        full.push_str("#include <sys/resource.h>\n");
        full.push_str("using namespace std;\n");
        full.push_str("using namespace std::chrono;\n\n");
        full.push_str(code);
        full.push_str("\n\nlong get_memory_kb() {\n");
        full.push_str("    struct rusage usage;\n");
        full.push_str("    getrusage(RUSAGE_SELF, &usage);\n");
        full.push_str("    return usage.ru_maxrss;\n");
        full.push_str("}\n\n");

        full.push_str("int main() {\n");
        full.push_str("    auto program_start = steady_clock::now();\n");
        full.push_str("    long start_mem = get_memory_kb();\n\n");

        for t in test_cases {
            let args = t
                .input
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            full.push_str("    {\n");
            full.push_str("        auto start = steady_clock::now();\n");
            full.push_str(&format!("        int result = {}({});\n", function, args));
            full.push_str("        auto end = steady_clock::now();\n");
            full.push_str("        double time_ms = duration_cast<microseconds>(end - start).count() / 1000.0;\n");
            full.push_str(&format!(
                "        cout << \"Input: ({}) => Output: \" << result << \" | Expected: {} | \" \
                 << (result == {} ? \"✅ PASS\" : \"❌ FAIL\") << \" | Time: \" << time_ms << \"ms\" << endl;\n",
                args, t.expected, t.expected
            ));
            full.push_str("    }\n\n");
        }

        full.push_str("    auto program_end = steady_clock::now();\n");
        full.push_str("    long end_mem = get_memory_kb();\n");
        full.push_str("    double total_time = duration_cast<microseconds>(program_end - program_start).count() / 1000.0;\n");
        full.push_str("    cout << \"\\n=== Performance ===\" << endl;\n");
        full.push_str("    cout << \"Time: \" << total_time << \"ms\" << endl;\n");
        full.push_str("    cout << \"Memory: \" << end_mem << \" KB\" << endl;\n");

        full.push_str("    return 0;\n");
        full.push('}');

        Ok(full)
    }
}