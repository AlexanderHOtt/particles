//! Renders a 2D scene containing a single, moving sprite.

use bevy::{prelude::*, utils::HashMap};
use rand::random;

const G: f32 = 100.;
const R_MIN: f32 = 15.;
const R_MAX: f32 = 50.;
const REPELLING_FORCE: f32 = 5000.;
const NUM_PARTICLES: usize = 200;
const VELOCITY_HALFLIFE: f32 = 0.043;

const R_DIFF: f32 = R_MAX - R_MIN;
const R_DIFF2: f32 = R_DIFF / 2.;

#[derive(Component, Eq, PartialEq, Hash)]
enum Spin {
    Red,
    Blue,
    Green,
}
struct AttractionMapMap {
    map: HashMap<Spin, HashMap<Spin, f32>>,
}

impl AttractionMapMap {
    fn new() -> Self {
        let mut m = HashMap::new();
        m.insert(Spin::Red, HashMap::new());
        m.insert(Spin::Blue, HashMap::new());
        m.insert(Spin::Green, HashMap::new());

        m.get_mut(&Spin::Red).unwrap().insert(Spin::Red, 1.0);
        m.get_mut(&Spin::Red).unwrap().insert(Spin::Blue, -1.);
        m.get_mut(&Spin::Red).unwrap().insert(Spin::Green, 0.5);

        m.get_mut(&Spin::Blue).unwrap().insert(Spin::Red, 0.5);
        m.get_mut(&Spin::Blue).unwrap().insert(Spin::Blue, 1.);
        m.get_mut(&Spin::Blue).unwrap().insert(Spin::Green, -1.);

        m.get_mut(&Spin::Green).unwrap().insert(Spin::Red, -1.);
        m.get_mut(&Spin::Green).unwrap().insert(Spin::Blue, 0.5);
        m.get_mut(&Spin::Green).unwrap().insert(Spin::Green, 1.);

        // m.get_mut(&Spin::Red).unwrap().insert(Spin::Red, 2.0);
        // m.get_mut(&Spin::Red).unwrap().insert(Spin::Blue, -1.);
        // m.get_mut(&Spin::Red).unwrap().insert(Spin::Green, 3.);

        // m.get_mut(&Spin::Blue).unwrap().insert(Spin::Red, 1.5);
        // m.get_mut(&Spin::Blue).unwrap().insert(Spin::Blue, 1.);
        // m.get_mut(&Spin::Blue).unwrap().insert(Spin::Green, -1.);

        // m.get_mut(&Spin::Green).unwrap().insert(Spin::Red, -1.);
        // m.get_mut(&Spin::Green).unwrap().insert(Spin::Blue, 0.5);
        // m.get_mut(&Spin::Green).unwrap().insert(Spin::Green, 10.);

        Self { map: m }
    }

    fn get(&self, color1: &Spin, color2: &Spin) -> f32 {
        self.map.get(color1).unwrap().get(color2).unwrap().clone()
    }
}

