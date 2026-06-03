# bgustreadimg (bgustdown-img) 🖼️

**Preprocesador de Imágenes de Alta Fidelidad y Limpieza Adaptativa para Motores de OCR.**

`bgustreadimg` es un microservicio y biblioteca en Rust (con bindings nativos de Node.js mediante NAPI-RS) diseñado específicamente para optimizar imágenes y fotografías de documentos (como facturas físicas, contratos o capturas de cámara) antes de ser enviadas a motores de reconocimiento de texto (OCR).

A diferencia de los convertidores de formato convencionales, su función principal es preparar la imagen eliminando sombras, arrugas y variaciones de luz no uniformes, garantizando que el texto final sea extremadamente nítido y legible.

---

## 🌟 Diferencias Clave con `bgustdown`

| Módulo | `bgustdown` (Core) | `bgustreadimg` |
| :--- | :--- | :--- |
| **Rol** | Motor de Ingesta y ETL de Documentos. | Preprocesador Físico de Imágenes para OCR. |
| **Formatos** | PDF, DOCX, XLSX, ODT, CSV. | PNG, JPEG, JPG, WEBP. |
| **Salida** | Markdown Estructurado y Tablas Apache Arrow. | Búfer de Imagen (`Luma8` binarizado en PNG sin pérdidas). |
| **Enfoque** | Extracción semántica y optimización de tokens. | Calidad de imagen y corrección de contraste adaptativo. |

---

## 🚀 Algoritmo de Limpieza Adaptativa (Sauvola)

Anteriormente, la biblioteca utilizaba umbrales de binarización estáticos y rígidos (`min 110`, `max 165`), lo cual causaba la pérdida de texto en imágenes oscuras o con sombras marcadas.

En la versión **`0.1.4`**, hemos implementado el **Algoritmo de Binarización Adaptativa de Sauvola** en Rust, optimizado mediante **Imágenes Integrales (Summed Area Tables - SAT)** para ejecutarse en tiempo lineal $O(N)$ independientemente del tamaño de la ventana.

El algoritmo calcula dinámicamente un umbral de contraste local $T(x,y)$ para cada píxel:
$$T(x,y) = m(x,y) \cdot \left( 1 + k \cdot \left( \frac{s(x,y)}{R} - 1 \right) \right)$$
Donde $m$ es la media local, $s$ es la desviación estándar local, $R = 128$ y $k = 0.2$ es el factor de sensibilidad. Esto elimina arrugas y sombras de fondos sin distorsionar los caracteres de texto.

---

## 🛠️ Uso y API desde Node.js

### Instalación
```bash
npm install bgustdown-img
```

### Código de Ejemplo
```javascript
const { preprocessImage } = require('bgustdown-img');
const fs = require('fs');

async function cleanInvoice() {
  const inputBuffer = fs.readFileSync('./factura_arrugada.jpg');
  
  // Procesar imagen con configuración personalizada de Sauvola
  const cleanBuffer = await preprocessImage(inputBuffer, {
    windowSize: 25,     // Tamaño de la ventana local de análisis
    k: 0.2,             // Sensibilidad al contraste (menor = más agresivo con las sombras)
    targetWidth: 1920   // Ancho máximo (redimensionamiento con filtro Lanczos3)
  });

  fs.writeFileSync('./factura_lista_para_ocr.png', cleanBuffer);
  console.log('Imagen preprocesada con éxito.');
}

cleanInvoice();
```
