mod components;
mod plugins;
mod resources;

use crate::resources::MousePos;
use bevy::prelude::*;
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

type PlayerQ<'a> = (Mut<'a, Transform>, Mut<'a, TargetPosition>);
impl PlayerPlugin {
  fn player_system(
    mouse: Res<Input<MouseButton>>,
    mouse_pos: Res<MousePos>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut TargetPosition), With<Player>>,
  ) {
    for q in query.iter_mut() {
      let (mut transform, mut target_pos): PlayerQ = q;
      if mouse.just_pressed(MouseButton::Left) {
        let mouse_vec: &Vec2 = mouse_pos.deref();
        target_pos.set(*mouse_vec);
      }
      let t2d: Vec2 = transform.translation.truncate();
      let delta = time.delta_seconds();
      let rate = 2. * delta;
      let next = t2d.lerp(target_pos.clone(), rate);
      transform.translation = next.extend(0.);
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
