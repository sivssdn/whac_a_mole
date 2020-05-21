extern crate rand;

use piston_window::*;
use rand::Rng;
use std::path::PathBuf;

pub struct Game {
    pub page: u8,
    pub window: PistonWindow,
    pub assets: PathBuf,
    pub control_keys: [&'static str; 4],
    pub current_target_window: i32,
}

impl Game {
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
        let control_keys = self.control_keys;
        let [window_number, target_x, target_y] = self.get_target_location();
        self.set_current_target_window(window_number as i32);

        self.window.draw_2d(&event, |context, graphics, device| {
            clear([1.0; 4], graphics);
            rectangle(
                [0.0, 0.0, 0.0, 1.0], // page background
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
            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });
    }

    //checks if the user pressed the right window key for the target.
    fn is_right_target(&self, user_input: piston_window::Key) {
        let user_choice = [Key::Q, Key::R, Key::U, Key::P]
            .iter()
            .position(|&r| r == user_input);
        let mut user_choice_window: usize = 11;
        match user_choice {
            Some(index) => user_choice_window = index,
            None => {}
        };
        println!(
            "window {} -- user choice {}",
            self.current_target_window, user_choice_window
        );
    }

    /**
     * @returns Array f64 of size three.
     * 1. Random number between [0,3]. Window number where target will appear.
     * 2. X-axis generated randomly for window's position on x-axis.
     * 3. Y-axis fixed value.
     */
    fn get_target_location(&self) -> [f64; 3] {
        let choosen_coords: f64 = rand::thread_rng().gen_range(0.0, 4.0);
        [
            choosen_coords,
            230.0 + (choosen_coords.floor() * 810.0),
            500.0,
        ]
    }

    fn set_page(&mut self, page_number: u8) {
        self.page = page_number;
    }

    fn set_current_target_window(&mut self, window_number: i32) {
        self.current_target_window = window_number;
    }

    pub fn handle_usere_input(&mut self, key: piston_window::Key) {
        match key {
            Key::Return => {
                if self.page == 1 {
                    self.set_page(2);
                }
            }
            _ => {
                self.is_right_target(key);
            }
        }
    }
}
