use crate::resources::MousePos;
use bevy::prelude::*;

#[derive(Component)]
struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(MousePos::default())
      .add_system(camera_system);
  }
}

fn camera_system(
  wnds: Res<Windows>,
  q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
}
