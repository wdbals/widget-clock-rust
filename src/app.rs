use std::sync::{Arc, Mutex};
use tracing::{info, warn};
use crate::convolutions::*;
use crate::convolutions::base::SingleColorConvolution;
use crate::convolutions::fire::{IdkConvolution, IdkParConvolution};
use crate::convolutions::time::TimeConvolution;

pub fn run(width: usize, height: usize) {
    let mut window = Window::new(
        "Widget Hora: Fondo Procedural Example",
        width,
        height,
    );

    let convolutions = &mut window.convolutions;

    // Fondo Negro de la app
    convolutions.push((ConvolutionType::Simple(
        Box::new(SingleColorConvolution {
            red: 20,
            green: 20,
            blue: 20,
        })
    ), true));

    // Backgrounds
    convolutions.push((ConvolutionType::Advanced(
        Box::new(IdkConvolution::new(width, height))
    ), false));
    convolutions.push((ConvolutionType::Advanced(
        Box::new(IdkParConvolution::new(width, height))
    ), false));
    // End Backgrounds

    // Time's layer
    convolutions.push((ConvolutionType::Simple(
        Box::new(TimeConvolution {f24: true, color: Option::from(Color::rgb(50, 0, 125 ))})
        // Box::new(TimeConvolution {f24: false, color: None})
    ), true));

    window.run();
}

// #[derive(Debug)]
pub struct Window {
    buffer: Arc<Mutex<Vec<u32>>>,
    convolutions: Vec<(ConvolutionType, bool)>,
    width: usize,
    height: usize,
    window: minifb::Window,
}

impl Window {
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        let window = minifb::Window::new(
            title,
            width,
            height,
            minifb::WindowOptions {
                borderless: true,
                title: true,
                resize: true,
                scale: minifb::Scale::X2,
                scale_mode: minifb::ScaleMode::Stretch,
                topmost: true,
                transparency: false,
                none: false,
            },
        ).expect("The window can't be created");

        // window.set_target_fps(60);

        let buffer = vec![0u32; width * height];

        Window {
            window,
            buffer: Arc::new(Mutex::new(buffer)),
            convolutions: Vec::new(),
            width,
            height
        }
    }

    fn reset_convolutions(&mut self) {
        for (convolution, is_active) in self.convolutions.iter_mut() {
            if !*is_active {
                continue
            }

            match convolution {
                ConvolutionType::Simple(convolution) => {
                    let name = convolution.name();
                    warn!("{:?} hasn't reset", name);
                }
                ConvolutionType::Advanced(convolution) => {
                    let name = convolution.name();
                    warn!("{:?} has been reset", name);
                    convolution.reset();
                }
            }
        }
    }

    #[allow(unused)]
    fn transform_buffer(&mut self, convolution: &mut Box<dyn Convolution>) {
        let mut screen = self.buffer
            .lock()
            .expect("The mutex is poisoned");

        let mut new_buffer = screen.clone();
        convolution.transform(&mut new_buffer, self.width, self.height);

        screen.copy_from_slice(&*new_buffer);
    }

    fn apply_convolutions(&mut self) {
        for (convolution, is_active) in self.convolutions.iter_mut() {
            if !*is_active {
                continue
            }

            let name = match convolution {
                ConvolutionType::Simple(conv) => conv.name(),
                ConvolutionType::Advanced(conv) => conv.name(),
            };

            info!("Applying {:?} to buffer", name);

            let mut screen = self.buffer
                .lock()
                .expect("The mutex is poisoned");

            let mut new_buffer = screen.clone();
            match convolution {
                ConvolutionType::Simple(conv) => conv.transform(&mut new_buffer, self.width, self.height),
                ConvolutionType::Advanced(conv) => conv.transform(&mut new_buffer, self.width, self.height),
            }

            screen.copy_from_slice(&*new_buffer);
        }
    }

    fn render(&mut self) {
        let buffer = self.buffer.lock().expect("The mutex is poisoned");
        self.window.update_with_buffer(&buffer, self.width, self.height).expect("The window can't be updated");
    }

    pub fn run(&mut self) {
        while self.window.is_open()
            && !self.window.is_key_down(minifb::Key::Escape)
            && !self.window.is_key_down(minifb::Key::Q)
        {
            // Backgrounds visibility
            for (i, key) in [
                minifb::Key::Key1, minifb::Key::Key2
            ]
                .iter()
                .enumerate()
            {
                if self.window.is_key_pressed(*key, minifb::KeyRepeat::No) && i+1<3 {
                    if let Some((_, active)) = self.convolutions.get_mut(i+1) {
                        *active = !*active;
                    }
                }
            }

            if self.window.is_key_down(minifb::Key::R) { // reset convolution active
                self.reset_convolutions();
            }

            if self.window.is_key_released(minifb::Key::T) { // toggle clock
                let last = self.convolutions.len() - 1;
                if let Some((_, active)) = self.convolutions.get_mut(last) {
                    *active = !*active;
                }
            }

            self.apply_convolutions();

            self.render();
        }
    }
}