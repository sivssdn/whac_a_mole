extern crate find_folder;
extern crate piston_window;

use piston_window::*;

mod game;

fn main() {
    let opengl = OpenGL::V3_2;
    let window: PistonWindow = WindowSettings::new("Whac A Mole", [1024, 720])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let mut game = game::Game::new(window.max_fps(4), assets);

    while let Some(event) = game.window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.handle_usere_input(key);
        }

        if let Some(_args) = event.render_args() {
            game.render(event);
        }
    }
}
