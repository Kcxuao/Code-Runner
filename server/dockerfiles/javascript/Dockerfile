FROM node:20-slim

# 安装必要的工具
RUN apt-get update && apt-get install -y \
    time \
    bash \
    && rm -rf /var/lib/apt/lists/*

# 创建工作目录
WORKDIR /tmp

# 保持容器运行
CMD ["tail", "-f", "/dev/null"]