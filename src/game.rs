use std::{default, iter::once};

use bevy::{
    ecs::system::{Command, RunSystemOnce, SystemId},
    math::vec3, 
    prelude::*, 
    transform::TransformSystem,
};
use bevy_flycam::prelude::*;
// use map::{Room, Rotation};
use iyes_perf_ui::prelude::*;

use crate::postprocessing;
use crate::map;



#[derive(Component)]
pub struct DespawnOnExit;

#[derive(Component)]
pub struct Rotates;

#[derive(Component)]
pub struct MapRoom;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct SecondCamera;

#[derive(Resource)]
pub enum ActiveCamera {
    Primary,
    Secondary
}

#[derive(Component)]
pub struct MapParent;

#[derive(Component)]
pub struct CameraRef;


pub fn despawn_all(entities: Query<Entity, With<DespawnOnExit>>, mut commands: Commands) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

//                                                                      GAME SETUP
pub fn game_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // main camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 2.0, 5.0))
                .looking_at(Vec3::default(), Vec3::Y),
            camera: Camera {
                clear_color: Color::WHITE.into(),
                order: 1,
                is_active: true,
                ..default()
            },
            ..default()
        },
        postprocessing::PostProcessSettings {
            intensity: 0.02,
            sigma1: 8.0,
            tau: 0.01,
            gfact: 8.0,
            epsilon: 0.0001,
            num_gvf_iterations: 15,
            enable_xdog: 1,
        },
        FlyCam,
        MainCamera,
        DespawnOnExit,
    ));

    commands.spawn(PerfUiCompleteBundle::default()).insert(DespawnOnExit);

    // SECONDARY CAMERA
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 4.0))
                .looking_at(Vec3::default(), Vec3::Y),
            projection: Projection::Orthographic(OrthographicProjection { scale: 0.005, ..Default::default()}),
            camera: Camera {
                clear_color: Color::BLACK.into(),
                order: 0,
                is_active: false,
                ..default()

            },
            ..default()
        },
        SecondCamera,
        DespawnOnExit,
    ));

    let mut rooms: Vec<map::Room> = map::generate_map(3);
    spawn_cubes_from_matrix(&mut commands, &mut meshes, &mut materials, &mut rooms);

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1_000.,
            ..default()
        },
        ..default()
    });
}


pub fn spawn_new_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    println!("SPAWNING NEW MAP");
    let mut new_rooms = map::generate_map(5);
    spawn_cubes_from_matrix(&mut commands, &mut meshes, &mut materials, &mut new_rooms);
}



pub fn update_settings(mut settings: Query<&mut postprocessing::PostProcessSettings>, time: Res<Time>) {
    for mut setting in &mut settings {
        let mut intensity = (time.elapsed_seconds() * 3.0).sin();
        intensity = intensity.sin();
        intensity = intensity * 0.5 + 0.5;
        intensity *= 0.005;

        setting.intensity = intensity;
    }
}

//                                                              SWITCH CAMERAS
pub fn switch_cameras(
    mut active_camera: ResMut<ActiveCamera>,
    mut main_camera: Query<(Entity, &mut Camera), With<MainCamera>>, 
    mut secondary_camera: Query<&mut Camera, Without<MainCamera>>,
    // mut camera_entity: Query< With<MainCamera>>,
    mut commands: Commands,
) {

    // set main camera
    for (entity, mut camera) in main_camera.iter_mut() {
        match *active_camera {
            ActiveCamera::Primary => {
                camera.is_active = true;
                commands.entity(entity).insert(FlyCam);
            },
            ActiveCamera::Secondary => {
                camera.is_active = false;
                commands.entity(entity).remove::<FlyCam>();
            },
        }


        
    }

    // set other cameras
    for mut camera in secondary_camera.iter_mut() {
        match *active_camera {
            ActiveCamera::Primary => {
                camera.is_active = false;
            },
            ActiveCamera::Secondary => {
                camera.is_active = true;
            },
        }
    }

}

pub fn rotate_map(
    mut map_parent: Query<&mut Transform, (With<MapParent>, Without<MainCamera>)>,
    // camera: Query<&Transform, (With<MainCamera>)>,
) {

    // let mut r: Quat = Quat::default();
    // for transform in camera.iter() {
    //     r = transform.rotation;
    // }

    for mut parent in map_parent.iter_mut() {
        parent.rotate_axis(Vec3 { x: 0.0, y: 1.0, z: 0.0 }, 0.0015);
    }

}

//                                                              SPAWN MAP CUBES
pub fn spawn_cubes_from_matrix(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    rooms: &mut Vec<map::Room>,
) {

    let first_room = rooms.remove(0);
    let (dx, dy, dz) = first_room.dimensions;
    let (x, y, z) = first_room.position.unwrap();

    let offset = calculate_offset((dx as usize, dy as usize, dz as usize));
    let from_origin = 5.0;
    let spacing = 1.0;

    let px = ((x as f32 * spacing) - 1.0) - (from_origin);
    let py = ((y as f32 * spacing)) - (from_origin);
    let pz = ((z as f32 * spacing) - 1.0) - (from_origin);

    let mut transform = Transform::from_xyz(px, py, pz);

    transform.rotation = match first_room.rotation {
        map::Rotation::None => Quat::IDENTITY,
    };

    println!("{:?}", transform.rotation);

    let mut map: Entity = commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::from_size(vec3((dx * 0.95), (dy * 0.95), (dz * 0.95)))), // Use Cuboid if needed
            material: materials.add(Color::rgb(first_room.color[0], first_room.color[1], first_room.color[2])), // Adjust color as needed
            // transform,
            ..default()
        },
        MapParent,
        MapRoom,
        DespawnOnExit,
    )).id(); 

    {
        println!("{}", rooms.len());
        for room in rooms {
            let (dx, dy, dz) = room.dimensions;
            let (x, y, z) = room.position.unwrap();
    
            let offset = calculate_offset((dx as usize, dy as usize, dz as usize));
            let from_origin = 5.0;
            let spacing = 1.0;
    
            let px = ((x as f32 * spacing) - offset.0 + 0.5) - (from_origin);
            let py = ((y as f32 * spacing)) - (from_origin);
            let pz = ((z as f32 * spacing) - offset.2 + 0.5) - (from_origin);
    
            let mut transform = Transform::from_xyz(px, py, pz);
    
            transform.rotation = match room.rotation {
                map::Rotation::None => Quat::IDENTITY,
            };
    
            println!("{:?}", transform.translation);
    
            let child = commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::from_size(vec3((dx * 0.95), (dy * 0.95), (dz * 0.95)))), // Use Cuboid if needed
                    material: materials.add(Color::rgb(room.color[0], room.color[1], room.color[2])), // Adjust color as needed
                    transform,
                    ..default()
                },
                MapRoom
            )).id(); 
    
            commands.entity(map).add_child(child);
        }
    }
    
}

pub fn calculate_offset(dimentions: (usize, usize, usize)) -> (f32, f32, f32) {
    (dimentions.0 as f32 / 2.0, dimentions.1 as f32 / 2.0, dimentions.2 as f32 / 2.0,)
}