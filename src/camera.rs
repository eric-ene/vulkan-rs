use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct Camera {
  pos_x: f64,
  pos_y: f64,

  scale: f64,

  pub canvas: WindowCanvas
}

impl Camera {
  pub fn new (x: f64, y: f64, canvas: WindowCanvas) -> Self {
    Self {
      pos_x: x,
      pos_y: y,

      scale: 1.0,

      canvas: canvas
    }
  }

  pub fn translate(&mut self, dx: f64, dy: f64) {
    self.pos_x += dx;
    self.pos_y += dy;
  }

  pub fn draw_rect(&mut self, rect: Rect) -> Result<(), String> {
    let new_rect= Rect::new(
      rect.x - self.pos_x as i32,
      rect.y - self.pos_y as i32,
      rect.width(),
      rect.height()
    );

    return self.canvas.draw_rect(new_rect);
  }
}