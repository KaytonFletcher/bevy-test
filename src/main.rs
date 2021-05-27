use bevy::prelude::*;

use bevy_rapier2d::{
    physics::{RapierConfiguration, RapierPhysicsPlugin},
    rapier::{
        dynamics::RigidBodyBuilder,
        geometry::{ColliderBuilder, InteractionGroups},
    },
};

use bevy_asset_loader::{AssetCollection, AssetLoader};

#[derive(AssetCollection)]
struct SpriteAssets {
    #[asset(path = "sprites/green_fighter.png")]
    pub follow_enemy: Handle<Texture>,
    #[asset(path = "sprites/pirate_ship.png")]
    pub player: Handle<Texture>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    InitialSpawn,
    Playing,
}

fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO.into();
    rapier_config.time_dependent_number_of_timesteps = true;

    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut state: ResMut<State<GameState>>,
    sprites: Res<SpriteAssets>,
) {
    const SCALE: f32 = 0.33;
    const SPRITE_DIM: f32 = 549.;
    const WIDTH: f32 = (SPRITE_DIM - 350.) * SCALE;
    const HEIGHT: f32 = (SPRITE_DIM - 80.) * SCALE;

    let mut player_builder = commands.spawn();

    let player_entity = player_builder.id();

    player_builder
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::new(SCALE, SCALE, 1.),
                ..Default::default()
            },
            material: materials.add(sprites.player.clone().into()),
            ..Default::default()
        })
        .insert(
            RigidBodyBuilder::new_dynamic()
                .linear_damping(0.5)
                .angular_damping(5.0)
                .user_data(player_entity.to_bits() as u128),
        )
        .insert(
            ColliderBuilder::cuboid(WIDTH / 2.0, HEIGHT / 2.0)
                .collision_groups(InteractionGroups::new(0x00100, 0x00001)),
        );

        state.set(GameState::Playing).unwrap();
}

fn spawn_follow_enemy(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    sprites: Res<SpriteAssets>,
) {
    const ENEMY_SCALE: f32 = 2.5;
    const ENEMY_WIDTH: f32 = 12.0 * ENEMY_SCALE;
    const ENEMY_HEIGHT: f32 = 20.0 * ENEMY_SCALE;

    let mut enemy_builder = commands.spawn();

    let enemy_entity = enemy_builder.id();

    enemy_builder
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(300.0, 300.0, 1.0),
                scale: Vec3::new(ENEMY_SCALE, ENEMY_SCALE, 1.0),
                ..Default::default()
            },
            material: materials.add(sprites.follow_enemy.clone().into()),
            ..Default::default()
        }.clone())
        .insert(
            RigidBodyBuilder::new_dynamic()
                .linear_damping(1.0)
                .angular_damping(6.0)
                .translation(300.0, 300.0)
                .user_data(enemy_entity.to_bits() as u128),
        )
        .insert(
            ColliderBuilder::cuboid(ENEMY_WIDTH / 2.0, ENEMY_HEIGHT / 2.0)
                .collision_groups(InteractionGroups::new(0x00001, 0x00110)),
        );
    println!("spawn enemy");
}
fn main() {
    let mut app = App::build();

    AssetLoader::new(GameState::AssetLoading, GameState::InitialSpawn)
        .with_collection::<SpriteAssets>()
        .build(&mut app);

    app.insert_resource(WindowDescriptor {
        title: "bevy test".to_string(),
        width: 1920.0,
        height: 1080.0,
        ..Default::default()
    })
    .add_plugin(RapierPhysicsPlugin)
    .add_plugins(DefaultPlugins)
    .add_state(GameState::AssetLoading)
    .add_system_set(
        SystemSet::on_enter(GameState::InitialSpawn)
            .with_system(setup.system().label("setup"))
            .with_system(spawn_player.system().label("player").after("setup")), // .with_system(ui:spawn_player_ui.system().after("player")),
    )
    .add_system_set(
        SystemSet::on_enter(GameState::Playing).with_system(spawn_follow_enemy.system()),
    )
    .run();
}
