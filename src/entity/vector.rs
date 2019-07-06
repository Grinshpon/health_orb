extern crate num;
use num::Num;

#[derive(PartialEq, PartialOrd, Clone)]
pub struct Vector3<N: Num> {
  pub x: N,
  pub y: N,
  pub z: N, //z used for 2d depth
}

pub struct Vector2<N: Num> {
  pub x: N,
  pub y: N
}
