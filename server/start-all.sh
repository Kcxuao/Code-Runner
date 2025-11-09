#!/bin/bash

set -e

echo "🚀 Starting all language containers..."

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

# 🧩 通用运行参数（安全限制）
DOCKER_OPTS=(
  --detach
  --restart=unless-stopped
  --network=none
  --read-only
  --pids-limit=128
  --cpus="0.5"
  --memory="256m"
  --cap-drop=ALL
  --security-opt=no-new-privileges
  --user=nobody
)

# 🧱 容器配置表（语言 => 镜像名）
declare -A CONTAINERS=(
  ["c"]="gcc-test"
  ["cpp"]="gpp-test"
  ["python"]="python-test"
  ["rust"]="rust-test"
  ["go"]="go-test"
  ["java"]="java-test"
  ["js"]="javascript-test"
)

start_container() {
    local lang=$1
    local image=${CONTAINERS[$lang]}
    local name="${lang}-sandbox"

    if [ -z "$image" ]; then
        echo -e "${YELLOW}⚠️  Skipping unknown language: $lang${NC}"
        return
    fi

    # 检查容器是否已存在
    if docker ps -a --format '{{.Names}}' | grep -q "^${name}$"; then
        echo -e "${YELLOW}Container $name already exists, removing...${NC}"
        docker rm -f "$name" >/dev/null
    fi

    echo -e "${BLUE}Starting $name ($image)...${NC}"
    docker run "${DOCKER_OPTS[@]}" --name "$name" "$image"
    echo -e "${GREEN}✅ $name started${NC}\n"
}

# 🚀 启动启用的语言容器
for lang in "${!CONTAINERS[@]}"; do
    start_container "$lang"
done

echo -e "${GREEN}🎉 All sandboxes started successfully!${NC}"
echo ""
echo "📋 Running containers:"
docker ps --format "table {{.Names}}\t{{.Image}}\t{{.Status}}"
