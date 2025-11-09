<template>
  <div class="flex flex-col min-h-screen bg-gray-900 p-4 text-gray-100">
    <!-- 顶部栏 -->
    <div class="flex justify-between items-center mb-4">
      <h1 class="text-2xl font-bold">示例题目：两数之和</h1>
      <div class="flex space-x-2">
        <select v-model="language" class="bg-gray-800 border-gray-700 text-gray-100 rounded px-2 py-1">
          <option v-for="(val, key) in languageMap" :key="key" :value="key">
            {{ key.toUpperCase() }}
          </option>
        </select>
        
        <button @click="runCode" class="bg-blue-700 hover:bg-blue-600 px-4 py-2 rounded" :disabled="!!testCaseError">
          运行代码
        </button>
        <button @click="submitCode" class="bg-green-700 hover:bg-green-600 px-4 py-2 rounded">
          提交
        </button>
      </div>
    </div>

    <!-- 主体布局：编辑器 + 右侧信息栏 -->
    <div class="flex flex-1 gap-4">
      <!-- 左侧编辑器 -->
      <div class="flex-1 border rounded overflow-hidden">
        <MonacoEditor v-model="code" :language="languageMap[language]" height="calc(100vh - 180px)" theme="vs-dark"
          @change="val => code = val" :options="editorOptions" />
      </div>

      <!-- 右侧信息栏 -->
      <div class="w-96 bg-gray-800 p-4 rounded shadow overflow-y-auto">
        <h2 class="font-semibold mb-2 text-gray-100">题目描述</h2>
        <p>test-test-test</p>

        <h3 class="font-semibold mt-4 text-gray-100">示例输入输出</h3>
        <pre class="bg-gray-700 p-2 rounded text-gray-100">
输入: nums = [2,7], target = 9
输出: 9
解释: 因为 nums[0] + nums[1] == 9
        </pre>

        <h3 class="font-semibold mt-4 text-gray-100">测试用例</h3>
        <ul v-if="testCases.length > 0">
          <li v-for="(t, i) in testCases" :key="i" class="mb-1">
            {{ i + 1 }}. 输入: {{ t.input }} → 期望: {{ t.expected }}
          </li>
        </ul>
        <p v-else class="text-gray-400">暂无测试用例</p>
      </div>
    </div>

    <!-- 输出面板 -->
    <div class="mt-4 p-2 border rounded bg-gray-800 h-40 overflow-y-auto relative text-gray-100">
      <h3 class="font-semibold mb-2">运行结果</h3>
      <pre class="whitespace-pre-wrap">{{ output || '等待运行...' }}</pre>
      <button @click="copyOutput" class="absolute top-2 right-2 bg-gray-700 hover:bg-gray-600 px-2 py-1 rounded"
        :disabled="!output">
        复制
      </button>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'
import MonacoEditor from 'monaco-editor-vue3'
import axios from 'axios'

// 语言映射
const languageMap = { cpp: 'cpp', python: 'python', c: 'c' }

const language = ref('python')
const code = ref(`def add(a, b):
    return a + b
`)
const testCasesInput = ref('[{"input":[2,7],"expected":9},{"input":[3,2],"expected":5}]')
const output = ref('')
const testCaseError = ref('')
const testCases = ref([])

// Monaco 编辑器配置 - 启用所有代码补全功能
const editorOptions = {
  fontSize: 14,
  fontFamily: 'Fira Code, Monaco, Consolas',
  tabSize: 4,
  insertSpaces: true,
  automaticLayout: true,

  // ✅ 启用代码补全
  suggest: {
    enabled: true,
    maxVisibleSuggestions: 20,
    showWords: true,
    showSnippets: true,
    showIssues: true,
  },

  // ✅ 快速提示（补全延迟时间）
  quickSuggestions: {
    other: true,      // 其他代码
    comments: false,   // 注释
    strings: false,    // 字符串
  },
  quickSuggestionsDelay: 300,  // 300ms 后显示

  // ✅ 参数提示
  parameterHints: {
    enabled: true,
    cycle: true,
  },

  // ✅ 自动补全括号和引号
  autoClosingBrackets: 'always',
  autoClosingQuotes: 'always',
  autoClosingOvertype: 'always',

  // ✅ 自动格式化
  formatOnPaste: true,
  formatOnType: true,

  // ✅ IntelliSense（智能感知）
  wordBasedSuggestions: 'matchingDocuments',
  acceptSuggestionOnCommitCharacter: true,
  acceptSuggestionOnEnter: 'on',


  minimap: { enabled: true },
  scrollBeyondLastLine: false,
  renderWhitespace: 'none',
  cursorBlinking: 'blink',
  cursorSmoothCaretAnimation: 'on',
}

