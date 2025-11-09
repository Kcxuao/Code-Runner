#!/bin/bash

echo "🛑 Stopping all containers..."

containers=("gcc-test" "gpp-test" "python-test" "rust-test" "go-test" "java-test" "javascript-test")

for container in "${containers[@]}"; do
    if docker ps -a --format '{{.Names}}' | grep -q "^${container}$"; then
        echo "Stopping $container..."
        docker stop $container
        docker rm $container
        echo "✅ $container removed"
    fi
done

echo "🎉 All containers stopped and removed!"