use std::{thread, time::Duration};

use tank::{terminal, render, braille_canvas::BrailleCanvas, canvas::Canvas, sprite::Sprite, drawable::Drawable, position::AsPoint};

fn main() {
    let mut stdout = terminal::init().unwrap();

    let mut canvas = BrailleCanvas::new(50, 50);
    let mut sprite = Sprite::new();
    sprite.draw_circle((0i16, 0i16).as_point(), 5);

    
    sprite.draw(&mut canvas);
    render::render(&mut stdout, &canvas, &canvas, true);
    
    thread::sleep(Duration::from_secs(10));
    terminal::deinit(&mut stdout).unwrap();
    println!("Ok");
}
