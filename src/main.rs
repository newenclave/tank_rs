use std::{
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode};
use tank::{
    braille_canvas::BrailleCanvas,
    drawable::Drawable,
    obstacle::{Obstacle, Obstacles},
    position::IndexType,
    render,
    tank::Tank,
    terminal,
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

    let mut tank = Tank::new(10, (CANVAS_MAX_Y - 15 as usize) as IndexType);
    let mut obstacles = Obstacles::new();
    obstacles.add_obstacle(Obstacle::new_circle(10, 10, 10));
    obstacles.add_obstacle(Obstacle::new_rect(20, 0, 40, 20));
    obstacles.add_obstacle(Obstacle::new_rect(40, 0, 60, 25));
    obstacles.add_obstacle(Obstacle::new_transparent_rect(60, 0, 80, 25, 3));
    obstacles.add_obstacle(Obstacle::new_transparent_rect(60, 25, 80, 50, 2));
    obstacles.add_obstacle(Obstacle::new_frame(
        0,
        0,
        CANVAS_MAX_X as IndexType - 1,
        CANVAS_MAX_Y as IndexType - 1,
    ));
    obstacles.get_all_mut()[0].set_solid(false);
    obstacles.get_all_mut()[1].set_solid(false);
    obstacles.get_all_mut()[4].set_ground(true);
    obstacles.get_all_mut()[5].set_visible(false); // frame

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

        obstacles.draw(&mut canvas);
        tank.draw(&mut canvas);

        render_tx.send(canvas).unwrap();
        thread::sleep(Duration::from_millis(2));
    }

    drop(render_tx);
    terminal::deinit(&mut stdout).unwrap();
    thread_handle.join().unwrap();
}
