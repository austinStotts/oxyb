

use std::{default, iter::once};

use bevy::{
    ecs::system::{Command, RunSystemOnce, SystemId},
    math::vec3, 
    prelude::*, transform::TransformSystem,
};
use bevy_flycam::prelude::*;
use map::{Room, Rotation};
use iyes_perf_ui::prelude::*;



mod mainmenu;
mod game;
mod map;
mod postprocessing;



#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    MainMenu,
    Settings,
    Setup,
    Game,
}

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
        }), postprocessing::PostProcessPlugin, NoCameraPlayerPlugin))
        .add_plugins((
            bevy::diagnostic::FrameTimeDiagnosticsPlugin,
            bevy::diagnostic::EntityCountDiagnosticsPlugin,
            bevy::diagnostic::SystemInformationDiagnosticsPlugin,
            PerfUiPlugin,
        ))
        .insert_resource(MovementSettings {
            sensitivity: 0.00002, // default: 0.00012
            speed: 6.0, // default: 12.0
        })
        .insert_resource(KeyBindings {
            move_ascend: KeyCode::Space,
            move_descend: KeyCode::ControlLeft,
            ..Default::default()
        })
        .insert_resource(game::ActiveCamera::Primary)
        .init_state::<GameState>()
        // .add_systems(Startup, setup)
        // MAIN MENU SYSTEMS
        .add_systems(OnEnter(GameState::MainMenu), mainmenu::setup)
        .add_systems(OnExit(GameState::MainMenu), mainmenu::despawn_all)
        .add_systems(Update, (
            keyboard_input,
        ).run_if(in_state(GameState::MainMenu)))
        // GAME SYSTEMS
        .add_systems(OnEnter(GameState::Game), game::game_setup)
        .add_systems(OnExit(GameState::Game), game::despawn_all)
        .add_systems(Update, (
            game::rotate_map,
            game::update_settings,
            keyboard_input,
            game::switch_cameras,
        ).run_if(in_state(GameState::Game)))
        .run();
}

//.run_if(in_state(GameState::Game))



// #[derive(Component)]


fn print_state(state: Res<State<GameState>>) {
    match state.get() {
        GameState::MainMenu => { println!("GAME STATE: MAIN MENU") },
        GameState::Settings => { println!("GAME STATE: SETTINGS") },
        GameState::Setup => { println!("GAME STATE: SETUP") },
        GameState::Game => { println!("GAME STATE: GAME") },
    }
}


//                                                          KEYBOARD INPUTS
fn keyboard_input(
    input: Res<ButtonInput<KeyCode>>,
    mut active_camera: ResMut<game::ActiveCamera>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    map: Query<Entity, With<game::MapParent>>,
    gamestate: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>
) {
    if input.just_pressed(KeyCode::KeyW) {
        info!("'W' currently pressed");
    }
    if input.just_pressed(KeyCode::KeyA) {
        info!("'A' just pressed");
    }
    if input.just_pressed(KeyCode::KeyS) {
        info!("'S' just released");
    }
    if input.just_pressed(KeyCode::KeyD) {
        info!("'D' just released");
    }
    if input.just_pressed(KeyCode::Space) {
        info!("'Space' just released");
    }
    if input.just_pressed(KeyCode::ControlLeft) {
        info!("'Left CTRL' just released");
    }
    if input.just_pressed(KeyCode::ShiftLeft) {
        info!("'Left SHIFT' just released");
    }
    if input.just_pressed(KeyCode::KeyF) {
        info!("'F' just released");
        for entity in map.iter() {
            commands.entity(entity).despawn_recursive();
        }
        game::spawn_new_map(commands, meshes, materials);
    }
    if input.just_pressed(KeyCode::KeyE) {
        info!("'E' just released");
        match gamestate.get() {
            GameState::MainMenu => {
                println!("SETTING GAME STATE TO |GAME|");
                next_state.set(GameState::Game);
            }
            GameState::Game => {
                println!("SETTING GAME STATE TO |MAIN MENU|");
                next_state.set(GameState::MainMenu);
            }
            _ => {}
        }
    }
    if input.just_pressed(KeyCode::KeyQ) {
        info!("'Q' just released");
    }
    if input.just_pressed(KeyCode::Tab) {
        *active_camera = game::ActiveCamera::Secondary;
    }
    if input.just_released(KeyCode::Tab) {
        *active_camera = game::ActiveCamera::Primary;
    }
}




























