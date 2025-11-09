FROM golang:1.21

# 安装必要的工具
RUN apt-get update && apt-get install -y \
    time \
    bash \
    && rm -rf /var/lib/apt/lists/*

# 设置环境变量
ENV GO111MODULE=on \
    CGO_ENABLED=0 \
    GOOS=linux \
    GOARCH=amd64

# 创建工作目录
WORKDIR /tmp

# 保持容器运行
CMD ["tail", "-f", "/dev/null"]