extern crate piston_window;
extern crate find_folder;

use piston_window::*;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = 
        WindowSettings::new("Hello World!", [1024, 720])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let assets = find_folder::Search::ParentsThenKids(3,3)
                .for_folder("assets").unwrap();
    let background = Texture::from_path(
            &mut window.create_texture_context(),
            assets.join("wallpaper.jpeg"),
            Flip::None,
            &TextureSettings::new()
        ).unwrap();
    window.set_lazy(true);

    while let Some(event) = window.next() {
           
           if let Some(Button::Keyboard(key)) = event.press_args() {
               println!("{:?}", key);
           }

           window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      context.transform,
                      graphics);
            image(&background, context.transform, graphics)
            });
            
    }
}
