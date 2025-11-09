#!/bin/bash

set -e

echo "🐳 Building all Docker images..."

# 颜色输出
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

build_image() {
    local lang=$1
    local tag=$2
    
    echo -e "${BLUE}Building $lang image...${NC}"
    docker build -t $tag -f dockerfiles/$lang/Dockerfile dockerfiles/$lang/
    echo -e "${GREEN}✅ $lang image built successfully${NC}\n"
}

# 构建所有镜像
build_image "c" "gcc-test"
build_image "cpp" "gpp-test"
build_image "python" "python-test"
build_image "rust" "rust-test"
build_image "go" "go-test"
build_image "java" "java-test"
build_image "javascript" "javascript-test"

echo -e "${GREEN}🎉 All images built successfully!${NC}"