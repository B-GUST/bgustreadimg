# Plan de Empaquetado y Documentación para NPM (WASM) y Pip (Python)

Este documento detalla el plan de acción para configurar las publicaciones de:
1.  **NPM Frontend (WebAssembly):** Empaquetado con el nombre `bgustreadimg-wasm` (o similar), incluyendo su propio README optimizado para desarrollo frontend.
2.  **Python (Pip):** Documentación sobre la instalación y el uso de los nuevos bindings.
3.  **Actualización del README Principal:** Consolidar todas las distribuciones en el documento raíz.

---

## 📦 1. Estrategia de Empaquetado para NPM (WASM)

Dado que `bgustreadimg` ya existe en npm como la versión backend para Node.js (con binarios nativos `.node`), la versión para el navegador (frontend) debe publicarse como un paquete diferente.

**Nombre Recomendado:** `bgustreadimg-wasm` (o `bgustreadimg-web`).

### Automatización del Build y Metadatos de la Versión Web
Para generar y empaquetar el código WebAssembly sin modificar manualmente los archivos autogenerados por `wasm-pack`, implementaremos el siguiente flujo automatizado:

1.  **Script de Preparación (`scripts/prepare-wasm-pkg.js`):**
    Este script de Node.js se ejecutará automáticamente después de compilar con `wasm-pack`. Realizará lo siguiente:
    *   Leer el archivo `pkg-wasm/package.json` autogenerado.
    *   Cambiar el nombre del paquete a `bgustreadimg-wasm`.
    *   Ajustar la descripción para indicar que es la versión de frontend/navegador.
    *   Copiar un archivo `docs/README_WASM.md` (específico para frontend) a `pkg-wasm/README.md`.
2.  **Actualización de `package.json`:**
    Añadir el script `"build:wasm"` para simplificar la compilación:
    ```json
    "build:wasm": "wasm-pack build --target web --out-dir pkg-wasm && node scripts/prepare-wasm-pkg.js"
    ```

---

## 📝 2. Actualización de Documentación

### A. Nuevo README para WebAssembly (`docs/README_WASM.md`)
Este archivo acompañará al paquete `bgustreadimg-wasm` en npm y explicará:
*   Que es la versión compilada a WebAssembly de `bgustreadimg` optimizada para el navegador.
*   Cómo importarlo usando herramientas modernas (Vite, Webpack, etc.).
*   Ejemplos prácticos de uso con `Uint8Array` e integración con Canvas.
*   Cómo combinarlo con `onnxruntime-web` para realizar inferencia de modelos ONNX (Layout/OCR) en el cliente.

### B. Actualización del README Principal (`README.md`)
Añadir secciones claras para el uso de Python/pip y el nuevo paquete WASM de Frontend en el archivo raíz.

---

## 🗺️ Plan de Acción a Ejecutar

### Paso 1: Crear el Script de Automatización
Crear el archivo `scripts/prepare-wasm-pkg.js` en el proyecto para que modifique los metadatos del paquete autogenerado por `wasm-pack`.

### Paso 2: Crear la Documentación Específica para WASM
Crear el archivo `docs/README_WASM.md` con las instrucciones de uso para el navegador.

### Paso 3: Modificar `package.json`
Añadir el script `"build:wasm"` a los scripts de npm para automatizar todo el proceso.

### Paso 4: Actualizar `README.md` Principal
Editar el archivo `README.md` raíz de la librería para documentar todas las formas de uso actuales (Rust, Node.js, Python, y WebAssembly).
