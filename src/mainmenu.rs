use std::default;

use bevy::prelude::*;
use bevy_ui::prelude::*;

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
) {

    commands.spawn(Camera3dBundle::default()).insert(DespawnOnExit);

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
            ..default()
        },
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

pub fn button_interaction_system(
    mut interaction_query: Query<(&Interaction, &mut ButtonState, &mut BackgroundColor, &Children),(Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut state, mut background_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                state.pressed = true; 
                println!("Button clicked! (You'll need to add actions for each button)");
            }
            Interaction::Hovered => {
                // Change button appearance on hover (optional)
                *background_color = Color::rgb(0.5, 0.5, 0.5).into();
            }
            _ => {}
        }
    }
}

pub fn despawn_all(entities: Query<Entity, With<DespawnOnExit>>, mut commands: Commands) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}