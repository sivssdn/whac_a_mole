extern crate rand;

use piston_window::*;
use rand::{thread_rng, Rng};
use std::{collections::HashMap, path::PathBuf};

pub struct Game {
    page: u8,
    pub window: PistonWindow,
    pub assets: PathBuf,
    control_keys: [&'static str; 4],
    current_target_window: usize,
    last_target_window: usize,
    total_score: i32,
    target_hit_status: i32,
}

impl Game {
    pub fn new(window: PistonWindow, assets: PathBuf) -> Game {
        Game {
            page: 1,
            window: window,
            assets: assets,
            control_keys: ["Q", "R", "U", "P"],
            current_target_window: 11,
            last_target_window: 11,
            total_score: 0,
            target_hit_status: 0,
        }
    }

    //calls page render based on page number background: &G2dTexture
    pub fn render(&mut self, event: piston_window::Event) {
        match self.page {
            1 => self.menu_page(event),
            2 => {
                self.game_page(event);
            }
            _ => {}
        }
    }

    //gets image from assets path
    fn get_image(&mut self, image_name: &str) -> G2dTexture {
        return Texture::from_path(
            &mut self.window.create_texture_context(),
            self.assets.join(image_name),
            Flip::None,
            &TextureSettings::new(),
        )
        .unwrap();
    }

    //renders menu page
    fn menu_page(&mut self, event: piston_window::Event) {
        let mut glyphs = self
            .window
            .load_font(self.assets.join("Amatic-Bold.ttf"))
            .unwrap();
        let mole_image = self.get_image("wild.png");
        self.window.draw_2d(&event, |context, graphics, device| {
            clear([1.0; 4], graphics);
            rectangle(
                [0.0, 0.0, 0.0, 1.0], // page background
                [0.0, 0.0, 1024.0, 720.0],
                context.transform,
                graphics,
            );
            image(
                &mole_image,
                context.transform.scale(0.4, 0.4).trans(900.0, 300.0),
                graphics,
            );

            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 52)
                .draw(
                    "Press 'enter' to start",
                    &mut glyphs,
                    &context.draw_state,
                    context.transform.trans(320.0, 460.0),
                    graphics,
                )
                .unwrap();
            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });
    }

    //renders game page
    fn game_page(&mut self, event: piston_window::Event) {
        let window_image = self.get_image("window.png");
        let target_image = self.get_image("wild.png");
        let mut glyphs = self
            .window
            .load_font(self.assets.join("Amatic-Bold.ttf"))
            .unwrap();
        let y_axis = 320.0;
        let mut x_axis = 120.0;
        let mut x_axis_text = 120.0;
        let score = self.total_score;
        let control_keys = self.control_keys;
        let [window_number, target_x, target_y] = self.get_target_location();
        self.last_target_window = self.current_target_window;
        self.set_current_target_window(window_number as usize);
        if self.last_target_window == 11 {
            self.last_target_window = self.current_target_window; //for the first iteration
        }
        let black = self.get_color(0);
        let target_hit_status_color = self.get_color(self.target_hit_status);

        self.window.draw_2d(&event, |context, graphics, device| {
            clear([1.0; 4], graphics);
            rectangle(
                black, // page background
                [0.0, 0.0, 1024.0, 720.0],
                context.transform,
                graphics,
            );
            for index in 0..4 {
                //window
                image(
                    &window_image,
                    context.transform.scale(0.4, 0.4).trans(x_axis, y_axis),
                    graphics,
                );
                //window key
                text::Text::new_color([1.0, 1.0, 1.0, 1.0], 52)
                    .draw(
                        &control_keys[index],
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(x_axis_text, y_axis + 140.0),
                        graphics,
                    )
                    .unwrap();
                x_axis += 600.0;
                x_axis_text += 254.0;
            }
            //target image
            image(
                &target_image,
                context.transform.scale(0.3, 0.3).trans(target_x, target_y),
                graphics,
            );
            //score
            text::Text::new_color([1.0, 1.0, 0.0, 1.0], 52)
                .draw(
                    &format!("Score : {}", score),
                    &mut glyphs,
                    &context.draw_state,
                    context.transform.trans(60.0, 700.0),
                    graphics,
                )
                .unwrap();
            //keypress circle
            Ellipse::new(target_hit_status_color).draw(
                ellipse::circle(980.0, 680.0, 20.0),
                &context.draw_state,
                context.transform,
                graphics,
            );
            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });
        self.target_hit_status = 0; //reset
    }

    //checks if the user pressed the right window key for the target. Increases the score for right target.
    //@returns true for every right target
    fn is_right_target(&mut self, user_input: piston_window::Key) -> bool {
        let user_choice = [Key::Q, Key::R, Key::U, Key::P]
            .iter()
            .position(|&r| r == user_input);
        match user_choice {
            Some(user_choice_window) => {
                if self.last_target_window == user_choice_window {
                    self.increment_score();
                    return true;
                }
                false
            }
            None => false,
        }
    }

    /**
     * @returns Array f64 of size three.
     * 1. Random number between [0,3]. Window number where target will appear.
     * 2. X-axis generated randomly for window's position on x-axis.
     * 3. Y-axis fixed value.
     */
    fn get_target_location(&self) -> [f64; 3] {
        let choosen_coords: f64 = thread_rng().gen_range(0.0, 4.0);
        [
            choosen_coords,
            230.0 + (choosen_coords.floor() * 810.0),
            500.0,
        ]
    }

    /**
     * @param int. 0 = black, 1 = red, 2 = green.
     * @return color codes array for piston window.
     */
    fn get_color(&self, code: i32) -> [f32; 4] {
        let mut colors = HashMap::new();
        colors.insert(0, [0.0, 0.0, 0.0, 1.0]); //black
        colors.insert(1, [1.0, 0.0, 0.0, 1.0]); //red
        colors.insert(2, [0.0, 1.0, 0.0, 0.7]); //green
        colors[&code]
    }

    fn set_page(&mut self, page_number: u8) {
        self.page = page_number;
    }

    fn set_current_target_window(&mut self, window_number: usize) {
        self.current_target_window = window_number;
    }

    fn increment_score(&mut self) {
        self.total_score += 10;
    }

    pub fn handle_usere_input(&mut self, key: piston_window::Key) {
        match key {
            Key::Return => {
                if self.page == 1 {
                    self.set_page(2);
                }
            }
            _ => {
                if self.is_right_target(key) {
                    self.target_hit_status = 2
                } else {
                    self.target_hit_status = 1
                };
            }
        }
    }
}
