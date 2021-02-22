use crate::quad::rectangle;

pub fn square(l: u32) -> rectangle::Rect {
  rectangle::Rect {
    length: l,
    width: l,
  }
}
