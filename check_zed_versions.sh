#!/bin/bash
# Test different Zed versions for core-graphics compatibility

VERSIONS=(
    "v0.123.2"
    "v0.115.3-pre"
    "v0.86.1-pre"
    "v0.68.2"
)

for version in "${VERSIONS[@]}"; do
    echo "=== Testing Zed $version ==="

    # Create temp Cargo.toml
    cat > /tmp/test-cargo.toml << EOF
[package]
name = "test"
version = "0.1.0"
edition = "2021"

[dependencies]
gpui = { git = "https://github.com/zed-industries/zed", tag = "$version" }
EOF

    # Check what core-text version it uses
    echo "Checking core-text dependency for $version..."
    cd /tmp && cargo update --manifest-path /tmp/test-cargo.toml 2>&1 | grep -E "(core-text|core-graphics)" || echo "No match"
    echo ""
done
