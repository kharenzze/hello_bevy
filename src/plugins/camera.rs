use crate::resources::MousePos;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

#[derive(Component)]
struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(MousePos::default())
      .add_startup_system(camera_startup_system)
      .add_system(camera_system);
  }
}

fn camera_startup_system(mut cmd: Commands) {
  cmd
    .spawn_bundle(OrthographicCameraBundle::new_2d())
    .insert(MainCamera);
}

type CameraQuery<'a> = (&'a Camera, &'a GlobalTransform);
fn camera_system(
  wnds: Res<Windows>,
  mut mouse_pos: ResMut<MousePos>,
  q_camera: Query<CameraQuery, With<MainCamera>>,
) {
  let (camera, camera_transform): CameraQuery = q_camera.single();
  let wnd = if let RenderTarget::Window(id) = camera.target {
    wnds.get(id).unwrap()
  } else {
    wnds.get_primary().unwrap()
  };

  if let Some(screen_pos) = wnd.cursor_position() {
    let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
    let world_pos: Vec2 = world_pos.truncate();
    println!("{world_pos}");
    mouse_pos.set(world_pos);
  }
}
