# bgustreadimg-wasm 🖼️⚡

**WebAssembly build of `bgustreadimg` optimized for high-performance frontend image preprocessing.**

This package is a WebAssembly port of the native [bgustreadimg](https://github.com/B-GUST/bgustreadimg) library. It brings the power of **O(N) Sauvola adaptive binarization** and **high-quality Lanczos3 resizing** directly to modern web browsers, executing pure Rust at near-native speeds.

Use it to clean, resize, and remove uneven shadows, wrinkles, or background noise from camera-captured document images in the browser before running OCR or sending them to backend pipelines.

---

## 🚀 Installation

```bash
npm install bgustreadimg-wasm
```

---

## 💡 Quick Start

Here is how you can load an image file in the browser, preprocess it using `bgustreadimg-wasm`, and display it in a `<canvas>` element.

```javascript
import init, { preprocessImage } from 'bgustreadimg-wasm';

async function processDocumentImage(imageFile) {
  // 1. Initialize the WebAssembly module
  await init();

  // 2. Read file to ArrayBuffer
  const arrayBuffer = await imageFile.arrayBuffer();
  const inputUint8Array = new Uint8Array(arrayBuffer);

  // 3. Process image (returns Uint8Array of the binarized PNG)
  const windowSize = 25;       // local window size (odd integer >= 3)
  const k = 0.2;               // contrast sensitivity threshold (lower = more aggressive)
  const targetWidth = 800;     // (Optional) target width to resize image (conserving aspect ratio)

  try {
    const outputPngBuffer = preprocessImage(inputUint8Array, windowSize, k, targetWidth);

    // 4. Create image URL and display it
    const blob = new Blob([outputPngBuffer], { type: 'image/png' });
    const imageUrl = URL.createObjectURL(blob);
    
    document.getElementById('output-image').src = imageUrl;
    console.log('Image successfully cleaned in the browser!');
  } catch (error) {
    console.error('Error processing image:', error);
  }
}
```

---

## 🧠 Hybrid Frontend OCR/Layout Inherence

The Rust `bgustreadimg` native backend supports ONNX-based OCR and document layout analysis. In browser environments, running native ONNX engines via Rust WASM is slow and resource-heavy. 

Instead, the recommended hybrid approach is:

1.  **Preprocessing (sauvola binarization & resizing):** Execute client-side using `bgustreadimg-wasm` to clean the image inside the browser in milliseconds.
2.  **Inference (OCR & Layout):** Feed the clean output buffer from `bgustreadimg-wasm` directly into the JavaScript [onnxruntime-web](https://www.npmjs.com/package/onnxruntime-web) package, which utilizes browser WebGPU or WebGL acceleration for fast, native-speed model execution.

---

## ⚙️ API Configuration

### `preprocessImage(data, windowSize, k, targetWidth)`

| Parameter | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `data` | `Uint8Array` | *(Required)* | Binary data buffer of the input image (JPEG, PNG, etc.). |
| `windowSize` | `number` | `25` | Local analysis window size (must be odd and $\ge 3$). |
| `k` | `number` | `0.2` | Sauvola contrast sensitivity parameter. Smaller values yield more aggressive shadow removal. |
| `targetWidth` | `number` | `1280` | Maximum width resolution. If the input image is wider, it is scaled down using Lanczos3 resizing. |

---

## 📜 License

Distributed under the MIT License. See original repository for details.
