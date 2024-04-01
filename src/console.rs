use std::f32::consts::PI;

use bevy::{prelude::*, render::render_asset::RenderAssetUsages, transform::commands};
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
    text: Vec<String>,
}

#[derive(Component)]
pub struct TerminalScreen;



pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app.insert_resource(Terminal{ text: vec![String::from("hello world")] });
    }
}

fn access_terminal(
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyF) { // and check if in front of console
        // move camera to be looking right at console screen
    }
}

fn update_terminal(
    terminal: ResMut<Terminal>,
    terminal_screen_query: Query<Entity, With<TerminalScreen>>,
) {

    // make the terminal screen match the global resourse each frame.

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

    let line = commands.spawn(bundle).id();
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

fn get_text_pos(bundle: &mut PbrBundle, index: usize) {
    let y = 1.6 - (0.05*index as f32);
    println!("{}", y);

    bundle.transform.translation = vec3(0.0, y, 0.0);
}
