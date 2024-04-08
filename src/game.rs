use std::{default, f32::consts::PI, iter::once};
// use bevy_flycam::prelude::*;
use bevy::{
    ecs::{entity, system::{Command, RunSystemOnce, SystemId}}, tasks::IoTaskPool, math::vec3, prelude::*, render::camera::Viewport, text, transform::{self, TransformSystem}, winit::WinitSettings
};
// use bevy_flycam::prelude::*;
// use map::{Room, Rotation};
use iyes_perf_ui::{prelude::*, window};
use bevy::window::{CursorGrabMode, PrimaryWindow};
use crate::{camera::*, postprocessing};
use crate::map;
use crate::console;
use bevy_rapier3d::{parry::query::Ray, prelude::*};

use serde::{Deserialize, Serialize};
use bevy_renet::{
    client_connected,
    renet::{
        transport::{ClientAuthentication, ServerAuthentication, ServerConfig},
        ConnectionConfig, DefaultChannel, RenetClient, RenetServer, ServerEvent,
    },
    transport::{NetcodeClientPlugin, NetcodeServerPlugin},
    RenetClientPlugin, RenetServerPlugin,
};
use renet::{
    transport::{NetcodeClientTransport, NetcodeServerTransport, NetcodeTransportError},
    ClientId,
};
use std::{collections::HashMap, net::UdpSocket};

#[derive(Debug, Default, Serialize, Deserialize, Component, Resource)]
struct PlayerInput {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

#[derive(Debug, Component)]
struct Player {
    id: ClientId,
}

#[derive(Debug, Default, Resource)]
struct Lobby {
    players: HashMap<ClientId, Entity>,
}

#[derive(Debug, Serialize, Deserialize, Component)]
enum ServerMessages {
    PlayerConnected { id: ClientId },
    PlayerDisconnected { id: ClientId },
}


#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum OnlineState {
    #[default]
    Offline,
    Host,
    Client,
}


// #[derive(Resource)]
// pub struct OnlineState {
//     number_of_players: isize,
    
// }




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

#[derive(Component)]
pub struct Furniture;

#[derive(Component)]
pub struct UICamera;

#[derive(Component)]
pub struct UIInteractText;




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

    let root = make_defualt_directory();
    let dir = root.root.clone();
    commands.insert_resource(root);
    commands.insert_resource(console::CurrentDirectory(dir));


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

    commands.spawn((
        PerfUiRoot::default(),
        PerfUiEntryFPS::default(),
        PerfUiEntryFrameTime::default(),
        PerfUiEntryEntityCount::default(),
        PerfUiEntryRunningTime::default(),
    ));

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

    // commands.spawn(Camera2dBundle {
    //     camera: Camera {
    //         order: 1,
    //         ..default()
    //     },
    //     ..default()
    // }).insert(UICamera);

    commands
    .spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        // text
        parent.spawn((
            TextBundle::from_section(
                "hello :3",
                TextStyle {
                    font: asset_server.load("fonts/KodeMono-Regular.ttf"),
                    font_size: 30.0,
                    color: Color::BLACK.into(),
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect::all(Val::Px(5.)),
                ..default()
            }),
            Label,
        ));

        parent.spawn((
            TextBundle::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/KodeMono-Regular.ttf"),
                    font_size: 20.0,
                    color: Color::BLACK.into(),
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect::all(Val::Px(5.)),
                align_content: AlignContent::Center,
                ..default()
            }),
            Label,
            UIInteractText,
        ));
    });

    
    // commands.spawn(PerfUiRoot {}).insert(DespawnOnExit);

    // SECONDARY CAMERA
    // commands.spawn((
    //     Camera3dBundle {
    //         transform: Transform::from_translation(Vec3::new(20.0, 0.0, 24.0))
    //             .looking_at(Vec3 { x: 20.0, y: 0.0, z: 20.0 }, Vec3::Y),
    //         projection: Projection::Orthographic(OrthographicProjection { scale: 0.04, ..Default::default()}),
    //         camera: Camera {
    //             viewport: Some(Viewport {
    //                 physical_position: UVec2 { x: (window_size.0 as u32 - 150), y: (0) },
    //                 physical_size: UVec2 { x: 150, y: 150 },
    //                 ..Default::default()
    //             }),
    //             clear_color: Color::BLACK.into(),
    //             order: 1,
    //             is_active: true,
    //             ..default()

    //         },
    //         ..default()
    //     },
    //     SecondCamera,
    //     DespawnOnExit,
    // ));

    let mut rooms: Vec<map::Room> = map::generate_map(3);
    map::spawn_cubes_from_matrix(&mut commands, &mut meshes, &mut materials, &mut rooms, (20.0, 0.0, 20.0));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1_000.,
            ..default()
        },
        ..default()
    });

    // Spawn the object as a PBR entity (adjust if needed):
    let mut transform: Transform = Transform {
        translation: vec3(0.0, 0.3, -1.0),
        scale: vec3(0.25, 0.25, 0.25),
        rotation: Quat::from_axis_angle(vec3(0.0, 1.0, 0.0), PI)
    };


    transform.rotate_y(0.17);

    console::spawn_console(transform, String::from("primary"), &asset_server, &mut commands, &mut meshes, &mut materials);


    let transform2 = Transform {
        translation: vec3(0.0, 0.0, -0.6),
        scale: vec3(0.25, 0.15, 0.35),
        ..default()
    };

    let table = commands.spawn(SceneBundle {
        scene: asset_server.load("objects/table2.gltf#Scene0"),
        transform: transform2,
        ..default()
    }).insert(Furniture).id();
}



