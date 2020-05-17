extern crate piston_window;
extern crate find_folder;

use piston_window::*;

mod game;

fn main() {
    let opengl = OpenGL::V3_2;
    let window: PistonWindow = 
        WindowSettings::new("Precision Sniper", [1024, 720])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let assets = find_folder::Search::ParentsThenKids(3,3)
                .for_folder("assets").unwrap();
    let mut game = game::Game { page: 1, window: window, assets: assets, control_keys: ["Q", "W", "E", "R"] }; //move to constructor

    while let Some(event) = game.window.next() {
           
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

        if let Some(_args) = event.render_args() {
            game.render(event);
        }
        
    }
}