impl Resource for AttractionMapMap {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        // .add_system(sprite_movement)
        .add_system(apply_g)
        .insert_resource(AttractionMapMap::new())
        .run();
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    for _ in 0..NUM_PARTICLES {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.75, 0.25, 0.25),
                    custom_size: Some(Vec2::new(5.0, 5.0)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    random::<f32>() * 1280. - 640.,
                    random::<f32>() * 720. - 360.,
                    0.,
                ),
                // transform: Transform::from_xyz(
                //     random::<f32>() * 400. - 200.,
                //     random::<f32>() * 400. - 200.,
                //     0.,
                // ),
                ..default()
            },
            Spin::Red,
            Velocity { x: 0., y: 0. },
        ));
    }

    for _ in 0..NUM_PARTICLES {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(5.0, 5.0)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    random::<f32>() * 1280. - 640.,
                    random::<f32>() * 720. - 360.,
                    0.,
                ),
                ..default()
            },
            Spin::Blue,
            Velocity { x: 0., y: 0. },
        ));
    }

    for _ in 0..NUM_PARTICLES {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.75, 0.25),
                    custom_size: Some(Vec2::new(5.0, 5.0)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    random::<f32>() * 1280. - 640.,
                    random::<f32>() * 720. - 360.,
                    0.,
                ),
                ..default()
            },
            Spin::Green,
            Velocity { x: 0., y: 0. },
        ));
    }

    // commands.spawn((
    //     SpriteBundle {
    //         // texture: asset_server.load("branding/icon.png"),
    //         sprite: Sprite {
    //             color: Color::rgb(0.75, 0.25, 0.25),
    //             custom_size: Some(Vec2::new(5.0, 5.0)),
    //             ..default()
    //         },
    //         transform: Transform::from_xyz(10., 0., 0.),
    //         ..default()
    //     },
    //     MyColor::Red,
    //     Velocity { x: 0., y: 0. },
    // ));
    // commands.spawn((
    //     SpriteBundle {
    //         // texture: asset_server.load("branding/icon.png"),
    //         sprite: Sprite {
    //             color: Color::rgb(0.75, 0.25, 0.25),
    //             custom_size: Some(Vec2::new(5.0, 5.0)),
    //             ..default()
    //         },
    //         transform: Transform::from_xyz(-10., 0., 0.),
    //         ..default()
    //     },
    //     MyColor::Red,
    //     Velocity { x: 0., y: 0. },
    // ));
    // commands.spawn((
    //     SpriteBundle {
    //         // texture: asset_server.load("branding/icon.png"),
    //         sprite: Sprite {
    //             color: Color::rgb(0.25, 0.25, 0.75),
    //             custom_size: Some(Vec2::new(5.0, 5.0)),
    //             ..default()
    //         },
    //         transform: Transform::from_xyz(0., -10., 0.),
    //         ..default()
    //     },
    //     MyColor::Blue,
    //     Velocity { x: 0., y: 0. },
    // ));
}

fn apply_g(
    attraction_map: Res<AttractionMapMap>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity, &Spin)>,
) {
    // F_g = G * m1 * m2 / r * r
    // m1 = 1, m2 = 1
    // F_g = G / r * r

    // let G = 6.67408e-1;
    let mut forces: HashMap<usize, Vec3> = HashMap::new();

    for (i, (transform_i, _, color_i)) in query.iter().enumerate() {
        for (j, (transform_j, _, color_j)) in query.iter().enumerate() {
            if i == j {
                continue;
            }

            let mut force_g = calculate_force(
                transform_i.translation,
                transform_j.translation,
                attraction_map.get(color_i, color_j),
            );

            let total_force = forces.get(&i).unwrap_or(&Vec3::ZERO);
            force_g += *total_force;

            // println!("force: {force_g}");

            forces.insert(i, force_g);
        }

        // match *color {
        //     MyColor::Red => transform.translation.x += 150. * time.delta_seconds(),
        //     MyColor::Green => transform.translation.x -= 150. * time.delta_seconds(),
        //     MyColor::Blue => transform.translation.x -= 50. * time.delta_seconds(),
        // }
    }

    for (i, (mut transform, mut velocity, _)) in query.iter_mut().enumerate() {
        let force = forces.get(&i).unwrap_or(&Vec3::ZERO);

        velocity.x *= calculate_friction(time.delta_seconds());
        velocity.y *= calculate_friction(time.delta_seconds());

        velocity.x += force.x * time.delta_seconds();
        velocity.y += force.y * time.delta_seconds();

        let x = transform.translation.x;
        let y = transform.translation.y;

        transform.translation.x = transform.translation.x + velocity.x * time.delta_seconds();
        transform.translation.y = transform.translation.y + velocity.y * time.delta_seconds();
        // println!("{}: {} {}", i, velocity.x, velocity.y);

        if x > 640. {
            transform.translation.x = -640.;
        } else if x < -640. {
            transform.translation.x = 640.;
        }

        if y > 360. {
            transform.translation.y = -360.;
        } else if y < -360. {
            transform.translation.y = 360.;
        }
    }
}

fn calculate_force(i: Vec3, j: Vec3, factor: f32) -> Vec3 {
    let direction = (j - i).normalize();
    let distance = (j - i).length();

    if distance < R_MIN {
        return ((REPELLING_FORCE * distance / R_MIN) - REPELLING_FORCE) * direction;
    } else if distance < R_MAX {
        return factor
            * (2. * G / R_DIFF)
            * ((distance - R_MIN + R_DIFF2).abs() + R_DIFF2)
            * direction;
    } else {
        return Vec3::ZERO;
    }
}

fn calculate_friction(dt: f32) -> f32 {
    (0.5 as f32).powf(dt / VELOCITY_HALFLIFE)
}
