# widget-clock-rust-seqvspar
proyecto personal de una pequeña pantalla que muestra la hora con un fondo animado

## Descripción
El proyecto realizado consiste en un programa elaborado con RUST en donde se proyecta un reloj con un fondo mutable generado por convoluciones de ruido en el que hace uso de funciones tanto secuenciales como paralelas mostrando la diferencia entre ambos tipos de programación haciendo uso de la fluidez de imagen como herramienta gráfica para la visualización del efecto de estos sobre el sistema.

## Resultados
### Tiempos (segundos)
| Píxeles     | 100x100     | 360x360     | 360x720     | 1080x720    |
|-------------|-------------|-------------|-------------|-------------|
| Secuencial  | 0.00635188  | 0.04025     | 0.07259     | 0.2039556   |
| Paralelo    | 0.003503846 | 0.01594474  | 0.02926706  | 0.06621455  |
| SpeedUp     | 1.812831    | 2.524343    | 2.480263    | 3.080223    |
| Eficiencia  | 0.2266039   | 0.3155429   | 0.3100329   | 0.3850279   |
