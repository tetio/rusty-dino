use bevy::{prelude::*, window::*};

const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 600.;
const DINO_SPEED: f32 = 500.;

#[derive(Component)]
struct Dino;

#[derive(Component)]
struct Mob;

#[derive(Component)]
struct Collider;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component, PartialEq, Eq, Deref, DerefMut)]
struct Facing(Direction);

#[derive(Component, Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let dino = Sprite::from_image(asset_server.load("dino.png"));
    let mob = Sprite::from_image(asset_server.load("mob.png"));

          
    commands.spawn(Camera2d);

    commands.spawn((
        dino,
        Transform::from_xyz(-250., 0., 0.),
        Dino,
        Collider,
        Facing(Direction::Right),
    ));

    commands.spawn((
        mob.clone(),
        Transform::from_xyz(0., 0., 0.),
        Mob,
        Velocity(Vec2::new(-250.0, 0.)),
        Collider,
    ));

    let mut transform = Transform::default();
    transform = transform.with_translation(Vec3::new(100., 0., 0.));
    transform.rotate(Quat::from_xyzw(0., -1., 0., 0.));
    commands.spawn((
        mob.clone(),
        transform,
        Mob,
        Velocity(Vec2::new(250.0, 0.)),
        Collider,
    ));
}

fn move_dino(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Facing), With<Dino>>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;
    let (mut transform, mut facing) = query.single_mut();

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        if *facing == Facing(Direction::Right) {
            transform.rotation *= Quat::from_xyzw(0., -1., 0., 0.);
            *facing = Facing(Direction::Left);
        }
        direction.x -= 1.;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        if *facing == Facing(Direction::Left) {
            transform.rotation *= Quat::from_xyzw(0., -1., 0., 0.);
            *facing = Facing(Direction::Right);
        }
        direction.x += 1.;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.;
    }
    transform.translation.x += direction.x * DINO_SPEED * time.delta_secs();
    transform.translation.y += direction.y * DINO_SPEED * time.delta_secs();
}

fn move_mobs(mut query: Query<(&mut Transform, &mut Velocity)>, time: Res<Time>) {
    for (mut transform, mut velocity) in &mut query {
        let x = transform.translation.x + velocity.x * time.delta_secs();
        let y = transform.translation.y + velocity.y * time.delta_secs();

        if x > WINDOW_WIDTH / 2. || x < -WINDOW_WIDTH / 2. {
            transform.rotation *= Quat::from_xyzw(0., -1., 0., 0.);
            velocity.x *= -1.;
        }
        if y > WINDOW_HEIGHT / 2. || y < -WINDOW_HEIGHT / 2. {
            velocity.y *= -1.;
        }
        transform.translation.x = x.clamp(-WINDOW_WIDTH / 2., WINDOW_WIDTH / 2.);
        transform.translation.y = y.clamp(-WINDOW_HEIGHT / 2., WINDOW_HEIGHT / 2.);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_dino, move_mobs).chain())
        .run();
}
