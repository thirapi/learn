#!/usr/bin/env bash
set -e
cd "$(dirname "$0")/.."
echo "🚀 Meluncurkan Learn (Native Linux Desktop 120 FPS)..."
cargo +nightly run
