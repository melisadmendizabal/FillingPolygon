# FillingPolygon

Este proyecto está escrito en **Rust** y tiene como objetivo principal **dibujar y rellenar polígonos** utilizando un framebuffer personalizado. Implementa algoritmos gráficos como el dibujo de líneas (Bresenham) y el llenado de polígonos mediante una solución creada por el estudiante. Mi enfoque se basa en recorrer los puntos del contorno, agruparlos por coordenada y, y rellenar horizontalmente entre pares de puntos alineados en esa fila. Luego ordena cada fila (grupo) por coordenada x, para identificar las "intersecciones" de esa fila con el borde del polígono. Recorre pares de puntos adyacentes y, si hay un espacio entre ellos (a.x + 1 < b.x), se considera un posible segmento de relleno. Se lleva un contador salto que actúa como una aproximación al algoritmo even-odd fill, alternando entre "pintar" o "no pintar" cuando se cruza un borde. Dado que el algoritmo no es estándar, se agregaron casos específicos para corregir comportamientos no deseados en vértices particulares de figuras complejas como estrellas o teteras.

---

## 📦 Requisitos
- Rust (`cargo`)
- Sistema Linux (Ubuntu recomendado)
- Dependencias listadas en `Cargo.toml`  `raylib`

---

## 🚀 Cómo ejecutar

```bash
cargo run
```

Esto compilará y ejecutará el proyecto. El resultado es una imagen out.png generada y guardada en el disco.

✍️ Autora
Melisa Mendizábal
