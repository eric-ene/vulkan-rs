use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{TextureCreator, TextureQuery, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use crate::utils;


pub struct Renderer {
  pos_x: f64,
  pos_y: f64,

  scale: f64,

  pub canvas: WindowCanvas,
}

impl Renderer {
  pub fn new (x: f64, y: f64, canvas: WindowCanvas) -> Self {
    Self {
      pos_x: x,
      pos_y: y,

      scale: 1.0,

      canvas: canvas,
    }
  }

  pub fn translate(&mut self, dx: f64, dy: f64) {
    self.pos_x += dx / self.scale;
    self.pos_y += dy / self.scale;
  }

  pub fn zoom(&mut self, factor: f64) {
    self.scale += factor;
    self.scale = self.scale.clamp(0.1, 10.0);
  }

  pub fn draw_rect(&mut self, rect: Rect) -> Result<(), String> {
    let new_rect= Rect::new(
      (self.scale * (rect.x as f64 - self.pos_x)) as i32 + 400,
       (self.scale * (rect.y as f64 - self.pos_y)) as i32 + 300,
      (rect.width() as f64 * self.scale) as u32,
      (rect.height() as f64 * self.scale) as u32,
    );

    return self.canvas.draw_rect(new_rect);
  }

  pub fn draw_line(&mut self, p1: Point, p2: Point) -> Result<(), String> {
    let line_x1 = (self.scale * (p1.x as f64 - self.pos_x)) as i32 + 400;
    let line_y1 = (self.scale * (p1.y as f64 - self.pos_y)) as i32 + 300;

    let line_x2 = (self.scale * (p2.x as f64 - self.pos_x)) as i32 + 400;
    let line_y2 = (self.scale * (p2.y as f64 - self.pos_y)) as i32 + 300;

    return self.canvas.draw_line(
      Point::new(line_x1, line_y1),
      Point::new(line_x2, line_y2)
    )
  }

  pub fn draw_text(&mut self, font: &Font, text: &str, p: Point, color: Color) -> Result<(), String> {
    let text_surface = font
      .render(text)
      .blended(color)
      .unwrap();

    let texture_creator = self.texture_creator();

    let texture = text_surface
      .as_texture(&texture_creator)
      .unwrap();

    let TextureQuery { width, height, .. } = texture.query();

    let target = utils::rect!(
      self.scale * (p.x as f64 - self.pos_x) + 400.0,
      self.scale * (p.y as f64 - self.pos_y) + 300.0,
      width as f64 * self.scale,
      height as f64 * self.scale
    );

    return self.canvas.copy(&texture, None, Some(target));
  }

  pub fn set_draw_color(&mut self, color: Color) {
    return self.canvas.set_draw_color(color);
  }

  pub fn clear(&mut self) {
    return self.canvas.clear();
  }

  pub fn present(&mut self) {
    return self.canvas.present();
  }

  pub fn texture_creator(&self) -> TextureCreator<WindowContext> {
    return self.canvas.texture_creator();
  }
}