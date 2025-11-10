# 🚀 Code Runner - 多语言代码在线执行平台

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Vue](https://img.shields.io/badge/Vue-3.x-green.svg)](https://vuejs.org/)
[![Docker](https://img.shields.io/badge/Docker-required-blue.svg)](https://www.docker.com/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

一个基于 Rust + Vue 3 的高性能在线代码执行平台，支持多种编程语言的实时编译、运行和性能分析。

## ✨ 特性

- 🎯 **多语言支持**: C, C++, Python, Rust, Go, Java, JavaScript
- ⚡ **高性能**: 使用 Docker 容器隔离，复用连接池，毫秒级响应
- 🔒 **安全隔离**: 每个代码在独立容器中执行，防止恶意代码
- 📊 **性能分析**: 自动测量执行时间、内存使用和复杂度分析
- 🧪 **测试驱动**: 支持多个测试用例并行执行
- 🎨 **现代UI**: Vue 3 + TailwindCSS + Monaco Editor
- 🛠️ **配置驱动**: 通过 TOML 配置文件轻松扩展语言支持

## 📸 截图
```
┌─────────────────────────────────────────────────────────────┐
│  代码编辑器           │  测试用例      │  执行结果            │
│  ┌──────────────┐    │  ┌──────────┐ │  ┌────────────┐      │
│  │ int add() {  │    │  │ 1, 2 → 3 │ │  │ ✓ 2/2 通过 │      │
│  │   return a+b;│    │  │ 5, 7 → 12│ │  │ 时间: 2ms  │      │
│  │ }            │    │  └──────────┘ │  │ 内存: 1MB  │      │
│  └──────────────┘    │               │  └────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

## 🏗️ 架构
```
┌─────────────┐      HTTP       ┌──────────────┐
│   Vue 3     │ ────────────→   │  Axum Server │
│  Frontend   │                 │   (Rust)     │
└─────────────┘                 └──────┬───────┘
                                       │ Docker API
                                       ↓
                        ┌──────────────────────────┐
                        │   Docker Containers      │
                        ├──────────┬───────────────┤
                        │ gcc-test │ python-test   │
                        │ gpp-test │ rust-test     │
                        │ ...      │ ...           │
                        └──────────┴───────────────┘
```

## 🚀 快速开始

### 前置要求

- Rust 1.75+
- Node.js 18+
- Docker 20.10+
- Docker Daemon 开启远程 API (端口 2375)

### 后端部署

#### 1. 克隆项目
```bash
git clone https://github.com/Kcxuao/run_coder.git
cd run_coder
```

#### 2. 配置 Docker

确保 Docker Daemon 开启远程 API：
```bash
# 编辑 Docker daemon 配置
sudo nano /etc/docker/daemon.json

# 添加以下内容
{
  "hosts": ["unix:///var/run/docker.sock", "tcp://0.0.0.0:2375"]
}

# 重启 Docker
sudo systemctl restart docker
```

#### 3. 构建语言容器
```bash
cd dockerfiles
chmod +x build-all.sh start-all.sh
./build-all.sh    # 构建所有语言镜像
./start-all.sh    # 启动所有容器
```

或使用 Docker Compose：
```bash
docker-compose up -d
```

#### 4. 配置后端

编辑 `config.toml`：
```toml
[docker]
host = "http://YOUR_DOCKER_HOST_IP:2375"  # 替换为你的 Docker 主机 IP
```

#### 5. 运行后端
```bash
cargo build --release
cargo run --release

# 或直接运行
./target/release/run_code
```

服务将在 `http://0.0.0.0:3000` 启动

### 前端部署
```bash
cd web

# 安装依赖
pnpm install

# 开发模式
pnpm dev

# 生产构建
pnpm build
```

前端将在 `http://localhost:5173` 启动

## 📝 使用示例

### API 调用
```bash
curl -X POST http://localhost:3000/run \
  -H "Content-Type: application/json" \
  -d '{
    "language": "python",
    "code": "def add(a, b):\n    return a + b",
    "function": "add",
    "test_cases": [
      {"input": [1, 2], "expected": 3},
      {"input": [5, 7], "expected": 12}
    ]
  }'
```

### 返回结果
```json
{
  "language": "python",
  "output": {
    "summary": {
      "total": 2,
      "passed": 2,
      "pass_rate": 1.0,
      "total_time_ms": 0.523,
      "peak_memory_kb": 8192,
      "time_complexity": "O(1)",
      "space_complexity": "O(1)"
    },
    "cases": [
      {
        "input": "1, 2",
        "output": "3",
        "expected": "3",
        "passed": true,
        "time_ms": 0.012
      }
    ]
  },
  "execution_time_ms": 156
}
```

## 🔧 配置说明

### config.toml
```toml
[server]
bind_addr = "0.0.0.0:3000"

[docker]
host = "http://10.211.55.8:2375"
connect_timeout = 3
keepalive_secs = 60
request_timeout = 120

[containers]
c = "gcc-test"
python = "python-test"
# ... 更多语言

[languages.c]
enabled = true
file_extension = "c"
compiler = "gcc"
compile_cmd = "gcc {source} -o {binary} -Wall -O2 2>&1"
run_cmd = "{binary}"
```

### 添加新语言

1. 创建 Dockerfile：
```dockerfile
# dockerfiles/newlang/Dockerfile
FROM newlang:latest
RUN apt-get update && apt-get install -y time bash
CMD ["tail", "-f", "/dev/null"]
```

2. 更新配置：
```toml
[containers]
newlang = "newlang-test"

[languages.newlang]
enabled = true
file_extension = "nl"
compiler = "nlc"
compile_cmd = "nlc {source} -o {binary}"
run_cmd = "{binary}"
```

3. 创建代码生成器：
```rust
// src/codegen/newlang_generator.rs
pub struct NewLangCodeGenerator;

impl CodeGenerator for NewLangCodeGenerator {
    fn generate(&self, code: &str, function: &str, test_cases: &[TestCase]) -> Result<String> {
        // 实现代码生成逻辑
    }
}
```

## 📊 性能优化

### 已实现的优化

- ✅ 全局 Docker 客户端连接池
- ✅ HTTP Keep-Alive 长连接
- ✅ 正则表达式预编译
- ✅ 容器复用，避免重复创建
- ✅ 直接使用 `clock()` 而非 `/usr/bin/time`
- ✅ 字符串预分配容量

### 性能指标

| 操作 | 耗时 |
|------|------|
| 代码生成 | < 1ms |
| 文件写入 | ~2ms |
| 编译 (C) | ~50ms |
| 执行 | ~5ms |
| 总耗时 | **~60-200ms** |

## 🔒 安全考虑

### 当前措施

- ✅ Docker 容器隔离
- ✅ 临时文件唯一命名 (UUID)
- ✅ 执行后自动清理
- ⚠️ Heredoc 注入风险（待修复）

### 建议改进

1. 使用 Docker API 上传文件，避免 shell 注入
2. 添加执行超时限制
3. 限制容器资源使用 (CPU/内存)
4. 实现速率限制
5. 添加用户认证

## 🛠️ 开发

### 项目结构
```
run_code/
├── src/
│   ├── main.rs              # 入口
│   ├── api/                 # API 路由
│   │   └── run_code.rs
│   ├── codegen/             # 代码生成器
│   │   ├── c_generator.rs
│   │   ├── cpp_generator.rs
│   │   └── python_generator.rs
│   ├── config/              # 配置管理
│   ├── docker/              # Docker 客户端
│   ├── executor/            # 代码执行器
│   ├── model/               # 数据模型
│   ├── parser/              # 输出解析
│   └── utils/               # 工具函数
├── dockerfiles/             # Docker 镜像
│   ├── c/
│   ├── cpp/
│   └── python/
├── config.toml              # 配置文件
└── Cargo.toml
```

### 运行测试
```bash
# 后端测试
cargo test

# 容器测试
cd dockerfiles
./test-containers.sh
```

### 调试模式
```bash
# 启用详细日志
RUST_LOG=debug cargo run
```

## 📋 TODO

- [ ] 修复 shell 注入漏洞
- [ ] 添加执行超时控制
- [ ] 实现容器池管理
- [ ] 支持标准输入/输出
- [ ] 支持复杂数据类型测试
- [ ] 添加速率限制
- [ ] WebSocket 实时输出
- [ ] 代码历史记录
- [ ] 用户系统和权限管理
- [ ] 代码分享功能

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

---

⭐ 如果这个项目对你有帮助，请给它一个 Star！
```

## 额外文件

### LICENSE
```
MIT License

Copyright (c) 2024 Your Name

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
