# Propuesta de Aprendizaje (`/learn`) - Regla de Directorio de Reportes

El usuario ha solicitado que cada vez que se le presente un reporte o un plan en formato `.md`, este se guarde en la carpeta `docs` dentro del directorio actual del proyecto (creando la carpeta si no existe).

A continuación se detalla la clasificación, justificación y los cambios propuestos para registrar este comportamiento de manera persistente.

---

## 📋 Clasificación y Justificación

*   **Clasificación:** Regla de Espacio de Trabajo (Workspace Rule).
*   **Justificación:** Es una restricción estricta de salida y gestión de archivos para mantener organizados los documentos de planificación/análisis dentro de la estructura física del proyecto del usuario.
*   **Ubicación del Archivo de Regla:** Crearemos el archivo de reglas en `.agents/rules/docs_reports.md` dentro de la raíz del proyecto actual (`/home/august/code/bgustecosystem/bgustreadimg`).

---

## 🛠️ Modificaciones Propuestas

### Nuevo archivo: `.agents/rules/docs_reports.md`

```markdown
# Regla: Guardado de Reportes y Planes en Carpeta `docs`

## Contexto
El usuario requiere tener acceso a todos los reportes, análisis y planes en formato Markdown (`.md`) directamente en la estructura de su espacio de trabajo.

## Regla
* Cada vez que generes o edites un reporte, plan de acción, análisis o propuesta de aprendizaje en formato `.md` destinado a ser leído por el usuario, debes guardarlo en el directorio `docs/` en la raíz del espacio de trabajo actual.
* Si el directorio `docs/` no existe en la raíz del espacio de trabajo, debes crearlo primero antes de escribir el archivo.
* Esto aplica de manera adicional a los archivos guardados en el directorio de artefactos del sistema.
```
