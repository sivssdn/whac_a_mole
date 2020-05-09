use piston_window::*;

pub struct Game {
    pub page: u8
}

impl Game {
    pub fn get_background(&mut self, window: &mut PistonWindow) -> G2dTexture {
        let assets = find_folder::Search::ParentsThenKids(3,3)
                .for_folder("assets").unwrap();
        return Texture::from_path(
            &mut window.create_texture_context(),
            assets.join("wallpaper.jpeg"),
            Flip::None,
            &TextureSettings::new()
        ).unwrap();
    }

    pub fn render(&mut self, window: &mut PistonWindow, background: &G2dTexture, event: piston_window::Event) {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
        let mut glyphs = window.load_font(assets.join("Amatic-Bold.ttf")).unwrap();
    //=====----for fonts    

        window.draw_2d(&event, |context, graphics, device| {
            clear([1.0; 4], graphics);
            rectangle([0.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 1024.0, 720.0],
                      context.transform,
                      graphics);
            // image(background, context.transform, graphics);

            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 52).draw(
                "Press 'enter' to start",
                &mut glyphs,
                &context.draw_state,
                context.transform.trans(120.0, 200.0),
                graphics
            ).unwrap();
            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });
    }

    pub fn set_page(&mut self, page_number: u8) {
        self.page = page_number;
    }
}