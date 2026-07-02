# Guía de Contribución

¡Gracias por tu interés en contribuir a **bgustreadimg**! Este es un proyecto de código abierto industrial y damos la bienvenida a desarrolladores, ingenieros de visión computacional e investigadores de OCR para mejorar el motor.

Para asegurar un desarrollo coordinado, estable y limpio, solicitamos a todos los colaboradores seguir las siguientes pautas al proponer cambios a través de **Pull Requests (PR)** en GitHub.

---

## 🗺️ 1. Flujo de Trabajo en Git (Git Flow)

El proyecto utiliza un sistema estructurado de ramas basado en ramas de características (`feature branches`).

### Paso 1: Crear una rama de desarrollo
Crea una rama descriptiva desde `main` utilizando nombres en inglés que expliquen el cambio:
```bash
git checkout -b feature/your-awesome-feature
git checkout -b fix/resolve-some-bug
```

### Paso 2: Desarrollar e integrar pruebas locales
Asegúrate de que tus modificaciones compilen y pasen la suite de validación:
```bash
cargo fmt --all
cargo clippy
cargo test
```

### Paso 3: Mantener la rama sincronizada
Antes de enviar tu propuesta, haz un merge de los últimos cambios de `main` para evitar conflictos:
```bash
git fetch origin
git merge origin/main
```

---

## 📝 2. Convención de Commits (Conventional Commits)

Seguimos el estándar de **Conventional Commits** para mantener un historial limpio y estructurado.

Cada mensaje de commit debe tener el siguiente formato:
```
<tipo>(<ámbito opcional>): <descripción breve en imperativo>

[cuerpo opcional detallando el motivo y contexto]

[pie opcional con referencias a issues, ej: Closes #123]
```

### Tipos Permitidos:
* **`feat`**: Nueva funcionalidad (ej: `feat(binarization): add Niblack thresholding`).
* **`fix`**: Corrección de un error (ej: `fix(sauvola): handle edge case with uniform images`).
* **`docs`**: Cambios en la documentación.
* **`style`**: Cambios cosméticos que no afectan la lógica.
* **`refactor`**: Reestructuración de código sin cambios funcionales.
* **`test`**: Añadir o corregir pruebas existentes.
* **`chore`**: Tareas de mantenimiento o configuración del build system.

---

## 🚀 3. Guía para Crear Pull Requests (PR)

1. **Abre un Pull Request** apuntando a la rama `main` del repositorio oficial.
2. **Describe detalladamente el cambio:**
   * **Descripción:** ¿Qué problema resuelve y cómo?
   * **Tipo de Cambio:** `feat`, `fix`, `docs`, `refactor`, etc.
   * **Validación Realizada:** Especifica qué pruebas ejecutaste (`cargo test`, benchmarks, etc.).
   * **Issues Relacionados:** Enlaza los tickets resueltos con `Closes #numero`.

### Requisitos Técnicos Obligatorios:
* **Compilación Perfecta:** El código debe compilar sin advertencias de Clippy.
* **Sin Regresiones:** El 100% de las pruebas preexistentes deben seguir pasando.
* **Nuevas Pruebas:** Si añades una nueva característica, incluye su prueba unitaria o de integración.
* **Documentación Actualizada:** Si modificas la API pública, actualiza la documentación en `docs/` y el `README.md`.

---

## 🤝 4. Código de Conducta

Mantén una comunicación profesional, respetuosa y constructiva. La revisión por pares es un proceso de aprendizaje mutuo; agradecemos el feedback enfocado en la calidad, mantenibilidad y rendimiento del software.

¡Gracias por ayudarnos a construir el mejor motor de preprocesamiento de imágenes para OCR en Rust!
