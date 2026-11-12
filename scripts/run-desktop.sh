#!/usr/bin/env bash
set -e
cd "$(dirname "$0")/.."
echo "🚀 Meluncurkan DiskritLearn (Native Linux Desktop 120 FPS)..."
cargo +nightly run
