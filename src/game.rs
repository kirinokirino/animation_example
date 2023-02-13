use speedy2d::dimen::UVec2;
use speedy2d::shape::Rectangle;
use speedy2d::window::VirtualKeyCode;
use speedy2d::Graphics2D;
use speedy2d::{
    dimen::Vec2,
    image::{ImageFileFormat, ImageHandle, ImageSmoothingMode},
};

use crate::app::{Keyboard, Mouse};
use crate::config::Config;
use crate::robot::Robot;
use crate::spritesheet::Spritesheet;

pub struct Game {
    config: Config,
    images: Vec<ImageHandle>,
    spritesheets: Vec<Spritesheet>,

    robot: Option<Robot>,

    counter: usize,

    viewport_size: UVec2,
}

impl Game {
    pub const fn new(config: Config) -> Self {
        let viewport_size = UVec2::new(config.window_width, config.window_height);
        Self {
            config,
            images: Vec::new(),
            spritesheets: Vec::new(),

            robot: None,

            counter: 0,
            viewport_size,
        }
    }

    pub fn setup(&mut self, graphics: &mut Graphics2D) {
        let image_handle = graphics
            .create_image_from_file_path(
                Some(ImageFileFormat::PNG),
                ImageSmoothingMode::Linear,
                "assets/robot.png",
            )
            .unwrap();
        //self.images.push(spritesheet);
        self.robot = Some(Robot::new(Spritesheet::new(image_handle, 9, 5)));
    }

    pub fn input(&mut self, viewport_size: UVec2, _mouse: &Mouse, keyboard: &Keyboard) {
        self.viewport_size = viewport_size;
        if let Some(robot) = &mut self.robot {
            if keyboard.pressed.contains(&VirtualKeyCode::Space) {
                robot.dbg_next_state(self.counter);
                self.counter += 1;
            }
        }
    }

    pub fn update(&mut self, current_frame: u64) {
        if let Some(robot) = &mut self.robot {
            robot.update(current_frame);
        }
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {
        if let Some(robot) = &self.robot {
            robot.draw(
                &Rectangle::new(Vec2::new(0.0, 0.0), Vec2::new(150.0, 200.0)),
                graphics,
            );
        }
    }
}
