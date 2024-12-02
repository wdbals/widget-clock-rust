# widget-clock-rust-seqvspar
proyecto personal de una pequeña pantalla que muestra la hora con un fondo animado

## Integrantes

- Ángel Alejandro Balderas Pech.
- Diego Alexander Rosado Valle.
- Kevin Leandro Camelo Suaste.

## Descripción

El proyecto realizado consiste en un programa elaborado con RUST en donde se proyecta un reloj con un fondo mutable generado por convoluciones de ruido en el que hace uso de funciones tanto secuenciales como paralelas mostrando la diferencia entre ambos tipos de programación haciendo uso de la fluidez de imagen como herramienta gráfica para la visualización del efecto de estos sobre el sistema.

**Sugerencia:** No tener ambos efectos visibles.

## Resultados

La aplicación fue probada con la versión compilada en debug (Sin optimizaciones).

### Tiempos (segundos)
| Píxeles     | 100x100     | 360x360     | 360x720     | 1080x720    |
|-------------|-------------|-------------|-------------|-------------|
| Secuencial  | 0.00635188  | 0.04025     | 0.07259     | 0.2039556   |
| Paralelo    | 0.003503846 | 0.01594474  | 0.02926706  | 0.06621455  |
| SpeedUp     | 1.812831    | 2.524343    | 2.480263    | 3.080223    |
| Eficiencia  | 0.2266039   | 0.3155429   | 0.3100329   | 0.3850279   |

## Aplicación del Paralelismo

Se aplico paralelismo en las lineas de código:
- [línea 228, 233 y 238](https://github.com/wdbals/widget-clock-rust/blob/main/src/convolutions/fire.rs#L223)

```rust
fn gen_palette() -> Palette {
        let mut palette = Palette::new();
        let colors = &mut palette;

        let red_colors: Vec<u32> = (1..=85u8) // Línea 228
            .into_par_iter()
            .map(|i| Color::rgb(i * 3, 0, 0)) // Obtener el valor RGB
            .collect();

        let green_colors: Vec<u32> = (1..=85u8) // Linea 233
            .into_par_iter()
            .map(|i| Color::rgb(255, i * 3, 0)) // Obtener el valor RGB
            .collect();

        let blue_colors: Vec<u32> = (1..=85u8) // Linea 238
            .into_par_iter()
            .map(|i| Color::rgb(255, 255, i * 3)) // Obtener el valor RGB
            .collect();

        colors.add_colors(red_colors);
        colors.add_colors(green_colors);
        colors.add_colors(blue_colors);

        palette
    }
```

- [línea 257](https://github.com/wdbals/widget-clock-rust/blob/main/src/convolutions/fire.rs#L257)

```rust
local_buffer.par_iter_mut().enumerate().for_each(|(i, pixel)| { // Linea 257
            let x = (i % width) as f32;
            let y = (i / width) as f32;

            // Parametrización de la onda
            let speed = 10f32; // Velocidad de la animación de la onda
            let amplitude = 170f32; // Amplitud máxima (0 a 254)
            let frequency_x = (5.0 * time).clamp(1f32, 4f32) * speed * std::f32::consts::PI / width as f32;  // Frecuencia en x
            let frequency_y = (5.0 * time).clamp(1f32, 4f32) * speed * std::f32::consts::PI / height as f32; // Frecuencia en y

            // Movimiento de la onda
            let wave_x = (x * frequency_x + time * speed).sin(); // Onda senoidal en la dirección x
            let wave_y = (y * frequency_y + time * speed).sin();

            // Combinación de las ondas en ambas direcciones (x, y)
            let wave = (wave_x * wave_y) * amplitude;

            // Margin
            let mx = width as f32 * 0.025;
            let my = height as f32 * 0.025;

            if x > mx && x < width as f32 - mx && y > my && y < height as f32 - my {
                let value = wave.clamp(10f32, 254f32) as u32;

                *pixel = value;
            }
        });
```

- [línea 297](https://github.com/wdbals/widget-clock-rust/blob/main/src/convolutions/fire.rs#L297)

```rust
pixels.par_iter_mut().enumerate().for_each(|(i, pixel)| { // Linea 297
            if *pixel != 0 {
                *pixel = *self.palette.get(self.local_buffer[i] as usize)
                    .expect("Color not found in pallete");
            }
        });
        println!("{:.4},", timer.elapsed().as_secs_f64());
    }
```

**Conclusión:** como podemos ver Rayon nos permite hacer paralelización de manera simple.

## ScreemShot
![image](https://github.com/user-attachments/assets/602bff1d-b1bf-408a-b8af-1b042f3958e0)

![image](https://github.com/user-attachments/assets/fc8ae57f-977f-4a81-95c0-49ea26c2f61f)

## Atajos:

| Tecla     | Función     |
|-------------|-------------|
| 1  | Alternar visibilidad de efecto (versión secuencial)  |
| 2    | Alternar visibilidad de efecto (versión paralela) |
| T     | Alternar visibilidad de la hora |
| ESC o Q     | Cierra el programa |


