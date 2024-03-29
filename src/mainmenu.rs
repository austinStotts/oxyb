use std::default;
use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_ui::prelude::*;
use crate::camera::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};




#[derive(Component)]
pub struct DespawnOnExit;

#[derive(Component)]
pub struct ButtonState {
    pressed: bool,
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>
) {


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
            create_button(parent, "Play", &asset_server);
            create_button(parent, "Settings", &asset_server);
            create_button(parent, "Connect", &asset_server);
            create_button(parent, "Exit", &asset_server);
        });
    });
}

fn create_button(parent: &mut ChildBuilder, text: &str, asset_server: &Res<AssetServer>) {
    
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
    });
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiImage, &mut BorderColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut image, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
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
    }
}

pub fn despawn_all(entities: Query<Entity, With<DespawnOnExit>>, mut commands: Commands) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}