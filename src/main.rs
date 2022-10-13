use tank::{terminal, render, braille_canvas::BrailleCanvas, canvas::Canvas};

fn main() {
    let mut stdout = terminal::init().unwrap();

    let mut canvas = BrailleCanvas::new(50, 50);
    canvas.draw_dot(10, 10);
    render::render(&mut stdout, &canvas, &canvas, true);
    
    terminal::deinit(&mut stdout).unwrap();
    println!("Ok");
}
