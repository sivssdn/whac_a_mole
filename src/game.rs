use std::path::PathBuf;
use piston_window::*;

pub struct Game {
    pub page: u8,
    pub window: PistonWindow,
    pub assets: PathBuf
}

impl Game {
    //calls page render based on page number background: &G2dTexture
    pub fn render(&mut self, event: piston_window::Event) {
        match self.page {
            1 => self.menu_page(event),
            2 => {
                println!("PAGE 2 -----");
                self.game_page(event);
            },
            _ => {}
        }
    }

    //gets image from assets path
    fn get_image(&mut self, image_name: &str) -> G2dTexture {
        return Texture::from_path(
            &mut self.window.create_texture_context(),
            self.assets.join(image_name),
            Flip::None,
            &TextureSettings::new()
        ).unwrap();
    }

    //renders menu page
    fn menu_page(&mut self, event: piston_window::Event) {
        let mut glyphs = self.window.load_font(self.assets.join("Amatic-Bold.ttf")).unwrap();  
        self.window.draw_2d(&event, |context, graphics, device| {
            clear([1.0; 4], graphics);
            rectangle([0.0, 0.0, 0.0, 1.0], // page background
                      [0.0, 0.0, 1024.0, 720.0],
                      context.transform,
                      graphics);

            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 52).draw(
                "Press 'enter' to start",
                &mut glyphs,
                &context.draw_state,
                context.transform.trans(320.0, 300.0),
                graphics
            ).unwrap();
            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });
    }

    //renders game page
    fn game_page(&mut self, event: piston_window::Event) {
        let window_image = self.get_image("window.png");
        let y_axis = 320.0;
        let mut x_axis = 120.0;
        self.window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle([0.0, 0.0, 0.0, 1.0], // page background
                      [0.0, 0.0, 1024.0, 720.0],
                      context.transform,
                      graphics);
            //windows
            image(
                &window_image,
                context.transform.scale(0.4, 0.4).trans(x_axis, y_axis),
                graphics);
            x_axis += 600.0;
            image(
                &window_image,
                context.transform.scale(0.4, 0.4).trans(x_axis, y_axis),
                graphics);
            x_axis += 600.0;
            image(
                &window_image,
                context.transform.scale(0.4, 0.4).trans(x_axis, y_axis),
                graphics);
            x_axis += 600.0;
            image(
                &window_image,
                context.transform.scale(0.4, 0.4).trans(x_axis, y_axis),
                graphics);

        });
    }

    pub fn set_page(&mut self, page_number: u8) {
        self.page = page_number;
    }
}