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
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            // rectangle([1.0, 0.0, 0.0, 1.0], // red
            //           [0.0, 0.0, 100.0, 100.0],
            //           context.transform,
            //           graphics);
            image(background, context.transform, graphics)
        });
    }

    pub fn setPage(&mut self, page_number: u8) {
        &self.page = page_number;
    }
}