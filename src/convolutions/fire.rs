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


pub struct RisingFireConvolution {
    fire_buffer: Vec<u32>,
    palette: Palette,
}

impl RisingFireConvolution {
    pub fn new(width: usize, height: usize) -> Self {
        RisingFireConvolution {
            fire_buffer: vec![0; width*height],
            palette: Self::gen_palette(),
        }
    }

    pub fn gen_palette() -> Palette {
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

    fn gen_base(&mut self, width: usize, height: usize) {
        let mut rng = rand::thread_rng();
        let i = width * (height - 1);

        for x in 0..width {
            self.fire_buffer[i + x] = rng.gen_range(0..255);
        }
    }

    fn calculation(&mut self, width: usize, height: usize) {
        let fire_buffer = &mut self.fire_buffer;

        for y in 0..height-1 {
            for x in 1..width-1 {
                let i = y * width + x;

                fire_buffer[i] = (
                    10 * fire_buffer[i - 1]
                        + 20 * fire_buffer[i]
                        + 10 * fire_buffer[i + 1]
                        + 160 * fire_buffer[i - 1 + width]
                        + 320 * fire_buffer[i + width]
                        + 160 * fire_buffer[i + 1 + width]
                ) / (680f32 * 1.01) as u32;
            }
        }
    }
}

impl Convolution for RisingFireConvolution {
    fn name(&self) -> &str {
        "RisingFire"
    }

    fn transform(&mut self, pixels: &mut [u32], width: usize, height: usize) {
        self.gen_base(width, height);
        self.calculation(width, height);

        pixels.par_iter_mut().enumerate().for_each(|(i, pixel)| {
            if *pixel != 0 {
                *pixel = *self.palette.get(self.fire_buffer[i] as usize)
                    .expect("Color not found in pallete");
            }
        });
    }
}

impl ConvolutionAdvanced for RisingFireConvolution {
    fn reset(&mut self) {
        self.fire_buffer = vec![0; self.fire_buffer.len()];
    }
}