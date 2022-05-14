use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

#[derive(Component, Default)]
pub struct TargetPosition(Vec3);

impl Deref for TargetPosition {
  type Target = Vec3;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for TargetPosition {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
