#[derive(Clone)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8
}

pub static Red: Color = Color { r: 255, g: 0, b: 0 };
pub static Green: Color = Color { r: 0, g: 255, b: 0 };
pub static Blue: Color = Color { r: 0, g: 0, b: 255 };
pub static White: Color = Color { r: 255, g: 255, b: 255 };
pub static Black: Color = Color { r: 0, g: 0, b: 0 };
