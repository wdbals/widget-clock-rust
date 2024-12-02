use std::time::Instant;
use rayon::prelude::*;
use rand::Rng;
use crate::convolutions::{Color, Convolution, ConvolutionAdvanced, Palette};

pub struct FireConvolution {
    pub intensity: f32,  // Un parámetro que controla la "intensidad" del fuego.
}

impl Convolution for FireConvolution {
    fn name(&self) -> &str {
        "Fire"
    }

    fn transform(&mut self, pixels: &mut [u32], _width: usize, _height: usize) {
        // Usamos un generador de números aleatorios para crear variabilidad en el color del fuego
        let mut rng = rand::thread_rng();
        // Iterar sobre todos los píxeles y aplicar el efecto de fuego
        for pixel in pixels.iter_mut() {
            let flame_intensity: f32 = rng.gen_range(0.0..self.intensity); // Aleatorio en el rango [0.0, intensity)

            // Los colores cálidos para simular fuego (rojo, naranja, amarillo)
            let red = (flame_intensity * 255.0) as u8;
            let green = (flame_intensity * 180.0) as u8;
            let blue = (flame_intensity * 100.0) as u8;

            // Aplicar el color "fuego" al píxel (rojo, naranja, amarillo)
            *pixel = Color::rgb(red, green, blue);
        }
    }
}


pub struct HeatFireConvolution {
    base_intensity: f32,  // La intensidad base del fuego en la parte inferior
    falloff: f32,         // La "caída" o disminución de la intensidad a medida que subimos
    heat_buffer: Vec<f32>, // Buffer de calor para acumular la intensidad del fuego
}

impl HeatFireConvolution {
    pub fn new(base_intensity: f32, falloff: f32, width: usize, height: usize) -> Self {
        HeatFireConvolution {
            base_intensity,
            falloff,
            heat_buffer: vec![0.0; width * height], // Inicializa el buffer de calor
        }
    }

    // function to update the heat in the heat_buffer
    fn update_heat_buffer(&mut self, width: usize, height: usize) {
        let mut rng = rand::thread_rng(); // Generador de números aleatorios

        for y in 0..height {
            for x in 0..width {
                let index = y * width + x;

                // Genera un valor aleatorio de fuego en la base (color rojo), ajusta según la posición (altura)
                let intensity = self.base_intensity * (1.0 - self.falloff * (y as f32 / height as f32));
                let flame_intensity = rng.gen_range(0.0..intensity); // Aleatorio en el rango [0.0, intensity)

                // Acumulamos el calor en el heat_buffer
                self.heat_buffer[index] += flame_intensity; // Incrementa la intensidad de calor
            }
        }
    }

    // Function to mix heat with color
    fn apply_heat_to_pixels(&self, pixels: &mut [u32], width: usize, height: usize) {
        for y in 0..height {
            for x in 0..width {
                let index = y * width + x;
                let heat = self.heat_buffer[index];

                // Ajuste del color del fuego dependiendo de la intensidad de calor
                let red = (heat * 255.0) as u8;
                let green = (heat * 150.0) as u8;
                let blue = (heat * 50.0) as u8;

                // Aplicamos el color calculado a cada píxel
                pixels[index] = Color::rgb(red, green, blue);
            }
        }
    }
}

impl Convolution for HeatFireConvolution {
    fn name(&self) -> &str {
        "HeatFire"
    }

    fn transform(&mut self, pixels: &mut [u32], width: usize, height: usize) {
        // Primero actualizamos el buffer de calor
        self.update_heat_buffer(width, height);

        // Luego combinamos el calor con el buffer de píxeles
        self.apply_heat_to_pixels(pixels, width, height);
    }
}

impl ConvolutionAdvanced for HeatFireConvolution {
    fn reset(&mut self) {
        for pixel in self.heat_buffer.iter_mut() {
            *pixel = 0f32;
        }
    }
}


pub struct IdkConvolution {
    local_buffer: Vec<u32>,
    palette: Palette,
}

impl IdkConvolution {
    pub fn new(width: usize, height: usize) -> Self {
        IdkConvolution {
            local_buffer: vec![0; width*height],
            palette: Self::gen_palette(),
        }
    }

