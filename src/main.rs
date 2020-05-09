extern crate piston_window;
extern crate find_folder;

use piston_window::*;

mod game;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = 
        WindowSettings::new("Precision Sniper", [1024, 720])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    

    let mut game = game::Game { page: 1 };
    let background = game.get_background(&mut window);

    while let Some(event) = window.next() {
           
        if let Some(Button::Keyboard(key)) = event.press_args() {
            println!("{:?}", key);
            match key {
                Key::Return => {
                    if game.page == 1 {
                        game.set_page(2);
                    }
                },
                _ => {}
            }
        }
        game.render(&mut window, &background, event);
        
    }
}
