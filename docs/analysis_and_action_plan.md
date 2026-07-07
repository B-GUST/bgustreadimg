# Análisis de Viabilidad y Plan de Acción: `bgustreadimg` (Python & Frontend/WASM)

Este documento detalla el análisis de viabilidad técnica para:
1. Empaquetar el proyecto bgustreadimg para Python utilizando **Maturin**.
2. Convertir el proyecto o adaptarlo para que funcione como una **librería de Frontend** (en el navegador) mediante **WebAssembly (WASM)**.

---

## 📊 1. Empaquetado con Maturin (Bindings para Python)

### ¿Qué es Maturin?
Maturin es la herramienta estándar y más recomendada en el ecosistema Rust para construir, empaquetar y publicar crates de Rust como módulos de Python (utilizando bindings de PyO3).

### Viabilidad: 🟢 Excelente (100% Viable)
El núcleo del procesamiento de imágenes (binarización adaptativa de Sauvola, re-escalado e inicialización de imágenes) está escrito en Rust puro dentro de `src/lib.rs`. Rust permite compilar el mismo código nativo para múltiples bindings (Node.js con `napi-rs` y Python con `pyo3` / `maturin`).

#### Requisitos para la implementación:
1. **Configuración del proyecto (`pyproject.toml`):**
   Se requiere crear un archivo `pyproject.toml` en la raíz del proyecto para indicarle a pip/maturin cómo compilar el proyecto como un paquete de Python.
2. **Dependencias en `Cargo.toml`:**
   Añadir `pyo3` con la feature `extension-module`.
3. **Módulo de bindings Python:**
   Crear un módulo exclusivo para Python (`py_bindings.rs` o dentro de `lib.rs` usando directivas de compilación condicional) que exponga las funciones mediante los macros de PyO3 (`#[pyfunction]` y `#[pymodule]`).
4. **Adaptación de tipos:**
   En lugar de recibir `Buffer` de Node.js, la función de Python recibirá un `&[u8]` (bytes) o un array de NumPy (utilizando el crate `numpy` de Rust) y devolverá `PyResult<Vec<u8>>` (o `bytes`).

---

## 🌐 2. Funcionamiento como Librería de Frontend (Navegador)

### Viabilidad en su estado actual: 🔴 Imposible
Actualmente, el proyecto **no puede ejecutarse en el frontend/navegador** por las siguientes razones:

1. **Destino de Compilación:** Compila a un binario nativo de Node.js (`.node`), que es una biblioteca compartida cargada dinámicamente (`cdylib`). Los navegadores no pueden cargar archivos `.node` nativos.
2. **Dependencia de `sysinfo`:** En `src/lib.rs`, el código intenta leer la memoria RAM disponible del sistema operativo. WebAssembly en el navegador se ejecuta en un sandbox sin acceso a APIs de bajo nivel del sistema operativo. Esta librería causaría un fallo de compilación o pánico en WASM.
3. **Dependencia de `tokio` (Multihilo nativo):** Se usa `tokio::task::spawn_blocking`. El runtime multihilo de Tokio requiere APIs de hilos del sistema operativo. WASM en el navegador es monohilo de forma predeterminada.
4. **Dependencia de `ort` (ONNX Runtime):** Los módulos `layout.rs` y `ocr.rs` usan el crate `ort`, el cual enlaza estática o dinámicamente con la librería nativa C++ de ONNX Runtime (`onnxruntime.dll`/`.so`). Esto no compila para la web de forma directa.

### Viabilidad con Adaptaciones: 🟡 Viable con Limitaciones
Es perfectamente viable crear una versión frontend (compilada a WebAssembly usando `wasm-bindgen` y `wasm-pack`), pero requiere **desacoplar** las dependencias nativas del sistema operativo.

#### Estrategia para Frontend (WASM):
* **Focalización en el Preprocesamiento:** El algoritmo core de binarización adaptativa Sauvola y el redimensionamiento con `image` (Lanczos3) son 100% compatibles con WebAssembly.
* **Exclusión de ONNX (OCR y Layout):** Para el navegador, lo ideal es compilar únicamente el módulo de preprocesamiento de imágenes en WASM. La inferencia ONNX (OCR y Layout) es sumamente pesada para correr en Rust-WASM dentro del navegador. Si es requerida en frontend, se debe realizar usando librerías de JS como `onnxruntime-web` o delegar el OCR/Layout a un backend.
* **Compilación condicional:** Usar características de Cargo (`features`) o banderas de plataforma (`#[cfg(target_arch = "wasm32")]`) para omitir `sysinfo`, `tokio` y `ort` al compilar para WebAssembly.

---

## 🗺️ Plan de Acción Recomendado

Proponemos estructurar la transición en dos fases principales, manteniendo la compatibilidad con la versión actual de Node.js.

### Fase 1: Modularización y Limpieza de Dependencias (Preparación)
Antes de añadir bindings para Python o WASM, reestructuraremos el código para separar la lógica de negocio pura de la lógica de plataforma.

1. **Configurar Características de Cargo (`features`):**
   Modificar `Cargo.toml` para definir features opcionales:
   * `nodejs` (incluye `napi` y `sysinfo`).
   * `onnx` (incluye `ort` y los módulos de OCR/Layout).
   * `python` (incluye `pyo3`).
   * `wasm` (incluye `wasm-bindgen`).
2. **Abstraer el Chequeo de RAM:**
   Hacer que la función de preprocesamiento acepte el `target_width` directamente. Si no se provee, usar un valor por defecto (ej. `1920`) en entornos WASM/Python, y solo consultar `sysinfo` si la feature `nodejs` o nativa está activa.
3. **Eliminar dependencias de Tokio de la lógica pura:**
   Hacer que el núcleo del procesamiento de imágenes sea síncrono. La asincronía la manejará la capa de binding correspondiente (como ya hace `napi` o como se maneja en JS/WASM con promesas).

### Fase 2: Implementación de Maturin (Python)
1. Instalar `maturin` en el entorno de desarrollo.
2. Crear el archivo `pyproject.toml` en la raíz.
3. Añadir la feature `python` en `Cargo.toml` que active `pyo3`.
4. Implementar los bindings de Python en `src/lib.rs` (o un módulo separado).
5. Probar la compilación y empaquetamiento usando `maturin develop`.

### Fase 3: Implementación de Frontend (WebAssembly)
1. Configurar la compilación de WASM usando `wasm-pack`.
2. Añadir la feature `wasm` que dependa de `wasm-bindgen`.
3. Crear el binding de WASM.
4. Compilar con `wasm-pack build --target web` para generar el paquete npm frontend utilizable en navegadores con frameworks como Vite o Webpack.
