use std::time::Duration;
use sdl2::event::Event;
use sdl2::libc::time;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

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

  let mut lines = Vec::new();
  let step = 30;

  for x in (30..771).step_by(step) {
    for y in (30..571).step_by(step) {
      lines.push(FollowLine::new(x, y, 30_f32));
    }
  }

  let mut theta_zero = 1.0;
  let pendulum_length = 60.0;
  let speed_factor = 20.0;

  let mut timer = sdl_context.timer().unwrap();
  'render: loop {
    for event in events.poll_iter() {
      match event {
        Event::Quit { .. }  => break 'render,
        Event::MouseMotion {x, y, .. } => {
          mx = x as f32;
          my = y as f32;
        },
        _ => {}
      }
    }

    let time_seconds = timer.ticks() as f32 / 1000.0;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(200, 150, 255));
    //println!("{}", timer.ticks() as f32 / 1000_f32);
    let (x1, y1) = (400, 300);

    let omega = (speed_factor * 9.81_f32 / pendulum_length).sqrt();
    let theta = theta_zero * (omega * time_seconds).cos();

    let x2 = x1 + (pendulum_length * (theta + (3.14159 / 2.0)).cos()) as i32;
    let y2 = y1 + (pendulum_length * (theta + (3.14159 / 2.0)).sin()) as i32;

    canvas.draw_line(
      Point::new(x1, y1),
      Point::new(x2, y2)
    );

    canvas.draw_rect(Rect::new(x2 - 2, y2 - 2, 4, 4));
    
    canvas.present();
    std::thread::sleep(Duration::from_millis(1_000 / 60));
  }
}

struct FollowLine {
  origin: Point,
  dest: Point,

  length: f32
}

impl FollowLine {
  pub fn new(x: i32, y: i32, len: f32) -> Self {
    Self {
      origin: Point::new(x, y),
      dest: Point::new(x, y),
      length: len,
    }
  }

  pub fn update_pos(&mut self, mx: f32, my: f32) {
    let x1 = self.origin.x as f32;
    let y1 = self.origin.y as f32;

    let dist = ((mx - x1).powi(2) + (my - y1).powi(2)).abs().sqrt();
    let dist_x = (mx - x1) / dist * self.length;
    let dist_y = (my - y1) / dist * self.length;

    self.dest = Point::new((x1 + dist_x) as i32, (y1 + dist_y) as i32);
  }

  pub fn render(&self, canvas: &mut WindowCanvas) {
    canvas.draw_line(
      self.origin,
      self.dest
    ).unwrap();
  }
}