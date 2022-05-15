use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

type Inner = Vec2;
#[derive(Component, Default)]
pub struct TargetPosition(Inner);

impl TargetPosition {
  pub fn set(&mut self, value: Inner) {
    self.0 = value;
  }
}

impl Deref for TargetPosition {
  type Target = Inner;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for TargetPosition {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
