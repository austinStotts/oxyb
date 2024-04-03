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
    IsNotUsingConsole,
    IsUsingConsole,
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
    mut terminal_child_query: Query<(Entity, &mut Handle<Mesh>), With<ConsoleText>>,
    mut terminal_screen_query: Query<Entity, (With<TerminalScreen>, Without<ConsoleText>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let mut text_list: Vec<&str> = vec![];
    for line in terminal.text.iter() {
        text_list.push(&line);
    }

    if text_list.len() > 13 {
        let s = text_list.len() - 13;
        text_list = text_list[s..].to_vec();
    }

    let leftover = 14 - text_list.len();

    for x in 0..leftover {
        text_list.push(".");
    }
    let mut command_line_string = String::from("> ");
    command_line_string.push_str(&current_command.text);
    text_list.insert(13, &command_line_string);

    for (i, (mut entity, mut mesh) ) in terminal_child_query.iter_mut().enumerate() {
        *mesh = meshes.add(get_text_mesh(text_list[i]));
    }
}



pub fn use_console(
    mut terminal: ResMut<Terminal>,
    mut current_command: ResMut<CurrentCommand>,
    mut console_state: Res<State<ConsoleState>>,
    input: Res<ButtonInput<KeyCode>>,
    mut next_console_state: ResMut<NextState<ConsoleState>>,
) {




    match console_state.get() {
        ConsoleState::IsUsingConsole => {

            for key in input.get_just_pressed() {
                println!("{}", current_command.text);
                if input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
                    // shift is held
                    for key in input.get_just_pressed() {
                        match key {
                            KeyCode::Digit0 => { current_command.text.push_str("!") }
                            KeyCode::Digit1 => { current_command.text.push_str("@") }
                            KeyCode::Digit2 => { current_command.text.push_str("#") }
                            KeyCode::Digit3 => { current_command.text.push_str("$") }
                            KeyCode::Digit4 => { current_command.text.push_str("%") }
                            KeyCode::Digit5 => { current_command.text.push_str("^") }
                            KeyCode::Digit6 => { current_command.text.push_str("&") }
                            KeyCode::Digit7 => { current_command.text.push_str("*") }
                            KeyCode::Digit8 => { current_command.text.push_str("(") }
                            KeyCode::Digit9 => { current_command.text.push_str(")") }
                            KeyCode::Slash => { current_command.text.push_str("?") }
                            KeyCode::Comma => { current_command.text.push_str("<") }
                            KeyCode::Period => { current_command.text.push_str(">") }
                            KeyCode::Semicolon => { current_command.text.push_str(":") }
                            KeyCode::Quote => { current_command.text.push_str("\"") }
                            KeyCode::BracketLeft => { current_command.text.push_str("{") }
                            KeyCode::BracketRight => { current_command.text.push_str("}") }
                            KeyCode::Backslash => { current_command.text.push_str("|") }
                            KeyCode::Minus => { current_command.text.push_str("_") }
                            KeyCode::Equal => { current_command.text.push_str("+") }
                            _ => {}
                        }
                    }
                } else {
                    for key in input.get_just_pressed() {

                        match key {
                            KeyCode::KeyA => { current_command.text.push_str("a") }
                            KeyCode::KeyB => { current_command.text.push_str("b") }
                            KeyCode::KeyC => { current_command.text.push_str("c") }
                            KeyCode::KeyD => { current_command.text.push_str("d") }
                            KeyCode::KeyE => { current_command.text.push_str("e") }
                            KeyCode::KeyF => { current_command.text.push_str("f") }
                            KeyCode::KeyG => { current_command.text.push_str("g") }
                            KeyCode::KeyH => { current_command.text.push_str("h") }
                            KeyCode::KeyI => { current_command.text.push_str("i") }
                            KeyCode::KeyJ => { current_command.text.push_str("j") }
                            KeyCode::KeyK => { current_command.text.push_str("k") }
                            KeyCode::KeyL => { current_command.text.push_str("l") }
                            KeyCode::KeyM => { current_command.text.push_str("m") }
                            KeyCode::KeyN => { current_command.text.push_str("n") }
                            KeyCode::KeyO => { current_command.text.push_str("o") }
                            KeyCode::KeyP => { current_command.text.push_str("p") }
                            KeyCode::KeyQ => { current_command.text.push_str("q") }
                            KeyCode::KeyR => { current_command.text.push_str("r") }
                            KeyCode::KeyS => { current_command.text.push_str("s") }
                            KeyCode::KeyT => { current_command.text.push_str("t") }
                            KeyCode::KeyU => { current_command.text.push_str("u") }
                            KeyCode::KeyV => { current_command.text.push_str("v") }
                            KeyCode::KeyW => { current_command.text.push_str("w") }
                            KeyCode::KeyX => { current_command.text.push_str("x") }
                            KeyCode::KeyY => { current_command.text.push_str("y") }
                            KeyCode::KeyZ => { current_command.text.push_str("z") }
                            KeyCode::Digit0 => { current_command.text.push_str("0") }
                            KeyCode::Digit1 => { current_command.text.push_str("1") }
                            KeyCode::Digit2 => { current_command.text.push_str("2") }
                            KeyCode::Digit3 => { current_command.text.push_str("3") }
                            KeyCode::Digit4 => { current_command.text.push_str("4") }
                            KeyCode::Digit5 => { current_command.text.push_str("5") }
                            KeyCode::Digit6 => { current_command.text.push_str("6") }
                            KeyCode::Digit7 => { current_command.text.push_str("7") }
                            KeyCode::Digit8 => { current_command.text.push_str("8") }
                            KeyCode::Digit9 => { current_command.text.push_str("9") }
                            KeyCode::Slash => { current_command.text.push_str("/") }
                            KeyCode::Comma => { current_command.text.push_str(",") }
                            KeyCode::Period => { current_command.text.push_str(".") }
                            KeyCode::Semicolon => { current_command.text.push_str(";") }
                            KeyCode::Quote => { current_command.text.push_str("'") }
                            KeyCode::BracketLeft => { current_command.text.push_str("[") }
                            KeyCode::BracketRight => { current_command.text.push_str("]") }
                            KeyCode::Backslash => { current_command.text.push_str("\\") }
                            KeyCode::Minus => { current_command.text.push_str("-") }
                            KeyCode::Equal => { current_command.text.push_str("=") }
                            KeyCode::Space => { current_command.text.push_str(" ") }
                            KeyCode::Backspace => { current_command.text.pop(); }
                            KeyCode::Enter => { 
            
                                let command = current_command.text.clone();
                                // let c = String::from("$");
                                terminal.text.push("$ ".to_owned() + &current_command.text.clone());
                                current_command.text = String::from("");
        
                                if command.to_lowercase().eq("clear") {
                                    println!("CLEAR COMMAND");
                                    terminal.text = Vec::new();
                                }
                                else if command.to_lowercase().eq("exit") {
                                    next_console_state.set(ConsoleState::IsNotUsingConsole);
                                }
                                if command.to_lowercase().eq("hello") {
                                    println!("HELLO COMMAND");
                                    terminal.text.push(make_hello());
                                }
                                if command.to_lowercase().eq("ls") { // ls
                                    println!("list directory COMMAND");
                                    terminal_list();
                                }
                                if command.to_lowercase().starts_with("cd ") { // cd
                                    println!("move COMMAND");
                                    terminal.text.push(make_hello());
                                }
                            }
                            _ => {}
                        }
                    }
                }
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

    
    let mesh = get_text_mesh("");
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
        mesh: meshes.add(mesh),
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


    for i in 0..13 {
        let mut b = bundle.clone();
        let new_bundle = get_text_pos(&mut b, i as usize);
        let line = commands.spawn(new_bundle).insert(ConsoleText).id();
        commands.entity(screen).add_child(line);
    }

    let command_bundle = get_text_pos(&mut bundle.clone(), 13);
    let command_line = commands.spawn(command_bundle).insert(ConsoleText).id();
    commands.entity(screen).add_child(command_line);


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




use std::collections::HashMap;

// A node representing a directory
struct Directory {
    name: String,
    children: HashMap<String, Directory>, // Map of child directory names to Directory structs
}

impl Directory {
    fn new(name: String) -> Self {
        Directory {
            name,
            children: HashMap::new(),
        }
    }

    fn add_child(&mut self, name: String) -> Result<(), String> {
        let n = name.clone();
        if self.children.contains_key(&name) {
            Err(format!("Directory with name '{}' already exists", &name))
        } else {
            self.children.insert(name, Directory::new(n));
            Ok(())
        }
    }

    fn delete_child(&mut self, name: &str) -> Result<(), String> {
        if let Some(_child) = self.children.remove(name) {
            Ok(())
        } else {
            Err(format!("Directory with name '{}' not found", name))
        }
    }

    fn cd(&mut self, path: &str) -> Result<&Directory, String> {
        let mut current_dir = self;

        for dir_name in path.split('/') {
            match dir_name {
                "." => continue, // Stay in the current directory
                ".." => {
                    return Err("Cannot go beyond the root directory".to_string());
                    // We'll assume you have a root and cannot go beyond it
                }
                name => {
                    current_dir = current_dir.children.get_mut(name)
                        .ok_or_else(|| format!("Directory not found: {}", name))?;
                }
            }
        }

        Ok(current_dir)
    }

    fn ls(&self) {
        if self.children.is_empty() {
            println!("(empty)");
        } else {
            for child_name in self.children.keys() {
                let v = child_name.split("/");
                let name = v.last().unwrap();
                println!("{}", name);
            }
        }
    }
}

fn terminal_list()
{
    let mut root = Directory::new(String::from("root"));
    root.add_child(String::from("root/user"));
    root.add_child(String::from("root/programs"));
    root.add_child(String::from("root/files"));
    root.add_child(String::from("root/system"));

    root.ls();
} 

fn make_hello() -> String {
    return String::from("hello, my name is sumi :3");
}

// very happy with this
// need to add the root to the world resources
// need to format the outputs
// need to make it happen!
