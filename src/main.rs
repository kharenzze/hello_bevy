mod components;
mod plugins;
mod resources;

use crate::resources::MousePos;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use components::target::TargetPosition;
use plugins::camera::CameraPlugin;
use std::ops::Deref;

fn main() {
  App::new()
    .add_startup_system(setup)
    .add_plugins(DefaultPlugins)
    .add_plugin(CameraPlugin)
    .add_plugin(PlayerPlugin)
    .run();
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(PlayerPlugin::player_system);
  }
}

#[derive(Component)]
struct Player;

impl PlayerPlugin {
  fn player_system(
    mouse: Res<Input<MouseButton>>,
    mouse_pos: Res<MousePos>,
    mut query: Query<(&mut Transform, &mut TargetPosition), With<Player>>,
  ) {
    for (mut transform, target_pos) in query.iter_mut() {
      if mouse.just_pressed(MouseButton::Left) {
        let mouse_vec: &Vec2 = mouse_pos.deref();
        target_pos.set(mouse_vec.extend(0.))
      }
      transform.translation.x += 2.;
    }
  }
}

const PLAYER_COLOR: Color = Color::rgb(0.25, 0.25, 0.75);

fn setup(mut commands: Commands) {
  commands
    .spawn_bundle(SpriteBundle {
      sprite: Sprite {
        color: PLAYER_COLOR,
        custom_size: Some(Vec2::new(50.0, 50.0)),
        ..default()
      },
      ..default()
    })
    .insert(TargetPosition::default())
    .insert(Player);
}
