#!/usr/bin/env bash
set -e

cd "$(dirname "$0")/.."

if [[ ! -f "target/web/app.js" ]]; then
  echo "⚠️ Belum ada build web. Menjalankan build terlebih dahulu..."
  ./scripts/build-web.sh
fi

echo "🚀 Menjalankan local server di http://localhost:8000 ..."
echo "Buka http://localhost:8000 di browser Chrome / Firefox / Edge!"
python3 -m http.server 8000 -d target/web
