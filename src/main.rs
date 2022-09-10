#![warn(clippy::pedantic)]

mod tuple;

use std::time::{Duration, SystemTime};

use pixels::{Error, Pixels, SurfaceTexture};
use tuple::{Point, Scalar, Vector};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const BOX_SIZE: i16 = 8;
const SCALE: f64 = 4.;

struct World {
    proj: Projectile,
    env: Environment,
}

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = {
        let size: LogicalSize<f64> =
            LogicalSize::new(f64::from(WIDTH) * SCALE, f64::from(HEIGHT) * SCALE);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut world = World::new();
    let mut input = WinitInputHelper::new();

    let mut prev_time = SystemTime::now();
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| eprintln!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            let time = SystemTime::now();
            let delta_t = time.duration_since(prev_time).unwrap();
            prev_time = time;
            world.tick(delta_t);
            window.request_redraw();
        }
    });
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            proj: Projectile {
                position: Point {
                    x: 10.,
                    y: 100.,
                    z: 0.,
                },
                velocity: Vector {
                    x: 150.,
                    y: -90.,
                    z: 0.,
                },
            },
            env: Environment {
                gravity: Vector {
                    x: 0.,
                    y: 100.,
                    z: 0.,
                },
                wind: Vector {
                    x: -1.,
                    y: 0.,
                    z: 0.,
                },
            },
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn tick(&mut self, delta_t: Duration) {
        let delta_t = Scalar::new(delta_t.as_secs_f64());
        if self.proj.position.x <= 0. || self.proj.position.x >= f64::from(WIDTH - BOX_SIZE as u32)
        {
            self.proj.velocity.x *= -0.8;
            self.proj.velocity.y *= 0.8;
        }
        if self.proj.position.y <= 0. || self.proj.position.y >= f64::from(HEIGHT - BOX_SIZE as u32)
        {
            self.proj.velocity.x *= 0.8;
            self.proj.velocity.y *= -0.8;
        }

        if self.proj.velocity.magnitude() < Scalar::new(1.) {
            self.proj.velocity = Vector {
                x: 0.,
                y: 0.,
                z: 0.,
            }
        } else {
            self.proj.velocity += (&self.env.gravity + &self.env.wind) * delta_t;
        }
        self.proj.position += &self.proj.velocity * delta_t;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            #[allow(clippy::cast_precision_loss)]
            let (x, y) = ((i % WIDTH as usize) as f64, (i / WIDTH as usize) as f64);

            let inside_the_box = x >= self.proj.position.x
                && x < self.proj.position.x + f64::from(BOX_SIZE)
                && y >= self.proj.position.y
                && y < self.proj.position.y + f64::from(BOX_SIZE);

            let rgba = if inside_the_box {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
