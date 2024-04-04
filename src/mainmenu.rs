use std::default;
use bevy::{input::keyboard::KeyboardInput, prelude::*, window::ExitCondition};
use bevy_ui::prelude::*;
use crate::camera::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::app::AppExit;

use crate::floor;

// use crate::main::GameState;

#[derive(Component)]
pub struct DespawnOnExit;

#[derive(Component)]
pub struct ButtonState {
    pressed: bool,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Settings,
    Setup,
    Game,
}

#[derive(Component)]
pub enum ButtonType {
    Play,
    Online,
    Settings,
    Exit,
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>
) {

    let graph: floor::DungeonGraph = floor::generate_simple_dungeon();
    // println!("{}", graph.nodes.len());
    graph.print_graph();


    if let Ok(mut window) = primary_window.get_single_mut() {
        toggle_grab_cursor(&mut window);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }


    
    commands.spawn((Camera2dBundle::default(), IsDefaultUiCamera, DespawnOnExit));

    // Root UI node
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::NONE.into(),
        ..default()
    }).insert(DespawnOnExit)
    .with_children(|parent| {
        // Create the buttons horizontally
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center, // Center buttons vertically
                ..default()
            },
            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })   
        .with_children(|parent| {
            create_button(parent, "Play", &asset_server, ButtonType::Play);
            create_button(parent, "Settings", &asset_server, ButtonType::Online);
            create_button(parent, "Connect", &asset_server, ButtonType::Settings);
            create_button(parent, "Exit", &asset_server, ButtonType::Exit);
        });
    });
}

fn create_button(parent: &mut ChildBuilder, text: &str, asset_server: &Res<AssetServer>, button_type: ButtonType) {
    
    parent.spawn(ButtonBundle {
        style: Style {
            width: Val::Px(120.0),
            height: Val::Px(50.0),
            margin: UiRect::all(Val::Px(10.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(3.0)),
            ..default()
        },
        border_color: Color::rgb(0.3, 0.3, 0.3).into(),
        background_color: Color::rgb(0.3, 0.3, 0.3).into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(text, TextStyle {
            font: asset_server.load("fonts/KodeMono-Regular.ttf"),
            font_size: 25.0,
            color: Color::WHITE,
        }));
    }).insert(button_type);
}

pub fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiImage, &mut BorderColor, &Children, &ButtonType),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut events: EventWriter<AppExit>,
    gamestate: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut image, mut border_color, children, button_type) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match button_type {
            ButtonType::Play => {
                match *interaction {
                    Interaction::Pressed => {
                        border_color.0 = Color::RED;
                        match gamestate.get() {
                            GameState::MainMenu => {
                                println!("SETTING GAME STATE TO |GAME|");
                                next_state.set(GameState::Game);
                            }
                            _ => {}
                        }
                    }
                    Interaction::Hovered => {
                        border_color.0 = Color::WHITE;
                    }
                    Interaction::None => {
                        border_color.0 = Color::BLACK;
                    }
                }
            },
            ButtonType::Online => {
                match *interaction {
                    Interaction::Pressed => {
                        border_color.0 = Color::RED;
                    }
                    Interaction::Hovered => {
                        border_color.0 = Color::WHITE;
                    }
                    Interaction::None => {
                        border_color.0 = Color::BLACK;
                    }
                }
            },
            ButtonType::Settings => {
                match *interaction {
                    Interaction::Pressed => {
                        border_color.0 = Color::RED;
                    }
                    Interaction::Hovered => {
                        border_color.0 = Color::WHITE;
                    }
                    Interaction::None => {
                        border_color.0 = Color::BLACK;
                    }
                }
            },
            ButtonType::Exit => {
                match *interaction {
                    Interaction::Pressed => {
                        border_color.0 = Color::RED;
                        events.send(AppExit);
                    }
                    Interaction::Hovered => {
                        border_color.0 = Color::WHITE;
                    }
                    Interaction::None => {
                        border_color.0 = Color::BLACK;
                    }
                }
            },
        }

    }
}

pub fn despawn_all(entities: Query<Entity, With<DespawnOnExit>>, mut commands: Commands) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_play_button() {

}

fn handle_online_button() {
    
}

fn handle_settings_button() {
    
}

fn handle_exit_button(mut events: EventWriter<AppExit>) {
    events.send(AppExit);
}