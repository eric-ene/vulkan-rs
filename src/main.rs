mod camera;

use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::libc::time;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use crate::camera::Camera;

fn main() {
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();

  let window = video_subsystem
    .window("Pendulum thingy", 800, 600)
    .vulkan()
    .build()
    .unwrap();

  let mut canvas = window
    .into_canvas()
    .build()
    .map_err(|e| e.to_string()).unwrap();

  canvas.clear();
  canvas.present();

  let mut events = sdl_context.event_pump().unwrap();

  let (mut mx, mut my) = (0_f32, 0_f32);

  let mut camera = Camera::new(0.0, 0.0, canvas);
  let speed = 4.0_f64;

  let mut timer = sdl_context.timer().unwrap();
  'render: loop {
    for event in events.poll_iter() {
      match event {
        Event::Quit { .. }  => break 'render,
        Event::MouseMotion {x, y, .. } => {
          mx = x as f32;
          my = y as f32;
        },
        Event::KeyDown { keycode, .. } => {
          match keycode {
            Some(key) => match key {
              Keycode::Left => camera.translate(-speed, 0.0),
              Keycode::Right => camera.translate(speed, 0.0),
              Keycode::Up => camera.translate(0.0, -speed),
              Keycode::Down => camera.translate(0.0,  speed),
              _ => {}
            },
            None => {}
          }
        },
        _ => {}
      }
    }

    let time_seconds = timer.ticks() as f32 / 1000.0;
    camera.canvas.set_draw_color(Color::RGB(0, 0, 0));
    camera.canvas.clear();
    camera.canvas.set_draw_color(Color::RGB(200, 175, 255));

    camera.draw_rect(
      Rect::new(100, 100, 100, 100)
    );

    camera.canvas.present();
    std::thread::sleep(Duration::from_millis(1_000 / 60));
  }
}