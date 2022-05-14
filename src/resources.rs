use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct MousePos(Vec2);

impl MousePos {
  pub fn set(&mut self, v: Vec2) {
    self.0 = v;
  }
}

impl Deref for MousePos {
  type Target = Vec2;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for MousePos {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
