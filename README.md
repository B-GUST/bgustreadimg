# bgustreadimg 🖼️

<p align="center">
  <b>Motor de Preprocesamiento de Imágenes Adaptativo de Alto Rendimiento para Pipelines de OCR.</b><br>
  <i>Elimina sombras, arrugas y variaciones de luz no uniformes en milisegundos — 100% Rust nativo.</i>
</p>

<p align="center">
  <a href="https://crates.io/crates/bgustreadimg"><img src="https://img.shields.io/crates/v/bgustreadimg.svg?style=flat-square" alt="Crates Version"></a>
  <a href="https://www.npmjs.com/package/bgustreadimg"><img src="https://img.shields.io/npm/v/bgustreadimg.svg?style=flat-square" alt="NPM Version"></a>
  <img src="https://img.shields.io/badge/version-0.1.5-orange.svg?style=flat-square" alt="Stable Version">
  <a href="https://github.com/B-GUST/bgustreadimg"><img src="https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square" alt="License"></a>
</p>

---

## 💡 La Visión

`bgustreadimg` es un motor de preprocesamiento de imágenes de nivel industrial construido desde cero en **Rust**. Está diseñado para eliminar el ruido visual en fotografías de documentos —facturas, contratos, capturas de cámara— antes de ser enviadas a motores de OCR. A diferencia de los convertidores de formato convencionales, su núcleo implementa **Binarización Adaptativa de Sauvola** con **Imágenes Integrales (SAT)** para lograr una limpieza uniforme en tiempo lineal O(N), independientemente del tamaño de la ventana de análisis local.

---

## 🌟 Características Clave

*   **Binarización Adaptativa Sauvola O(N):** Umbral de contraste local dinámico usando Summed Area Tables. Elimina sombras, arrugas y fondos no uniformes sin distorsionar los caracteres.
*   **Redimensionamiento Inteligente con Lanczos3:** Escalado de alta calidad que conserva la nitidez del texto. Selección automática del ancho objetivo basada en la memoria RAM disponible.
*   **Detección de Layout Opcional (ONNX):** Módulo `LayoutAnalyzer` basado en Table Transformer para extraer regiones tabulares de documentos escaneados.
*   **Inferencia OCR Opcional (ONNX):** Módulo `OcrEngine` basado en Surya OCR para reconocimiento de texto multilingüe end-to-end.
*   **Bindings NAPI-RS Nativos:** Extensión dinámica `.node` cargada directamente por Node.js sin sobrecoste de IPC ni dependencias Python.
*   **Doble Canal de Distribución:** Biblioteca estática (`rlib`) para Rust en crates.io y bindings dinámicos (`cdylib`) para npm.

---

## 🏗️ Arquitectura del Pipeline

```
                    ┌─────────────────────┐
                    │   Input Image       │
                    │  (JPEG, PNG, ...)   │
                    └─────────┬───────────┘
                              │
                    ┌─────────▼───────────┐
                    │  Metadata Probe     │
                    │  (formato, dims)    │  ── sin decodificar a RAM
                    └─────────┬───────────┘
                              │
                    ┌─────────▼───────────┐
                    │  Decode & Resize    │
                    │  Lanczos3, auto-RAM │
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
                    │  (opcional)         │
                    └─────────┬───────────┘
                              │
                    ┌─────────▼───────────┐
                    │  OCR Inference      │  ── ONNX (surya-ocr)
                    │  (opcional)         │
                    └─────────┬───────────┘
                              │
                    ┌─────────▼───────────┐
                    │  Clean Output PNG   │
                    │  (sin pérdidas)     │
                    └─────────────────────┘
```

---

## 📦 Canales de Distribución

### 1. Canal Rust (Crates.io) 🦀
*   **Tipo:** Biblioteca estática (`rlib`).
*   **Uso:**
    ```toml
    [dependencies]
    bgustreadimg = "0.1.5"
    ```

### 2. Canal Node.js & NPM (Backend) 🟢
*   **Tipo:** Extensión nativa (`cdylib` mediante NAPI-RS).
*   **Instalación:**
    ```bash
    npm install bgustreadimg
    ```

### 3. Canal Python & Pip (Maturin) 🐍
*   **Tipo:** Módulo nativo compilado (PyO3).
*   **Instalación:**
    ```bash
    pip install bgustreadimg
    ```

