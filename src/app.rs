use std::sync::{Arc, Mutex};
use tracing::{info, warn};
use crate::convolution::*;

pub fn run(width: usize, height: usize) {
    let mut window = Window::new(
        "Widget Hora: Fondo Procedural Example",
        width,
        height,
    );

    let convolutions = &mut window.convolutions;
    // convolutions.push(ConvolutionType::Normal(
    //     Box::new(ToGreenConvolution)
    // ));
    // convolutions.push(ConvolutionType::Normal(
    //     Box::new(FireConvolution { intensity: 0.85f32 })
    // ));
    // convolutions.push(ConvolutionType::Normal(
    //     Box::new(SingleColorConvolution {
    //         red: 0,
    //         green: 0,
    //         blue: 0,
    //     })
    // ));
    convolutions.push(ConvolutionType::Advanced(
        Box::new(HeatFireConvolution::new(0.005f32, 0.5f32, width, height))
    ));
    convolutions.push(ConvolutionType::Normal(
        Box::new(TimeConvolution {f24: true, color: Option::from(Color::rgb(50, 0, 125 ))})
        // Box::new(TimeConvolution {f24: true, color: None})
    ));

    window.run();
}

// #[derive(Debug)]
pub struct Window {
    buffer: Arc<Mutex<Vec<u32>>>,
    convolutions: Vec<ConvolutionType>,
    width: usize,
    height: usize,
    window: minifb::Window,
}

impl Window {
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        let mut window = minifb::Window::new(
            title,
            width,
            height,
            minifb::WindowOptions {
                borderless: false,
                title: true,
                resize: true,
                scale: minifb::Scale::FitScreen,
                scale_mode: minifb::ScaleMode::Stretch,
                topmost: true,
                transparency: false,
                none: false,
            },
        ).expect("The window can't be created");

        // window.set_target_fps(144);

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
        for convolution in self.convolutions.iter_mut() {
            match convolution {
                ConvolutionType::Normal(convolution) => {
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
        for convolution in self.convolutions.iter_mut() {
            let name = match convolution {
                ConvolutionType::Normal(conv) => conv.name(),
                ConvolutionType::Advanced(conv) => conv.name(),
            };

            info!("Applying {:?} to buffer", name);


            let mut screen = self.buffer
                .lock()
                .expect("The mutex is poisoned");

            let mut new_buffer = screen.clone();
            match convolution {
                ConvolutionType::Normal(conv) => conv.transform(&mut new_buffer, self.width, self.height),
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
            if self.window.is_key_down(minifb::Key::R) {
                self.reset_convolutions();
            }

            self.apply_convolutions();

            self.render();
        }
    }
}