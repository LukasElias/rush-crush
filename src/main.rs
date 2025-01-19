use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

mod game_state;
mod walls;
mod colliders;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::index(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: false,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_plugins(game_state::GameStatePlugin)
        .add_plugins(walls::WallPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut _rapier_config: Query<&mut RapierConfiguration>,
    asset_server: Res<AssetServer>,
) {
    // rapier_config.single_mut().gravity = Vec2::new(0.0, 0.0);

    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        },
    ));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("tilemap.ldtk").into(),
        ..Default::default()
    });
}