### 4. Canal Frontend & NPM (WebAssembly) 🌐
*   **Tipo:** Paquete JS/WASM para navegador (`wasm-bindgen`).
*   **Instalación:**
    ```bash
    npm install bgustreadimg-wasm
    ```

---

## 🛠️ Instalación y Compilación de Desarrollo

1.  **Clonar el repositorio:**
    ```bash
    git clone https://github.com/B-GUST/bgustreadimg.git
    cd bgustreadimg
    ```

2.  **Compilar para Node.js (NAPI-RS):**
    ```bash
    npm install
    npm run build
    ```

3.  **Compilar para Python (Maturin):**
    ```bash
    # Requiere instalar maturin
    pip install maturin
    PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin build --release
    ```

4.  **Compilar para Frontend/Navegador (WASM):**
    ```bash
    # Compila a WASM y prepara el paquete listo para npm en pkg-wasm/
    npm run build:wasm
    ```

---

## 🚀 Primeros Pasos

### Rust
```rust
use bgustreadimg::preprocess_image_rs;

let image_data = std::fs::read("input.jpg").unwrap();
let result = preprocess_image_rs(image_data, Some(
    bgustreadimg::PreprocessConfigRs {
        window_size: Some(25),
        k: Some(0.2),
        target_width: Some(1920),
    }
)).await.unwrap();

std::fs::write("output.png", result).unwrap();
```

### Node.js (Backend)
```javascript
const { preprocessImage } = require('bgustreadimg');
const fs = require('fs');

const clean = await preprocessImage(fs.readFileSync('input.jpg'), {
    windowSize: 25,
    k: 0.2,
    targetWidth: 1920,
});
fs.writeFileSync('output.png', clean);
```

### Python
```python
import bgustreadimg

with open("input.jpg", "rb") as f:
    data = f.read()

config = bgustreadimg.PreprocessConfigPy(window_size=25, k=0.2, target_width=1920)
clean = bgustreadimg.preprocess_image(data, config)

with open("output.png", "wb") as f:
    f.write(clean)
```

### Frontend (Navegador/WASM)
```javascript
import init, { preprocessImage } from 'bgustreadimg-wasm';

await init(); // Inicializar módulo WASM

const fileBuffer = await file.arrayBuffer();
const cleanBuffer = preprocessImage(new Uint8Array(fileBuffer), 25, 0.2, 1280);
```

---

## ⚙️ Configuración

| Parámetro     | Default | Descripción |
|---------------|---------|-------------|
| `windowSize`  | `25`    | Tamaño de la ventana local de análisis (impar, ≥3) |
| `k`           | `0.2`   | Sensibilidad al contraste (menor = más agresivo con sombras) |
| `targetWidth` | auto    | Ancho máximo de salida; auto-selecciona 1920 o 1280 según RAM libre |

---

## 🧩 Estructura del Proyecto

```
├── Cargo.toml          # Manifiesto Rust (publicable en crates.io)
├── pyproject.toml      # Manifiesto Python (publicable con maturin)
├── package.json        # Manifiesto npm
├── build.rs            # Script de compilación condicional
├── scripts/
│   └── prepare-wasm-pkg.js # Script de post-procesamiento para WASM
├── docs/
│   ├── README_WASM.md  # README del paquete frontend/WASM
│   ├── updated_multi_platform_plan.md # Plan de arquitectura multi-plataforma
│   └── implementation_report.md # Reporte de cambios realizados
├── src/
│   ├── lib.rs          # Núcleo: Sauvola threshold, preprocess_image_sync
│   ├── layout.rs       # LayoutAnalyzer — detección de tablas con ONNX
│   ├── ocr.rs          # OcrEngine — reconocimiento de texto con ONNX
│   ├── bindings_napi.rs # Bindings específicos para Node.js
│   ├── bindings_pyo3.rs # Bindings específicos para Python
│   └── bindings_wasm.rs # Bindings específicos para WebAssembly
├── index.js            # Binding NAPI-RS para Node.js (auto-generado)
├── index.d.ts          # Declaraciones de tipos TypeScript para Node.js
├── models/             # Modelos ONNX (gitignored, descarga bajo demanda)
└── LICENSE             # Licencia MIT
```

---

## 📜 Licencia y Créditos

Este proyecto se distribuye bajo la licencia MIT. Consulta el archivo [`CREDITS.md`](./CREDITS.md) para atribuciones al algoritmo de Sauvola y las librerías de terceros.
