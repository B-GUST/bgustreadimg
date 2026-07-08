# Créditos y Atribuciones

## Algoritmo de Binarización Adaptativa

El núcleo de **bgustreadimg** implementa el **Algoritmo de Sauvola** para binarización adaptativa de imágenes, basado en los trabajos académicos originales:

- **J. Sauvola, T. Seppänen, S. Haapakoski, and M. Pietikäinen**, *"Adaptive document binarization"*, in *Proceedings of the Fourth International Conference on Document Analysis and Recognition (ICDAR '97)*, vol. 1, pp. 147–152, 1997. DOI: [10.1109/ICDAR.1997.619831](https://doi.org/10.1109/ICDAR.1997.619831).
- **J. Sauvola and M. Pietikäinen**, *"Adaptive document image binarization"*, *Pattern Recognition*, vol. 33, no. 2, pp. 225–236, 2000. DOI: [10.1016/S0031-3203(99)00055-2](https://doi.org/10.1016/S0031-3203(99)00055-2).

La implementación en Rust utiliza **Summed Area Tables (SAT)** para lograr una complejidad O(N) independientemente del tamaño de la ventana, optimizando el algoritmo original para procesamiento en tiempo real.

## Bindings y Herramientas Multi-Distribución

- **NAPI-RS** — Infraestructura de bindings nativos entre Rust y Node.js.
- **PyO3 & Maturin** — Infraestructura de bindings para Python y empaquetador de módulos nativos (PyPI).
- **wasm-bindgen & wasm-pack** — Herramientas para la interacción y empaquetado de WebAssembly (WASM) en el frontend (NPM).

## Modelos e Inferencia ONNX

Los módulos opcionales de detección de layout y OCR utilizan modelos ONNX y motores de inferencia de código abierto:
- **Microsoft ONNX Runtime / ONNX Runtime Web** — Motor de inferencia de alto rendimiento desarrollado por Microsoft (bajo licencia MIT) para correr modelos ONNX tanto de forma nativa como en el navegador usando WebGPU/WebGL/WASM.
- **Table Transformer** — Detección de regiones tabulares en documentos.
- **Surya OCR** — Reconocimiento de texto multilingüe.

## Ecosistema B-GUST

**bgustreadimg** es parte del ecosistema **B-GUST NLP**, diseñado para proporcionar una infraestructura completa de procesamiento documental, desde la limpieza de imágenes hasta la extracción semántica de texto.
