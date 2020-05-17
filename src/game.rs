use std::path::PathBuf;
use piston_window::*;

pub struct Game {
    pub page: u8,
    pub window: PistonWindow,
    pub assets: PathBuf
}

impl Game {
    pub fn get_background(&mut self) -> G2dTexture {
        return Texture::from_path(
            &mut self.window.create_texture_context(),
            self.assets.join("window.png"),
            Flip::None,
            &TextureSettings::new()
        ).unwrap();
    }

    //calls page render based on page number background: &G2dTexture
    pub fn render(&mut self, event: piston_window::Event) {
        match self.page {
            1 => self.menu_page(event),
            2 => {
                println!("PAGE 2 -----");
                self.menu_page(event);
            },
            _ => {}
        }
    }

    //renders menu page
    fn menu_page(&mut self, event: piston_window::Event) {
        let mut glyphs = self.window.load_font(self.assets.join("Amatic-Bold.ttf")).unwrap();
        let background = self.get_background();
        self.window.draw_2d(&event, |context, graphics, device| {
            clear([1.0; 4], graphics);
            rectangle([0.0, 0.0, 0.0, 1.0], // bgcolor
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

            image(&background,
            context.transform.scale(0.5, 0.5),
            graphics);
        });
    }

    pub fn set_page(&mut self, page_number: u8) {
        self.page = page_number;
    }
}