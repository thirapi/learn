#!/usr/bin/env bash
set -e

cd "$(dirname "$0")/.."

echo "🔨 Mengompilasi library WebAssembly (wasm32-unknown-unknown)..."
cargo +nightly build --target wasm32-unknown-unknown --lib --release

OUT_DIR="target/web"
mkdir -p "$OUT_DIR"

echo "📦 Menjalankan wasm-bindgen untuk membundel modul Web..."
wasm-bindgen --target web --out-dir "$OUT_DIR" --out-name app target/wasm32-unknown-unknown/release/learn.wasm

echo "🌐 Menyiapkan index.html WebGPU container..."
cat << 'EOF' > "$OUT_DIR/index.html"
<!doctype html>
<html lang="id">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>DiskritLearn • Platform Belajar Matematika Diskrit (Rune WebGPU)</title>
  <style>
    html, body {
      margin: 0;
      padding: 0;
      width: 100%;
      height: 100%;
      background-color: #121417;
      overflow: hidden;
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      color: #f0f2f5;
    }
    #status {
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      font-size: 16px;
      color: #8b949e;
      text-align: center;
    }
    .spinner {
      border: 3px solid rgba(255,255,255,0.1);
      border-top: 3px solid #58a6ff;
      border-radius: 50%;
      width: 32px;
      height: 32px;
      animation: spin 1s linear infinite;
      margin: 0 auto 12px auto;
    }
    @keyframes spin { 0% { transform: rotate(0deg); } 100% { transform: rotate(360deg); } }
  </style>
</head>
<body>
  <div id="status">
    <div class="spinner"></div>
    <div>Memuat DiskritLearn (WebAssembly + WebGPU)...</div>
  </div>
  <script type="module">
    const ver = Date.now();
    try {
      const { default: init } = await import(`./app.js?v=${ver}`);
      await init(`./app_bg.wasm?v=${ver}`);
      const st = document.getElementById('status');
      if (st) st.remove();
    } catch (err) {
      document.getElementById('status').innerHTML = `<p style="color:#f85149">WebGPU Error: ${err.message || err}</p><p style="font-size:12px;color:#8b949e">Pastikan browsermu mendukung WebGPU (Chrome 113+, Edge 113+, atau Firefox WebGPU flag enabled).</p>`;
      console.error(err);
    }
  </script>
</body>
</html>
EOF

echo "🔤 Menyalin font Inter web..."
cp ../rune/crates/rune/assets/web/Inter-*.ttf "$OUT_DIR/" 2>/dev/null || true
mkdir -p "$OUT_DIR/target"
ln -sfn .. "$OUT_DIR/target/web" 2>/dev/null || true

echo "✅ Web build berhasil dibuat di target/web!"
echo "👉 Jalankan './scripts/serve-web.sh' untuk membuka di browser lokal!"