//                                                           SETUP PHYSICS

pub fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::capsule_y(1.5, 0.25))
        .insert(Restitution::coefficient(1.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 1.5, 0.0)))
        .insert(PlayerBody);
}

pub fn update_player_camera(
    mut player_camera: Query<(&mut Transform), With<MainCamera>>,
    mut player_body: Query<&Transform, (With<PlayerBody>, Without<MainCamera>)>
) {

    if let Ok(mut body_transform) = player_body.get_single_mut() {
        if let Ok(mut camera_transform) = player_camera.get_single_mut() {
            camera_transform.translation = vec3(body_transform.translation.x, body_transform.translation.y+1.0, body_transform.translation.z);
        }
    }


}


//                                                    SPAWN NEW MAP
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





//                                           MAKE DEFAULT DIRECTORY

pub fn make_defualt_directory () -> console::GameDirectory {
    
    let mut root = console::GameDirectory {
        root: console::Directory::new(String::from("root/"))
    };

    _=root.root.add_dir(String::from("root/users/")).unwrap();
    _=root.root.add_dir(String::from("root/system/")).unwrap();
    _=root.root.add_dir(String::from("root/programs/")).unwrap();
    _=root.root.add_dir(String::from("root/files/")).unwrap();

    let users = root.root.get_dir("root/users/").unwrap();
    _=users.add_dir(String::from("root/users/steve/")).unwrap();
    _=users.add_dir(String::from("root/users/sumi/")).unwrap();

    let programs = root.root.get_dir("root/programs/").unwrap();
    _=programs.add_program(String::from("root/programs/save.exe")).unwrap();
    _=programs.add_program(String::from("root/programs/decend.exe")).unwrap();

    let files = root.root.get_dir("root/files/").unwrap();
    _=files.add_dir(String::from("root/files/floors/")).unwrap();
    _=files.add_dir(String::from("root/files/enemies/")).unwrap();
    _=files.add_dir(String::from("root/files/items/")).unwrap();

    let floors = files.get_dir("root/files/floors/").unwrap();
    _=floors.add_file(String::from("root/files/floors/1.txt")).unwrap();
    _=floors.add_file(String::from("root/files/floors/2.txt")).unwrap();
    _=floors.add_file(String::from("root/files/floors/3.txt")).unwrap();

    return root;
}




#[derive(Component)]
pub enum InteractionType {
    Console,
    Item,
    Door
}



#[derive(Component)]
pub struct Interactable;


pub fn check_for_interactions(
    player_query: Query<(&GlobalTransform, &Camera), With<MainCamera>>,
    interaction_query: Query<(Entity, &InteractionType), With<Interactable>>,
    rapier_context: Res<RapierContext>,
    player_collider: Query<Entity, With<PlayerBody>>,
    mut uitext_query: Query<&mut Text, With<UIInteractText>>,
    input: Res<ButtonInput<KeyCode>>,
    mut console_state: Res<State<console::ConsoleState>>,
    mut next_console_state: ResMut<NextState<console::ConsoleState>>,
) {
    let mut is_interactable = false;
    let mut is_interacting = false;
    
    for (player_transform, camera) in player_query.iter() {
        let ray_direction = player_transform.forward();

        if let Ok(player_collider_entity) = player_collider.get_single() {
            // println!("INSIDE THE ONE");
            if let Some((interactable_entity, toi)) = rapier_context.cast_ray(player_transform.translation(), ray_direction, 100.0, true, QueryFilter::exclude_dynamic()) {
                if let Ok((object, interaction_type)) = interaction_query.get(interactable_entity) {
                    let distance = player_transform.translation().distance(ray_direction * toi);
                    if distance < 3.0 {
                        if let Ok(mut interaction_ui) = uitext_query.get_single_mut() {
                            match console_state.get() {
                                console::ConsoleState::IsNotUsingConsole => {
                                    interaction_ui.sections[0].value = String::from("[F] - use terminal");
                                    if input.pressed(KeyCode::KeyF) || input.just_pressed(KeyCode::KeyF) {
                                        next_console_state.set(console::ConsoleState::IsUsingConsole);
                                    }
                                }
                                _ => { is_interacting = true; }
                            }

                        }
                        is_interactable = true;
                    }
                }
            }
        }
    }

    if !is_interactable {
        if let Ok(mut interaction_ui) = uitext_query.get_single_mut() {
            interaction_ui.sections[0].value = String::from("");
        }
    }
    else if is_interacting {
        if let Ok(mut interaction_ui) = uitext_query.get_single_mut() {
            interaction_ui.sections[0].value = String::from("");
        }
    }

}
