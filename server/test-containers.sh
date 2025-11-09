#!/bin/bash

echo "🔍 Testing all containers..."

test_container() {
    local name=$1
    local cmd=$2
    
    echo "Testing $name..."
    if docker exec $name bash -c "$cmd" > /dev/null 2>&1; then
        echo "✅ $name is working"
    else
        echo "❌ $name failed"
    fi
}

test_container "gcc-test" "gcc --version"
test_container "gpp-test" "g++ --version"
test_container "python-test" "python3 --version"
test_container "rust-test" "rustc --version"
test_container "go-test" "go version"
test_container "java-test" "javac -version"
test_container "javascript-test" "node --version"

echo "🎉 Testing completed!"