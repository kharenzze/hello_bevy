use bevy::prelude::*;

#[derive(Component)]
struct Name(String);

fn main() {
  App::new()
    .add_startup_system(setup)
    .add_plugins(DefaultPlugins)
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
  fn player_system(mut query: Query<(&Name, &mut Transform), With<Player>>) {
    for (name, mut transform) in query.iter_mut() {
      transform.translation.x += 2.;
    }
  }
}

const PLAYER_COLOR: Color = Color::rgb(0.25, 0.25, 0.75);

fn setup(mut commands: Commands) {
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  commands
    .spawn_bundle(SpriteBundle {
      sprite: Sprite {
        color: PLAYER_COLOR,
        custom_size: Some(Vec2::new(50.0, 50.0)),
        ..default()
      },
      ..default()
    })
    .insert(Player)
    .insert(Name("MyPlayer".to_string()));
}
