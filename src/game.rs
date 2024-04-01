use std::{default, iter::once};
// use bevy_flycam::prelude::*;
use bevy::{
    ecs::system::{Command, RunSystemOnce, SystemId}, math::vec3, prelude::*, render::camera::Viewport, transform::{self, TransformSystem}, winit::WinitSettings
};
// use bevy_flycam::prelude::*;
// use map::{Room, Rotation};
use iyes_perf_ui::{prelude::*, window};
use bevy::window::{CursorGrabMode, PrimaryWindow};
use crate::{camera::*, postprocessing};
use crate::map;
use bevy_rapier3d::prelude::*;



#[derive(Component)]
pub struct DespawnOnExit;

#[derive(Component)]
pub struct Rotates;

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
pub struct PlayerBody;

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
    // mut windowsettings: Query<ResMut<WinitSettings>>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {


    let mut window_size: (f32, f32) = (0.0, 0.0);

    if let Ok(mut window) = primary_window.get_single_mut() {
        window_size = (window.width(), window.height());
        toggle_grab_cursor(&mut window);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
    

    // commands.insert_resource(WinitSettings::game());
    commands.insert_resource(MovementSettings {
        sensitivity: 0.00002, // default: 0.00012
        speed: 0.25, // default: 12.0
    });
    commands.insert_resource(KeyBindings {
        move_ascend: KeyCode::Space,
        move_descend: KeyCode::ControlLeft,
        ..Default::default()
    });

    // main camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 2.0, 5.0))
                .looking_at(Vec3::default(), Vec3::Y),
            camera: Camera {
                clear_color: Color::WHITE.into(),
                order: 0,
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

    commands.spawn(PerfUiEntryFPS::default()).insert(DespawnOnExit);
    // commands.spawn(PerfUiRoot {}).insert(DespawnOnExit);

    // SECONDARY CAMERA
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(20.0, 0.0, 24.0))
                .looking_at(Vec3 { x: 20.0, y: 0.0, z: 20.0 }, Vec3::Y),
            projection: Projection::Orthographic(OrthographicProjection { scale: 0.04, ..Default::default()}),
            camera: Camera {
                viewport: Some(Viewport {
                    physical_position: UVec2 { x: (window_size.0 as u32 - 150), y: (0) },
                    physical_size: UVec2 { x: 150, y: 150 },
                    ..Default::default()
                }),
                clear_color: Color::BLACK.into(),
                order: 1,
                is_active: true,
                ..default()

            },
            ..default()
        },
        SecondCamera,
        DespawnOnExit,
    ));

    let mut rooms: Vec<map::Room> = map::generate_map(3);
    map::spawn_cubes_from_matrix(&mut commands, &mut meshes, &mut materials, &mut rooms, (20.0, 0.0, 20.0));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1_000.,
            ..default()
        },
        ..default()
    });


    // Load the mesh file
    // let model_handle: Handle<Mesh> = 
    // asset_server.get_asset_loader_with_extension(extension)

    // Spawn the object as a PBR entity (adjust if needed):
    commands.spawn(SceneBundle {
        scene: asset_server.load("objects/console.gltf#Scene0"),
        transform: Transform {
            scale: vec3(0.25, 0.25, 0.25),
            ..default()
        },
        // material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
        // transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });


}

pub fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::capsule_y(1.5, 1.0))
        .insert(Restitution::coefficient(0.9))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)))
        .insert(PlayerBody);
}

pub fn update_player_camera(
    mut player_camera: Query<(&mut Transform), With<MainCamera>>,
    mut player_body: Query<&Transform, (With<PlayerBody>, Without<MainCamera>)>
) {

    if let Ok(mut body_transform) = player_body.get_single_mut() {
        if let Ok(mut camera_transform) = player_camera.get_single_mut() {
            camera_transform.translation = vec3(body_transform.translation.x, body_transform.translation.y, body_transform.translation.z);
        }
    }


}


pub fn spawn_new_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    println!("SPAWNING NEW MAP");
    let mut new_rooms = map::generate_map(5);
    map::spawn_cubes_from_matrix(&mut commands, &mut meshes, &mut materials, &mut new_rooms, (0.0, 0.0, 0.0));
}



pub fn update_settings(mut settings: Query<&mut postprocessing::PostProcessSettings>, time: Res<Time>) {
    for mut setting in &mut settings {
        let mut intensity = (time.elapsed_seconds() * 3.0).sin();
        intensity = intensity.sin();
        intensity = intensity * 0.5 + 0.5;
        intensity *= 0.0025;

        setting.intensity = intensity;
    }
}

//                                                              SWITCH CAMERAS
// pub fn switch_cameras(
//     mut active_camera: ResMut<ActiveCamera>,
//     mut main_camera: Query<(Entity, &mut Camera), With<MainCamera>>, 
//     mut secondary_camera: Query<&mut Camera, Without<MainCamera>>,
//     // mut camera_entity: Query< With<MainCamera>>,
//     mut commands: Commands,
//     mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
// ) {


//     let mut window_size: (f32, f32) = (0.0, 0.0);

//     if let Ok(mut window) = primary_window.get_single_mut() {
//         window_size = (window.width(), window.height());
//         toggle_grab_cursor(&mut window);
//     } else {
//         warn!("Primary window not found for `initial_grab_cursor`!");
//     }

//     // set main camera
//     for (entity, mut camera) in main_camera.iter_mut() {
//         match *active_camera {
//             ActiveCamera::Primary => {
//                 camera.is_active = true;
//                 commands.entity(entity).insert(FlyCam);
//             },
//             ActiveCamera::Secondary => {
//                 camera.is_active = false;
//                 commands.entity(entity).remove::<FlyCam>();
//             },
//         }
//     }

//     // set other cameras
//     for mut camera in secondary_camera.iter_mut() {
//         match *active_camera {
//             ActiveCamera::Primary => {
//                 // camera.is_active = false;
//                 // camera.viewport = Some(Viewport {
//                 //     physical_position: UVec2 { x: (window_size.0 as u32 - 100), y: (0) },
//                 //     physical_size: UVec2 { x: 100, y: 100 },
//                 //     ..Default::default()
//                 // })
//                 let cam = camera.clone();
//                 let viewport = cam.viewport.unwrap();
//                 if viewport.physical_position.x != (window_size.0 as u32 - 100) {
//                     camera.viewport = Some(Viewport {
//                         physical_position: UVec2 { x: (window_size.0 as u32 - 100), y: (0) },
//                         physical_size: UVec2 { x: 100, y: 100 },
//                         ..Default::default()
//                     })
//                 }
//             },
//             ActiveCamera::Secondary => {
//                 // camera.is_active = true;
//                 let cam = camera.clone();
//                 let viewport = cam.viewport.unwrap();
//                 if viewport.physical_size.x != (window_size.0 as u32) {
//                     camera.viewport = Some(Viewport::default());
//                 }
//             },
//         }
//     }

// }



