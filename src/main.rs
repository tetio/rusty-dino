use bevy::render::render_resource::{AsBindGroupShaderType, ShaderType};
use bevy::{math::prelude::*, prelude::*, window::*};

const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 600.;
const DINO_SPEED: f32 = 500.;
const DINO_SIZE: f32 = 64.;
const MOB_SIZE: f32 = 64.;
#[derive(Component)]
struct Dino;

#[derive(Component)]
struct DinoBox;

#[derive(Component)]
struct Mob;

#[derive(Component)]
struct Collider;

#[derive(Resource, Default)]
struct GameState {
    game_over: bool,
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut game_state: ResMut<GameState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    game_state.game_over = false;

    let dino = Sprite::from_image(asset_server.load("dino.png"));
    let mob_image = asset_server.load("mob.png");

    commands.spawn(Camera2d);

    commands.spawn((dino, Transform::from_xyz(-250., -25., 0.), Dino, Collider));
    let mesh = meshes.add(Rectangle::new(64., 64.));
    commands
        .spawn((
            Mesh2d(mesh),
            MeshMaterial2d(materials.add(Color::srgb(255., 0., 0.))),
            Transform::from_xyz(-250., -25., 0.),
            DinoBox,
        ))
        .insert(Visibility::Hidden);

    // commands.spawn((
    //     Sprite {
    //         image: mob_image.clone(),
    //         flip_x: false,
    //         ..default()
    //     },
    //     Transform::from_xyz(0., 0., 0.),
    //     Mob,
    //     Velocity(Vec2::new(-250.0, 0.)),
    //     Collider,
    // ));

    // commands.spawn((
    //     Sprite {
    //         image: mob_image.clone(),
    //         flip_x: false,
    //         ..default()
    //     },
    //     Transform::from_xyz(0., 0., 0.),
    //     Mob,
    //     Velocity(Vec2::new(-250.0, 0.)),
    //     Collider,
    // ));
    commands.spawn((
        Sprite {
            image: mob_image.clone(),
            flip_x: true,
            ..default()
        },
        Transform::from_xyz(100., -100., 0.),
        Mob,
        Velocity(Vec2::new(250.0, 0.)),
        Collider,
    ));

    commands.spawn((
        Sprite {
            image: mob_image.clone(),
            flip_x: true,
            ..default()
        },
        Transform::from_xyz(100., 0., 0.),
        Mob,
        Velocity(Vec2::new(250.0, 0.)),
        Collider,
    ));

    commands.spawn((
        Sprite {
            image: mob_image.clone(),
            flip_x: true,
            ..default()
        },
        Transform::from_xyz(-100., 0., 0.),
        Mob,
        Velocity(Vec2::new(250.0, 0.)),
        Collider,
    ));
}

fn move_dino(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &mut Sprite, &mut Transform), With<Dino>>,
    mut query_box: Query<&mut Transform, (With<DinoBox>, Without<Dino>)>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;
    let (entity, mut sprite, mut transform) = query.single_mut();
    let mut transform_box = query_box.single_mut();

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        if !sprite.flip_x {
            sprite.flip_x = true;
        }
        direction.x -= 1.;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        if sprite.flip_x {
            sprite.flip_x = false;
        }
        direction.x += 1.;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.;
    }

    if keyboard_input.pressed(KeyCode::Space) {
        commands.entity(entity).insert(Visibility::Hidden);
    }
    if keyboard_input.pressed(KeyCode::KeyV) {
        commands.entity(entity).insert(Visibility::Visible);
    }
    let mov_x = direction.x * DINO_SPEED * time.delta_secs();
    let mov_y = direction.y * DINO_SPEED * time.delta_secs();
    transform.translation.x += mov_x;
    transform.translation.y += mov_y;
    transform_box.translation.x += mov_x;
    transform_box.translation.y += mov_y;
}

fn move_mobs(mut query: Query<(&mut Sprite, &mut Transform, &mut Velocity)>, time: Res<Time>) {
    for (mut sprite, mut transform, mut velocity) in &mut query {
        let x = transform.translation.x + velocity.x * time.delta_secs();
        let y = transform.translation.y + velocity.y * time.delta_secs();

        if x > WINDOW_WIDTH / 2. || x < -WINDOW_WIDTH / 2. {
            sprite.flip_x = !sprite.flip_x;
            velocity.x *= -1.;
        }
        if y > WINDOW_HEIGHT / 2. || y < -WINDOW_HEIGHT / 2. {
            velocity.y *= -1.;
        }
        transform.translation.x = x.clamp(-WINDOW_WIDTH / 2., WINDOW_WIDTH / 2.);
        transform.translation.y = y.clamp(-WINDOW_HEIGHT / 2., WINDOW_HEIGHT / 2.);
    }
}

fn collision_detection(
    mut commands: Commands,
    dino_query: Query<&Transform, With<Dino>>,
    dino_box_query: Query<Entity, (With<DinoBox>, Without<Dino>)>,
    mobs_query: Query<&Transform, With<Mob>>,
    mut game_state: ResMut<GameState>,
) {
    let dino_transform = dino_query.single();
    let dino_rect = make_rect(dino_transform, DINO_SIZE);
    let dino_box_entity = dino_box_query.single();

    for mob_transform in mobs_query.iter() {
        let mob_rect = make_rect(mob_transform, MOB_SIZE);
        let ri = mob_rect.intersect(dino_rect);
        if !ri.is_empty() {
            game_state.game_over = true;
            let a = commands.entity(dino_box_entity).;
            commands.entity(dino_box_entity).insert(Visibility::Visible);
            break;
            // println!("{:?} WARNING! Collision detected!!", ri);
        } else {
            commands.entity(dino_box_entity).insert(Visibility::Hidden);
        }
    }
}

fn make_rect(dino_transform: &Transform, width: f32) -> Rect {
    let size = width / 2.;
    Rect::new(
        dino_transform.translation.x - size,
        dino_transform.translation.y - size,
        dino_transform.translation.x + size,
        dino_transform.translation.y + size,
    )
}

fn main() {
    App::new()
        .init_resource::<GameState>()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (move_dino, move_mobs, collision_detection).chain())
        .run();
}
