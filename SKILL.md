---
name: bgustreadimg
description: High-performance adaptive image preprocessing engine for document cleaning and Sauvola binarization.
tools:
  - name: preprocess_image
    description: Preprocess document images (JPEG, PNG, etc.) to remove wrinkles, shadows, and uneven lighting using O(N) Sauvola binarization and Lanczos3 resizing, making them highly readable for OCR engines.
    arguments:
      input_path:
        type: string
        description: The absolute or relative path to the source document image file.
        required: true
      output_path:
        type: string
        description: The path where the cleaned binarized PNG image should be saved.
        required: true
      window_size:
        type: number
        description: Local window size for adaptive analysis. Must be an odd integer >= 3. Defaults to 25.
        required: false
      k:
        type: number
        description: Sauvola contrast sensitivity parameter (range 0.0 to 1.0). Smaller values yield more aggressive shadow removal. Defaults to 0.2.
        required: false
      target_width:
        type: number
        description: Target width to resize the image to, maintaining aspect ratio. Defaults to auto-detection (1280 or 1920) based on free memory.
        required: false
---

# Claude Agent Skill: bgustreadimg

`bgustreadimg` is a document-cleaning and OCR image preprocessor skill for AI Agents. It implements adaptive local thresholding (Sauvola algorithm) using Summed Area Tables (SAT) in linear $O(N)$ complexity to clean up scanning artifacts (shadows, background noise, uneven lighting) at native speed.

## 🛠️ Tool Integration Guide

### 1. `preprocess_image`
Use this tool whenever the user provides an image scan or photograph of a document (e.g. invoice, contract, receipt) that has bad lighting, dark spots, wrinkles, or shadows, and needs to be prepared for text extraction (OCR).

* **Usage Example:**
  ```javascript
  // Preprocessing a shadowy document photo:
  await tools.preprocess_image({
    input_path: "./receipts/dark_photo.jpg",
    output_path: "./receipts/clean_receipt.png",
    window_size: 25,
    k: 0.15,
    target_width: 800
  });
  ```

## 🧠 Instruction Set for Claude

When executing this skill, adhere to the following rules:

1. **OCR Pre-cleaning:** Always run `preprocess_image` on photos of documents before feeding them to OCR engines like Tesseract. OCR engines perform significantly better on clean, binarized black-and-white text than on colored/shadowed images.
2. **K Parameter Tuning:**
   * If the input image has very dark shadows covering parts of the text, decrease the `k` parameter (e.g. to `0.1` or `0.15`) for more aggressive thresholding.
   * If text strokes are too thin or breaking apart, increase `k` (e.g. to `0.3` or `0.35`) to preserve character strokes.
3. **WASM vs Native Execution:** 
   * For backend and CLI automation, run the tool natively.
   * For frontend, direct the user to load the `bgustreadimg-wasm` package in their browser so that the image is cleaned client-side.
