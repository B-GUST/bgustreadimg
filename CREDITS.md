# Créditos y Atribuciones

## Algoritmo de Binarización Adaptativa

El núcleo de **bgustreadimg** implementa el **Algoritmo de Sauvola** para binarización adaptativa de imágenes, basado en el trabajo académico original:

- **J. Sauvola and M. Pietikäinen**, *"Adaptive document image binarization"*, Pattern Recognition, vol. 33, no. 2, pp. 225–236, 2000.

La implementación en Rust utiliza **Summed Area Tables (SAT)** para lograr una complejidad O(N) independientemente del tamaño de la ventana, optimizando el algoritmo original para procesamiento en tiempo real.

## Bindings Nativos

- **NAPI-RS** — Infraestructura de bindings nativos entre Rust y Node.js.

## Modelos de Inferencia ONNX

Los módulos opcionales de detección de layout y OCR utilizan modelos ONNX de código abierto:
- **Table Transformer** — Detección de regiones tabulares en documentos.
- **Surya OCR** — Reconocimiento de texto multilingüe.

## Ecosistema B-GUST

**bgustreadimg** es parte del ecosistema **B-GUST NLP**, diseñado para proporcionar una infraestructura completa de procesamiento documental, desde la limpieza de imágenes hasta la extracción semántica de texto.
