

use std::{default, iter::once};
use bevy_rapier3d::{prelude::*, rapier::geometry::Ball};
use bevy::{
    ecs::system::{Command, RunSystemOnce, SystemId},
    math::vec3, 
    prelude::*, transform::TransformSystem, winit::WinitSettings,
};
use camera::prelude::game::{check_for_interactions, MainCamera};
// use bevy_flycam::prelude::*;
use map::{Room, Rotation};
use iyes_perf_ui::prelude::*;

mod camera;
mod mainmenu;
mod game;
mod map;
mod postprocessing;
mod models;
mod console;




#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum DevState {
    #[default]
    On,
    Off,
}

#[derive(Component)]
struct KeyboardInput;






fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "oxy beta".into(),
                name: Some("bevy.app".into()),
                // resolution: (1920., 1080.).into(),
                resolution: (852., 480.).into(),
                prevent_default_event_handling: false,
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                ..default()
            }),
            ..default()
        })
        .set(AssetPlugin {
            ..default()
        })))
        // .insert_resource(WinitSettings::desktop_app()) // does not seem to be needed
        .add_plugins((
            postprocessing::PostProcessPlugin,
            // NoCameraPlayerPlugin,
            camera::CameraPluginV2,
            bevy::diagnostic::FrameTimeDiagnosticsPlugin,
            bevy::diagnostic::EntityCountDiagnosticsPlugin,
            bevy::diagnostic::SystemInformationDiagnosticsPlugin,
            PerfUiPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            // console::ConsolePlugin,
        ))
        .insert_resource(console::Terminal { text: vec![String::from("universal instruction terminal v0.2.3")], upper: 13, lower: 1 })
        .insert_resource(console::CurrentCommand { text: String::from("") })
        .insert_resource(game::ActiveCamera::Primary)
        .init_state::<mainmenu::GameState>()
        .init_state::<console::ConsoleState>()
        // .add_systems(Startup, models::load_model)
        // MAIN MENU SYSTEMS
        .add_systems(OnEnter(mainmenu::GameState::MainMenu), mainmenu::setup)
        .add_systems(OnExit(mainmenu::GameState::MainMenu), mainmenu::despawn_all)
        .add_systems(Update, (
            keyboard_input,
            mainmenu::button_interaction_system,
        ).run_if(in_state(mainmenu::GameState::MainMenu)))
        // GAME SYSTEMS
        .add_systems(OnEnter(mainmenu::GameState::Game), (game::game_setup, game::setup_physics))
        .add_systems(OnExit(mainmenu::GameState::Game), (game::despawn_all, map::despawn_all))
        .add_systems(Update, (
            map::rotate_map,
            game::update_settings,
            keyboard_input,
            game::update_player_camera,
            console::use_console,
            check_for_interactions,
            // game::switch_cameras,
        ).run_if(in_state(mainmenu::GameState::Game)))
        .add_systems(PostUpdate, (console::update_terminal).run_if(in_state(mainmenu::GameState::Game)))
        .run();
}

//.run_if(in_state(GameState::Game))



// #[derive(Component)]


fn print_state(state: Res<State<mainmenu::GameState>>) {
    match state.get() {
        mainmenu::GameState::MainMenu => { println!("GAME STATE: MAIN MENU") },
        mainmenu::GameState::Settings => { println!("GAME STATE: SETTINGS") },
        mainmenu::GameState::Setup => { println!("GAME STATE: SETUP") },
        mainmenu::GameState::Game => { println!("GAME STATE: GAME") },
    }
}


//                                                          KEYBOARD INPUTS
fn keyboard_input(
    input: Res<ButtonInput<KeyCode>>,
    // mut commands: Commands,
    // meshes: ResMut<Assets<Mesh>>,
    // materials: ResMut<Assets<StandardMaterial>>,
    // map: Query<Entity, With<map::MapParent>>,
    // gamestate: Res<State<mainmenu::GameState>>,
    // mut next_state: ResMut<NextState<mainmenu::GameState>>,
    mut player_body: Query<&mut Transform, (With<game::PlayerBody>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<game::PlayerBody>)>,
    time: Res<Time>,
    mut console_state: Res<State<console::ConsoleState>>,
    // mut next_console_state: ResMut<NextState<console::ConsoleState>>,
) {

    let speed: f32 = 2.0;
    let mut forward: Vec3 = vec3(0.0, 0.0, 0.0);
    let mut right: Vec3 = vec3(0.0, 0.0, 0.0);
    
    match console_state.get() {
        &console::ConsoleState::IsNotUsingConsole => {
            for mut transform in camera_query.iter_mut() {
                let mut velocity = Vec3::ZERO;
                let local_z = transform.local_z();
                forward = -Vec3::new(local_z.x, 0., local_z.z);
                right = Vec3::new(local_z.z, 0., -local_z.x);
        
                if input.pressed(KeyCode::KeyW) { velocity += forward; }
                if input.pressed(KeyCode::KeyA) { velocity -= right; }
                if input.pressed(KeyCode::KeyS) { velocity -= forward; }
                if input.pressed(KeyCode::KeyD) { velocity += right; }
                velocity = velocity.normalize_or_zero();
        
                if let Ok(mut player_transform) = player_body.get_single_mut() {
                    // player_transform.tra (velocity * time.delta_seconds() * speed).x;
                    player_transform.translation.x += (velocity * time.delta_seconds() * speed).x;
                    player_transform.translation.z += (velocity * time.delta_seconds() * speed).z;
                }
            }
        }
        _ => {}
    }
}




























