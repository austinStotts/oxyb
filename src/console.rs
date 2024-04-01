use std::f32::consts::PI;

use bevy::{core_pipeline::core_3d::graph::input, prelude::*, render::render_asset::RenderAssetUsages, transform::commands};
use bevy_ui::prelude::*;
use bevy::math::vec3;
use meshtext::{MeshGenerator, MeshText, TextSection};


#[derive(Component)]
pub struct ActiveTerminal {
    pub Id: String
}

#[derive(Component)]
pub struct ConsoleTerminal;

#[derive(Resource)]
pub struct Terminal {
    pub text: Vec<String>,
}

#[derive(Resource)]
pub struct CurrentCommand {
    pub text: String
}

#[derive(Component)]
pub struct TerminalScreen;

#[derive(Component)]
pub struct ConsoleText;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum ConsoleState {
    #[default]
    IsUsingConsole,
    IsNotUsingConsole,
}


pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        // app.insert_resource(Terminal{ text: vec![String::from("hello world")] });
        // app.add_systems(Update, update_terminal);
    }
}

fn access_terminal(
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyF) { // and check if in front of console
        // move camera to be looking right at console screen
    }
}

pub fn update_terminal(
    terminal: ResMut<Terminal>,
    current_command: ResMut<CurrentCommand>,
    mut terminal_child_query: Query<Entity, With<ConsoleText>>,
    mut terminal_screen_query: Query<Entity, (With<TerminalScreen>, Without<ConsoleText>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // if let Ok(mut screen) = console_wuery.get_single_mut() {
    //     println!("CAN GE THE CONSOLE");
    // }

    // make the terminal screen match the global resourse each frame.
    let mut text_list: Vec<&str> = vec![];
    for line in terminal.text.iter() {
        text_list.push(&line);
    }

    let mesh = get_text_mesh("hello world");
    let scale = vec3(0.05, 0.05, 0.05);

    let bundle = PbrBundle {
        mesh: meshes.add(mesh.clone()),
        material: materials.add(Color::rgb(0.1, 1.0, 0.1)),
        transform: Transform { 
            translation: vec3(0.0, 1.6, 0.0),
            scale,
            rotation: Quat::from_axis_angle(vec3(0.0, 1.0, 0.0), PI),
            ..Default::default()
        },
        ..Default::default()
    };

    // let text_list2 = vec![
    //     "> hello world",
    //     "> mission -1 -hard",
    //     "...running",
    //     "> list players",
    //     "steve, bob, joe, mac",
    //     "> print time",
    //     "11:34 AM",
    //     "> print depth",
    //     "2086 meters"
    // ];

    let mut bundles = vec![];

    for text in text_list {
        let mut b = bundle.clone();
        b.mesh = meshes.add(get_text_mesh(text));
        bundles.push(b);
    }

    // first push the new rows into the children list
    // then remove the old ones
    // if we do it in reverse and the children list becomes empty we cannot add more children
    if let Ok(mut screen) = terminal_screen_query.get_single_mut() {
        for (i, mut bundle) in bundles.iter_mut().enumerate() {
            let new_bundle = get_text_pos(&mut bundle, i);
            let line = commands.spawn(new_bundle).id();
            commands.entity(screen).push_children(&[line]);
        }

        for (i, mut entity) in terminal_child_query.iter_mut().enumerate() {
            commands.entity(screen).remove_children(&[entity]);
        }
        let mut b1 = bundle.clone();
        b1.mesh = meshes.add(get_text_mesh(&current_command.text));
        let new_bundle = get_text_pos(&mut b1, 14 as usize);
        let line = commands.spawn(new_bundle).id();
        commands.entity(screen).push_children(&[line]);
    } 




}

pub fn use_console(
    terminal: ResMut<Terminal>,
    console_state: Res<State<ConsoleState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    match console_state.get() {
        ConsoleState::IsUsingConsole => {
            for key in input.get_just_pressed() {
                println!("{:?}", key);
                // make a holder for the inputs. use enter to submit
                // somehow display the unfinished command with the other lines.
                // shouldnt be that hard
                // this is getting really complicated but i think as long as it is it's own system it should be fine.
                // maybe have 2 "screen"s 
                // show all the rows that are currently in the list
                // then show the value of the current unsubmitted command
                // or 1 screen but after the for loop to add the history lines it shows the new command line
                // it will be ">" by default

                // when user presses enter take value of currentCommand and add it to terminaltext
                // at the same time, check if valid command and do the command.
                // if not do nothing.
            }
        }
        _ => {}
    }
}


pub fn spawn_console(
    transform: Transform,
    Id: String,
    asset_server: & Res<AssetServer>,
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
) {


        // // Setup your 3D camera
        // commands.spawn(Camera3dBundle {
        //     transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        //     ..default()
        // });
    
        // Setup your UI camera
        // commands.spawn(Camera2dBundle::default());

    let console = commands.spawn(SceneBundle {
        scene: asset_server.load("objects/console.gltf#Scene0"),
        transform,
        // transform: Transform {
        //     scale: vec3(0.25, 0.25, 0.25),
        //     ..default()
        // },
        // material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
        // transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    })
    .insert(ConsoleTerminal)
    .insert(ActiveTerminal{Id}).id();


    // let screen = commands.spawn(TextMeshBundle {
    //     text_mesh: TextMesh::new_with_color(
    //         "Hello Bevy",
    //         asset_server.load("fonts/KodeMono-Regular.ttf"),
    //         Color::rgb(1., 1., 0.)),
    //     transform: Transform::from_xyz(-1., 1.75, 0.),
    //     ..Default::default()
    // }).id();

    // commands.entity(console).add_child(screen);



    // let text_list = vec![
    //     "> hello world",
    //     "> mission -1 -hard",
    //     "...running",
    //     "> list players",
    //     "steve, bob, joe, mac",
    //     "> print time",
    //     "11:34 AM",
    //     "> print depth",
    //     "2086 meters"
    // ];

    
    let mesh = get_text_mesh("hello world");
    let scale = vec3(0.05, 0.05, 0.05);

    // create the screen element to attatch the text children to
    let screen = commands.spawn(PbrBundle {
        transform: Transform {
            translation: vec3(
                -0.5,
                0.15,
                -1.7,
            ),
            rotation: Quat::from_axis_angle(vec3(1.0, 0.0, 0.0), PI / 18.0),
            ..default()
        },
        ..Default::default()
    }).insert(TerminalScreen)
    .id();

    // create the default bundle for the text
    let bundle = PbrBundle {
        mesh: meshes.add(mesh.clone()),
        material: materials.add(Color::rgb(0.1, 1.0, 0.1)),
        transform: Transform {
            translation: vec3(0.0, 1.6, 0.0), 
            scale,
            rotation: Quat::from_axis_angle(vec3(0.0, 1.0, 0.0), PI),
            ..Default::default()
        },
        ..Default::default()
    };

    // let mut bundles = vec![];

    // for text in text_list {
    //     let mut b = bundle.clone();
    //     b.mesh = meshes.add(get_text_mesh(text));
    //     bundles.push(b);
    // }

    // for (i, mut bundle) in bundles.iter_mut().enumerate() {
    //     get_text_pos(&mut bundle, i);
    //     let line = commands.spawn(bundle.clone()).id();
    //     commands.entity(screen).add_child(line);
    // }

    let line = commands.spawn(bundle).insert(ConsoleText).id();
    commands.entity(screen).add_child(line);
    


    // commands.entity(screen).add_child(line1);
    // commands.entity(screen).add_child(line2);
    // commands.entity(screen).add_child(line3);

    


    commands.entity(console).add_child(screen);



}

fn get_text_mesh(
    text: &str
) -> Mesh {
    let font_data = include_bytes!("../assets/fonts/KodeMono-Regular.ttf");
    let mut generator = MeshGenerator::new(font_data);
    let transform = Mat4::from_scale(Vec3::new(1f32, 1f32, 0.2f32)).to_cols_array();
    let text_mesh: MeshText = generator
        .generate_section(&text.to_string(), false, Some(&transform))
        .unwrap();

    let vertices = text_mesh.vertices;
    let positions: Vec<[f32; 3]> = vertices.chunks(3).map(|c| [c[0], c[1], c[2]]).collect();
    let uvs = vec![[0f32, 0f32]; positions.len()];

    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.compute_flat_normals();

    return mesh;
}

fn get_text_pos(bundle: &mut PbrBundle, index: usize) -> PbrBundle {
    let y = 1.6 - (0.05*index as f32);
    // println!("{}", y);
    let mut new_bundle = bundle.clone();
    new_bundle.transform.translation = vec3(0.0, y, 0.0);

    return new_bundle;
}
