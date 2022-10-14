use std::{
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode};
use tank::{
    braille_canvas::BrailleCanvas, canvas::Canvas, drawable::Drawable, position::AsPoint, render,
    sprite::Sprite, tank::Tank, terminal, game_object::{self, GameObject}, obstacle::{Obstacles, Obstacle},
};

const CANVAS_MAX_X: usize = 120;
const CANVAS_MAX_Y: usize = 80;

fn main() {
    let mut stdout = terminal::init().unwrap();
    let mut instant = Instant::now();

    // render
    let (render_tx, render_rx) = mpsc::channel();
    let thread_handle = thread::spawn(move || {
        let mut last_canvas = BrailleCanvas::new(CANVAS_MAX_X, CANVAS_MAX_Y);
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_canvas, &last_canvas, true);
        loop {
            let cur_canvas = match render_rx.recv() {
                Ok(value) => value,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_canvas, &cur_canvas, false);
            last_canvas = cur_canvas;
        }
    });

    let mut tank = Tank::new(10, 10);
    let mut obstacles = Obstacles::new();
    obstacles.add_obstacle(Obstacle::new_circle(20, 20, 10));
    obstacles.add_obstacle(Obstacle::new_rect(40, 40, 60, 60));

    'mainloop: loop {
        let mut canvas = BrailleCanvas::new(CANVAS_MAX_X, CANVAS_MAX_Y);
        let delta = instant.elapsed();
        instant = Instant::now();

        while event::poll(Duration::default()).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Left => tank.go_left(),
                    KeyCode::Right => tank.go_right(),
                    KeyCode::Up => tank.go_up(),
                    KeyCode::Down => tank.go_down(),
                    KeyCode::Char('c') => tank.explode(),
                    KeyCode::Char(' ') => tank.shoot(),
                    KeyCode::Char('q') => {
                        break 'mainloop;
                    }
                    _ => {}
                }
            }
        }

        tank.update(delta);
        obstacles.update(delta);
        tank.check_obstacles(&mut obstacles);

        tank.draw(&mut canvas);
        obstacles.draw(&mut canvas);

        render_tx.send(canvas).unwrap();
        thread::sleep(Duration::from_millis(10));
    }

    drop(render_tx);
    terminal::deinit(&mut stdout).unwrap();
    thread_handle.join().unwrap();
}