// 测试用例解析 - 添加 immediate: true
watch(testCasesInput, val => {
  try {
    const parsed = JSON.parse(val)
    if (!Array.isArray(parsed)) throw new Error('必须是数组')
    testCases.value = parsed
    testCaseError.value = ''
  } catch (e) {
    testCaseError.value = '❌ 测试用例必须是有效的 JSON 数组'
    testCases.value = []
  }
}, { immediate: true })

const extractFunctionName = (codeStr, lang) => {
  // 去除注释、预处理指令和多余空白
  let cleanCode = codeStr
    .replace(/\/\/.*$/gm, '')                    // 去除单行注释
    .replace(/\/\*[\s\S]*?\*\//g, '')           // 去除多行注释
    .replace(/^\s*#include\s+.*$/gm, '')        // 去除 #include
    .replace(/^\s*#define\s+.*$/gm, '')         // 去除 #define
    .replace(/^\s*#pragma\s+.*$/gm, '')         // 去除 #pragma

  if (lang === 'python') {
    const m = cleanCode.match(/^\s*def\s+(\w+)\s*\(/m)
    return m ? m[1] : ''
  }

  if (lang === 'cpp') {
    // C++: 返回类型 funcName(
    const m = cleanCode.match(/(?:int|void|bool|char|float|double|long|size_t|auto|string|vector\s*<[^>]+>|std::\w+(?:\s*<[^>]+>)?|\w+\s*\*+)\s+(\w+)\s*\(/)
    return m ? m[1] : ''
  }

  if (lang === 'c') {
    // C: 返回类型 funcName(
    const m = cleanCode.match(/(?:int|void|char|float|double|long|size_t|unsigned\s+(?:int|long|char)|\w+\s*\*+)\s+(\w+)\s*\(/)
    return m ? m[1] : ''
  }

  return ''
}
// 运行代码
const runCode = async () => {

  // 验证
  if (testCaseError.value) {
    output.value = testCaseError.value
    return
  }

  if (testCases.value.length === 0) {
    output.value = '❌ 没有测试用例'
    return
  }

  const funcName = extractFunctionName(code.value, language.value)
  if (!funcName) {
    output.value = '❌ 无法识别函数名，请检查代码格式'
    return
  }

  output.value = '⏳ 正在执行...'

  try {
    const resp = await axios.post('http://localhost:3000/run', {
      language: language.value,
      code: code.value,
      function: funcName,
      test_cases: testCases.value
    }, { timeout: 10000 })

    const data = resp.data
    let out = `语言: ${data.language}\n总耗时: ${data.execution_time_ms} ms\n\n`

    if (data.output.error) {
      out += `❌ 执行错误: ${data.output.error}\n`
      output.value = out
      return
    }

    if (data.output.summary) {
      out += `总测试数: ${data.output.summary.total}\n`
      out += `通过数: ${data.output.summary.passed}\n`
      out += `通过率: ${(data.output.summary.pass_rate * 100).toFixed(1)}%\n`
      out += `时间复杂度: ${data.output.summary.time_complexity || '未知'}\n`
      out += `空间复杂度: ${data.output.summary.space_complexity || '未知'}\n\n`
    }

    if (data.output.cases && data.output.cases.length) {
      out += '测试用例详情:\n'
      data.output.cases.forEach((c, i) => {
        out += `#${i + 1} 输入: ${JSON.stringify(c.input)} | 输出: ${c.output} | 期望: ${c.expected} | ${c.passed ? '✅' : '❌'} | 时间: ${c.time_ms}ms\n`
      })
    }

    output.value = out
  } catch (err) {
    console.error(err)
    if (err.code === 'ECONNREFUSED') {
      output.value = '❌ 无法连接到后端服务器 (localhost:3000)，请确保后端已启动'
    } else if (err.response?.data?.error) {
      output.value = `❌ 执行失败: ${err.response.data.error}`
    } else if (err.message === 'timeout of 10000ms exceeded') {
      output.value = '❌ 执行超时，代码可能陷入死循环'
    } else {
      output.value = `❌ 执行失败: ${err.message}`
    }
  }
}

// 提交代码
const submitCode = () => {
  if (testCaseError.value) {
    alert('❌ 请先修复测试用例错误')
    return
  }
  alert('✅ 提交成功！（模拟消息）')
}

// 复制输出
const copyOutput = () => {
  if (output.value) {
    navigator.clipboard.writeText(output.value).then(() => {
      alert('✅ 已复制到剪贴板')
    }).catch(() => {
      alert('❌ 复制失败')
    })
  }
}
</script>