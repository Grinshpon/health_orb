extern crate sdl2;
//pub mod vector;
//use crate::vector::{Vector3,Vector2};
use sdl2::render::Texture;
use sdl2::rect::Rect;

pub struct Entity<'a> {
  pub obj: Texture<'a>,
  pub z: i32,
  pub src: Rect,
  pub dst: Rect
}
impl <'a> Entity<'a> {
  pub fn x(&self) -> i32 {
    self.dst.x
  }
  pub fn y(&self) -> i32 {
    self.dst.y
  }
  #[allow(non_snake_case)]
  pub fn setX(&mut self, n: i32) {
    self.dst.x = n;
  }
  #[allow(non_snake_case)]
  pub fn setY(&mut self, n: i32) {
    self.dst.y = n;
  }
  pub fn set_coords(&mut self, n: i32, m: i32) {
    self.setX(n);
    self.setY(m);
  }
  pub fn width(&self) -> u32 {
    self.dst.width()
  }
  pub fn height(&self) -> u32 {
    self.dst.height()
  }
  pub fn set_width(&mut self, n: u32) {
    self.dst.set_width(n);
  }
  pub fn set_height(&mut self, n: u32) {
    self.dst.set_width(n);
  }
  pub fn set_dim(&mut self, n: u32, m: u32) {
    self.set_width(n);
    self.set_height(m);
  }
  pub fn dst(&self) -> Rect {//rename frame()
    self.dst
  }
  pub fn set_dst(&mut self, r: Rect) {
    self.dst = r;
  }
  //include methods for src (name it `slice` or `quad`)
}
