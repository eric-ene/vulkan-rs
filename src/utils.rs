macro_rules! color {
    ($r:expr, $g:expr, $b:expr) => {
      Color::RGB($r, $g, $b)
    }
}

macro_rules! point {
    ($x:expr, $y:expr) => {
      Point::new($x as i32, $y as i32)
    };
}

macro_rules! rect {
    ($x:expr, $y:expr, $w:expr, $h:expr) => {
      Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    };
}

pub(crate) use {color, point, rect};
