# bgustreadimg

**Adaptive image preprocessing engine for OCR pipelines.** Written in Rust with optional Node.js native bindings via NAPI-RS.

---

## Architecture

```
                    ┌─────────────────────┐
                    │   Input Image       │
                    │  (JPEG, PNG, ...)   │
                    └─────────┬───────────┘
                              │
                    ┌─────────▼───────────┐
                    │  Metadata Probe     │
                    │  (format, dims)     │
                    └─────────┬───────────┘
                              │
                    ┌─────────▼───────────┐
                    │  Decode & Resize    │
                    │  Lanczos3, O(1) RAM │
                    └─────────┬───────────┘
                              │
                    ┌─────────▼───────────┐
                    │  Sauvola Adaptive   │
                    │  Binarization (SAT) │
                    │  O(N), window_size  │
                    └─────────┬───────────┘
                              │
                    ┌─────────▼───────────┐
                    │  Layout Detection   │  ── ONNX (table-transformer)
                    │  (optional)         │
                    └─────────┬───────────┘
                              │
                    ┌─────────▼───────────┐
                    │  OCR Inference      │  ── ONNX (surya-ocr)
                    │  (optional)         │
                    └─────────┬───────────┘
                              │
                    ┌─────────▼───────────┐
                    │  Clean Output PNG   │
                    └─────────────────────┘
```

The core pipeline:
1. **Probe** — reads image metadata without decoding the full bitmap into RAM
2. **Resize** — Lanczos3 downscale to a target width (auto-selected based on available RAM)
3. **Sauvola Binarization** — O(N) using Summed Area Tables; removes shadows, wrinkles, and non-uniform lighting
4. **Layout detection** *(optional)* — ONNX model (table-transformer) for table region extraction
5. **OCR inference** *(optional)* — ONNX model (surya-ocr) for end-to-end text recognition
6. **Output** — lossless PNG

---

## Project Structure

```
├── Cargo.toml          # Rust crate manifest (publishable to crates.io)
├── build.rs            # NAPI-RS build script
├── src/
│   ├── lib.rs          # Core: Sauvola threshold, preprocess_image, NAPI bindings
│   ├── layout.rs       # LayoutAnalyzer — ONNX table detection
│   └── ocr.rs          # OcrEngine — ONNX text recognition
├── index.js            # Auto-generated NAPI-RS JS binding (entry point for npm)
├── index.d.ts          # TypeScript type declarations
├── models/             # ONNX model files (gitignored, downloaded on demand)
│   ├── sury-ocr/
│   └── table-transformer/
└── package.json        # npm package manifest
```

---

## Getting Started

### As a Rust crate

```toml
[dependencies]
bgustreadimg = "0.1"
```

```rust
use bgustreadimg::preprocess_image_rs;

let image_data = std::fs::read("input.jpg").unwrap();
let config = Some(bgustreadimg::PreprocessConfigRs {
    window_size: Some(25),
    k: Some(0.2),
    target_width: Some(1920),
});

let result = preprocess_image_rs(image_data, config).await.unwrap();
std::fs::write("output.png", result).unwrap();
```

### As an npm package

```bash
npm install bgustdown-img
```

```javascript
const { preprocessImage } = require('bgustdown-img');
const fs = require('fs');

const clean = await preprocessImage(fs.readFileSync('input.jpg'), {
    windowSize: 25,
    k: 0.2,
    targetWidth: 1920,
});
fs.writeFileSync('output.png', clean);
```

### Build from source

```bash
# Rust library only
cargo build --release

# With Node.js bindings
npm install
npm run build
```

---

## Configuration

| Parameter     | Default | Description |
|---------------|---------|-------------|
| `windowSize`  | `25`    | Local analysis window size (odd, ≥3) |
| `k`           | `0.2`   | Contrast sensitivity (lower = more aggressive shadow removal) |
| `targetWidth` | auto    | Max output width; auto-picks 1920 or 1280 based on free RAM |

---

## License

MIT
