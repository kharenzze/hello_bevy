use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

struct MousePos(Vec2);

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
