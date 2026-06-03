const { preprocessImage } = require('./index.js');

async function test() {
  console.log('--- Probando bgustdown-img ---');
  try {
    // Intentamos procesar un buffer vacío o uno inválido para ver la respuesta del manejador de errores de Rust
    console.log('Probando preprocessImage con buffer inválido...');
    const buffer = Buffer.from([]);
    await preprocessImage(buffer);
    console.log('Éxito (no esperado para buffer vacío)');
  } catch (err) {
    console.log('Error capturado con éxito de Rust (comportamiento esperado):', err.message);
  }
}

test();
