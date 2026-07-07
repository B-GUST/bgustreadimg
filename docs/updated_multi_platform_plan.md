# Plan de Arquitectura Multi-Distribución y Escalabilidad: `bgustreadimg`

Este documento describe la estrategia técnica y el plan de acción para estructurar el proyecto [bgustreadimg](file:///home/august/code/bgustecosystem/bgustreadimg) de tal manera que funcione en múltiples plataformas (Rust, Node.js/NPM, Python/PyPI, y WebAssembly/Frontend) desde un único repositorio monolítico, garantizando escalabilidad sin conflictos de dependencias.

---

## 🛠️ 1. Arquitectura de Dependencias y Compilación Cruzada

Para evitar que dependencias nativas del sistema operativo (como `sysinfo` u `ort`/ONNX Runtime) rompan la compilación para la Web (WebAssembly), estructuraremos el archivo [Cargo.toml](file:///home/august/code/bgustecosystem/bgustreadimg/Cargo.toml) usando **dependencias específicas por objetivo (target-specific dependencies)** y **características condicionales (Cargo features)**.

### Configuración Propuesta de `Cargo.toml`

```toml
[package]
name = "bgustreadimg"
version = "0.1.5"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[features]
default = ["nodejs"]
nodejs = ["dep:napi", "dep:napi-derive"]
python = ["dep:pyo3"]

# Dependencias comunes a todas las plataformas (incluido WASM)
[dependencies]
image = { version = "0.24.7", default-features = false, features = ["png", "jpeg"] }
ndarray = "0.15"

# Bindings opcionales
napi = { version = "2.12.0", default-features = false, features = ["async", "napi4"], optional = true }
napi-derive = { version = "2.12.0", optional = true }
pyo3 = { version = "0.20", features = ["extension-module"], optional = true }

# Dependencias exclusivas para plataformas nativas (Linux, macOS, Windows)
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
sysinfo = "0.29.11"
tokio = { version = "1.28.0", features = ["rt", "rt-multi-thread"] }
ort = "2.0.0-rc.9"

# Dependencias exclusivas para WebAssembly (Navegador)
[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
js-sys = "0.3"

[build-dependencies]
# napi-build solo se ejecuta si estamos compilando para Node.js
napi-build = { version = "2.0.1", optional = true }
```

---

## 🧠 2. Evaluación de Inferencia ONNX en el Frontend

Para ejecutar modelos como **Surya OCR** y **Table Transformer (Layout)** en el frontend, analizamos tres alternativas tecnológicas:

| Tecnología | Tipo | Aceleración GPU | Madurez de Modelos ONNX | Complejidad de Implementación | Recomendación |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **`onnxruntime-web` (JS/TS)** | Librería JS con backend WASM/WebGPU | **Alta** (WebGPU/WebGL) | **Máxima** (Soporte oficial de Microsoft) | **Baja** (Carga el `.onnx` directo) | **🟢 Altamente Recomendado (Híbrido)** |
| **`Candle` (Rust-WASM)** | Framework Rust ML | **Media** (wgpu experimental) | **Baja** (Requiere convertir a safetensors o implementar cargadores personalizados) | **Alta** (Manejo manual de tensores y operators) | **🟡 Alternativa Secundaria** |
| **`Burn` (Rust-WASM)** | Framework Rust ML | **Alta** (wgpu nativo) | **Media** (Tiene importador de ONNX, pero puede fallar con arquitecturas complejas) | **Muy Alta** (Traducción de pesos y código de inferencia en Rust) | **🔴 Desaconsejado por costo/riesgo** |

### Enfoque Híbrido Ganador (Rust-WASM + `onnxruntime-web`)
* **Rust-WASM (`wasm-bindgen`):** Se encarga de la decodificación de la imagen, el redimensionado Lanczos3 y la **Binarización de Sauvola en O(N)**. Devuelve un buffer optimizado (ej. escala de grises binarizada o floats normalizados).
* **`onnxruntime-web` (JS):** Recibe el buffer preprocesado por el WASM de Rust, lo convierte a un Tensor de JS y ejecuta la inferencia del OCR y Layout usando el motor WebGPU del navegador para máxima velocidad.

---

## 📂 3. Estructuración del Código Fuente

Organizaremos el código para que cada binding viva en su propio archivo/módulo:

```
src/
├── lib.rs              # Núcleo del algoritmo Sauvola y re-escalado (plataforma neutral)
├── layout.rs           # LayoutAnalyzer (ONNX - Excluido de WASM vía cfg)
├── ocr.rs              # OcrEngine (ONNX - Excluido de WASM vía cfg)
├── bindings_napi.rs    # Bindings para Node.js (#[cfg(feature = "nodejs")])
├── bindings_pyo3.rs    # Bindings para Python (#[cfg(feature = "python")])
└── bindings_wasm.rs    # Bindings para WASM (#[cfg(target_arch = "wasm32")])
```

---

## 🚀 4. Flujo de Trabajo para Publicación y Compilación

Para evitar conflictos al compilar y publicar, cada plataforma tiene su propio comando aislado:

### A. Rust Crate (Crates.io)
Para compilar y publicar la biblioteca nativa pura en Rust:
```bash
cargo publish --no-default-features
```

### B. Node.js Binding (NPM)
Para construir la versión nativa de Node.js:
```bash
npm run build # Ejecuta napi build --platform --release
```

### C. Python Binding (PyPI)
Maturin utiliza un archivo `pyproject.toml` en la raíz. Para compilar y empaquetar para Python:
```bash
maturin build --release --features python
```

### D. WebAssembly Binding (NPM/Frontend)
Para construir la librería JS/WASM para el navegador:
```bash
wasm-pack build --target web --out-dir pkg -- --no-default-features --features wasm
```

---

## 📋 5. Plan de Acción a Ejecutar

### Paso 1: Reestructuración de `Cargo.toml`
* Modificar el archivo `Cargo.toml` para incluir las dependencias condicionales (`target_arch = "wasm32"` y `not(wasm32)`).
* Declarar las features `nodejs` y `python` con sus respectivas dependencias opcionales.

### Paso 2: Separación de Bindings
* Mover los bloques de `napi` actuales de [src/lib.rs](file:///home/august/code/bgustecosystem/bgustreadimg/src/lib.rs) a `src/bindings_napi.rs` y habilitarlos condicionalmente con `#[cfg(feature = "nodejs")]`.
* Excluir `layout.rs` y `ocr.rs` usando `#[cfg(not(target_arch = "wasm32"))]` para que no rompan la compilación web.

### Paso 3: Configurar Maturin (Python)
* Crear `pyproject.toml` en la raíz del proyecto.
* Crear `src/bindings_pyo3.rs` y escribir la envoltura en Rust para la función de Sauvola.

### Paso 4: Configurar WebAssembly (Frontend)
* Crear `src/bindings_wasm.rs` y exponer la función `preprocess_image_wasm` usando `wasm-bindgen`.
* Asegurar que no use la lógica de hilos de `tokio` ni el chequeo de RAM de `sysinfo`.

### Paso 5: Pruebas de Compilación Cruzada
* Probar compilación nativa de Node.js: `npm run build`
* Probar compilación nativa de Python: `maturin build --features python`
* Probar compilación WebAssembly: `wasm-pack build --target web -- --no-default-features --features wasm`
