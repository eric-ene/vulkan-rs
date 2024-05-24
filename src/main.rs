mod camera;
mod utils;

use std::thread::sleep;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::ttf;

use crate::camera::Renderer;

fn main() {
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();

  let window = video_subsystem
    .window("Tree :D", 800, 600)
    .vulkan()
    .build()
    .unwrap();

  let canvas = window
    .into_canvas()
    .build()
    .map_err(|e| e.to_string()).unwrap();

  let ttf_context = ttf::init().unwrap();

  let mut renderer = Renderer::new(0.0, 0.0, canvas);

  let font = ttf_context.load_font(
    "./data/font/JetBrainsMonoNerdFont-Thin.ttf",
    32
  ).unwrap();

  renderer.clear();
  renderer.present();

  let mut events = sdl_context.event_pump().unwrap();

  let (mut mx, mut my) = (0_f32, 0_f32);
  let (mut mx_prev, mut my_prev) = (0_f32, 0_f32);
  let mut mouse_down = false;

  let speed = 4.0;
  let zoom_speed = 0.1;

  let timer = sdl_context.timer().unwrap();
  let mut current_start = timer.ticks64() as f64;

  'render: loop {
    for event in events.poll_iter() {
      match event {
        Event::Quit { .. }  => break 'render,
        Event::MouseMotion {x, y, .. } => {
          mx = x as f32;
          my = y as f32;
        },
        Event::MouseButtonDown { .. } => {
          mouse_down = true;
        },
        Event::MouseButtonUp { .. } => {
          mouse_down = false;
        },
        Event::MouseWheel { y, .. } => {
          renderer.zoom(y as f64 / 10.0);
        }
        Event::KeyDown { keycode, .. } => {
          match keycode {
            Some(key) => match key {
              Keycode::Left => renderer.translate(-speed, 0.0),
              Keycode::Right => renderer.translate(speed, 0.0),
              Keycode::Up => renderer.translate(0.0, -speed),
              Keycode::Down => renderer.translate(0.0, speed),
              Keycode::Z => renderer.zoom(zoom_speed),
              Keycode::X => renderer.zoom(-zoom_speed),
              _ => {}
            },
            None => {}
          }
        },
        _ => {}
      }
    }

    renderer.set_draw_color(utils::color!(0, 0, 0));
    renderer.clear();
    renderer.set_draw_color(utils::color!(200, 175, 255));

    if mouse_down {
      let dx = mx_prev - mx;
      let dy = my_prev - my;

      renderer.translate(dx as f64, dy as f64);
    }

    let _ = renderer.draw_rect(utils::rect!(-50, -50, 100, 100));
    let _ = renderer.draw_line(utils::point!(50.0, 50.0), utils::point!(100.0, 100.0));
    let _ = renderer.draw_text(
      &font, "sample text",
      utils::point!(-200, -100),
      utils::color!(200, 175, 255)
    );

    (mx_prev, my_prev) = (mx, my);
    renderer.present();

    let current_end = timer.ticks64() as f64;
    let frame_time = current_end - current_start;
    current_start = timer.ticks64() as f64;

    if frame_time < (1000.0 / 60.0) {
      sleep(Duration::from_millis(((1000.0 / 60.0) - frame_time) as u64));

    }
  }
}