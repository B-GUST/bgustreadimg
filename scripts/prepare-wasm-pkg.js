const fs = require('fs');
const path = require('path');

const pkgWasmDir = path.join(__dirname, '..', 'pkg-wasm');
const pkgJsonPath = path.join(pkgWasmDir, 'package.json');
const readmeSrcPath = path.join(__dirname, '..', 'docs', 'README_WASM.md');
const readmeDstPath = path.join(pkgWasmDir, 'README.md');

function main() {
  console.log('--- Preparando paquete NPM para WebAssembly ---');

  if (!fs.existsSync(pkgJsonPath)) {
    console.error(`Error: No se encontró package.json en ${pkgWasmDir}. ¿Ejecutaste wasm-pack build primero?`);
    process.exit(1);
  }

  // 1. Modificar package.json
  const pkgData = JSON.parse(fs.readFileSync(pkgJsonPath, 'utf8'));
  
  pkgData.name = 'bgustreadimg-wasm';
  pkgData.description = 'WebAssembly build of bgustreadimg for high-performance frontend image preprocessing and Sauvola binarization in the browser.';
  
  // Agregar palabras clave específicas para la Web
  pkgData.keywords = ['ocr', 'image-processing', 'sauvola', 'binarization', 'preprocessing', 'wasm', 'webassembly', 'browser'];
  
  // Guardar cambios en package.json
  fs.writeFileSync(pkgJsonPath, JSON.stringify(pkgData, null, 2), 'utf8');
  console.log(`package.json actualizado con el nombre: ${pkgData.name}`);

  // 2. Copiar README específico para la Web
  if (fs.existsSync(readmeSrcPath)) {
    fs.copyFileSync(readmeSrcPath, readmeDstPath);
    console.log('README_WASM.md copiado a la carpeta de distribución.');
  } else {
    console.warn(`Advertencia: No se encontró el archivo origen en ${readmeSrcPath}`);
  }

  console.log('Preparación completada con éxito.');
}

main();