    fn gen_palette() -> Palette {
        let mut palette = Palette::new();
        let colors = &mut palette;

        for i in 1..=85u8 {
            colors.add_color(
                Color::rgb(i * 3,0,0)
            );
        }

        for i in 1..=85u8 {
            colors.add_color(
                Color::rgb(255,i * 3,0)
            );
        }

        for i in 1..=85u8 {
            colors.add_color(
                Color::rgb(255,255,i*3)
            );
        }

        palette
    }

    fn calculation(&mut self, width: usize, height: usize) {
        let local_buffer = &mut self.local_buffer;

        let time = chrono::Local::now();
        let time = time.timestamp_subsec_micros() as f32 / 1_000_000.0;

        // Paralelizamos el cálculo de cada píxel usando su índice lineal
        local_buffer.iter_mut().enumerate().for_each(|(i, pixel)| {
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
    }
}

impl Convolution for IdkConvolution {
    fn name(&self) -> &str {
        "RisingFire"
    }

    fn transform(&mut self, pixels: &mut [u32], width: usize, height: usize) {
        let timer = Instant::now();
        // self.gen_base(width, height);
        self.calculation(width, height);

        pixels.iter_mut().enumerate().for_each(|(i,pixel) | {
            if *pixel != 0 {
                *pixel = *self.palette.get(self.local_buffer[i] as usize)
                    .expect("Color not found in pallete");
            }
        });
        println!("{:.4},", timer.elapsed().as_secs_f64());
    }
}

impl ConvolutionAdvanced for IdkConvolution {
    fn reset(&mut self) {
        self.local_buffer = vec![0; self.local_buffer.len()];
    }
}

pub struct IdkParConvolution {
    local_buffer: Vec<u32>,
    palette: Palette,
}

impl IdkParConvolution {
    pub fn new(width: usize, height: usize) -> Self {
        IdkParConvolution {
            local_buffer: vec![0; width*height],
            palette: Self::gen_palette(),
        }
    }

    fn gen_palette() -> Palette {
        let mut palette = Palette::new();
        let colors = &mut palette;

        let red_colors: Vec<u32> = (1..=85u8)
            .into_par_iter()
            .map(|i| Color::rgb(i * 3, 0, 0)) // Obtener el valor RGB
            .collect();

        let green_colors: Vec<u32> = (1..=85u8)
            .into_par_iter()
            .map(|i| Color::rgb(255, i * 3, 0)) // Obtener el valor RGB
            .collect();

        let blue_colors: Vec<u32> = (1..=85u8)
            .into_par_iter()
            .map(|i| Color::rgb(255, 255, i * 3)) // Obtener el valor RGB
            .collect();

        colors.add_colors(red_colors);
        colors.add_colors(green_colors);
        colors.add_colors(blue_colors);

        palette
    }


    fn calculation(&mut self, width: usize, height: usize) {
        // let local_buffer = &mut self.local_buffer;
        //
        // // Paralelizamos el cálculo de cada píxel usando su índice lineal
        // local_buffer.par_iter_mut().enumerate().for_each(|(_, pixel)| {
        //     let mut rng = rand::thread_rng();
        //     *pixel = rng.gen_range(0..255);
        // });

        let local_buffer = &mut self.local_buffer;

        let time = chrono::Local::now();
        let time = time.timestamp_subsec_micros() as f32 / 1_000_000.0;

        // Paralelizamos el cálculo de cada píxel usando su índice lineal
        local_buffer.par_iter_mut().enumerate().for_each(|(i, pixel)| {
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
    }
}

impl Convolution for IdkParConvolution {
    fn name(&self) -> &str {
        "RisingFire"
    }

    fn transform(&mut self, pixels: &mut [u32], width: usize, height: usize) {
        let timer = Instant::now();
        // self.gen_base(width, height);
        self.calculation(width, height);

        pixels.par_iter_mut().enumerate().for_each(|(i, pixel)| {
            if *pixel != 0 {
                *pixel = *self.palette.get(self.local_buffer[i] as usize)
                    .expect("Color not found in pallete");
            }
        });
        println!("{:.4},", timer.elapsed().as_secs_f64());
    }
}

impl ConvolutionAdvanced for IdkParConvolution {
    fn reset(&mut self) {
        self.local_buffer = vec![0; self.local_buffer.len()];
    }
}